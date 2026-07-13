use crate::{CantusApp, NUM_SWATCHES, pipelines::IMAGE_SIZE, spotify};
use arrayvec::ArrayVec;
use image::{DynamicImage, RgbaImage, imageops};
use kmeans_colors::Sort;
use palette::{Clamp, IntoColor, Lch, color_theory::Analogous};
use std::{array, sync::Arc, time::Instant};

#[derive(Clone, Default)]
pub enum ArtState {
    #[default]
    Missing,
    Fetching,
    RetryAt(Instant),
    Ready(Arc<AlbumArt>),
}

pub struct AlbumArt {
    /// RGBA image pixels.
    pub pixels: Box<[u8]>,
    /// RGB swatches with their relative influence packed into alpha.
    pub palette: [u32; NUM_SWATCHES],
}

pub fn prepare(image: &DynamicImage) -> AlbumArt {
    let image = image.resize_to_fill(IMAGE_SIZE, IMAGE_SIZE, imageops::FilterType::Lanczos3);
    let image = image.to_rgba8();
    let palette = image_palette(&image);
    AlbumArt {
        pixels: image.into_raw().into_boxed_slice(),
        palette,
    }
}

impl CantusApp {
    pub fn start_missing_art_downloads(&mut self) {
        let now = Instant::now();
        while let Some(url) = self
            .playback_state
            .queue
            .iter()
            .find_map(|track| art_request(&track.art, track.album.image.as_deref(), now))
            .or_else(|| {
                self.playback_state.playlists.iter().find_map(|playlist| {
                    art_request(&playlist.art, playlist.image_url.as_deref(), now)
                })
            })
        {
            self.set_art_state(&url, &ArtState::Fetching);
            spotify::download_image(Arc::clone(&self.spotify.client), self.updater.clone(), url);
        }
    }

    pub fn set_art_state(&mut self, url: &str, state: &ArtState) {
        for track in &mut self.playback_state.queue {
            if track.album.image.as_deref() == Some(url) {
                track.art = state.clone();
            }
        }
        for playlist in &mut self.playback_state.playlists {
            if playlist.image_url.as_deref() == Some(url) {
                playlist.art = state.clone();
            }
        }
    }
}

fn art_request(state: &ArtState, url: Option<&str>, now: Instant) -> Option<String> {
    (matches!(state, ArtState::Missing) || matches!(state, ArtState::RetryAt(at) if *at <= now))
        .then_some(url)
        .flatten()
        .map(str::to_owned)
}

fn similar_hue(a: Lch, b: Lch) -> bool {
    (a.hue - b.hue).into_degrees().abs() < 20.0
}

fn complete_palette(colors: &mut ArrayVec<(Lch, f32), NUM_SWATCHES>) {
    colors.sort_by(|a, b| b.1.total_cmp(&a.1));
    let mut index = 1;
    while index < colors.len() {
        let (color, weight) = colors[index];
        if let Some(duplicate) = colors[..index]
            .iter()
            .position(|(other, _)| similar_hue(color, *other))
        {
            colors[duplicate].1 += weight;
            colors.remove(index);
        } else {
            index += 1;
        }
    }

    let measured = colors.len();
    for index in 0..NUM_SWATCHES - measured {
        let (source, weight) = colors[index % measured];
        let (lower, upper) = source.analogous();
        let mut generated = match index {
            2 if measured == 1 => source.analogous_secondary().0,
            index if index % 2 == 0 => lower,
            _ => upper,
        };
        generated.chroma = generated.chroma.max(35.0);
        colors.push((generated, weight * 0.5));
    }
    colors.sort_by(|a, b| a.0.l.total_cmp(&b.0.l));
}

fn pack_color((color, weight): (Lch, f32), total: f32) -> u32 {
    let rgb: palette::Srgb = color.into_color();
    let rgb = rgb.clamp();
    u32::from_le_bytes([
        (rgb.red * 255.0) as u8,
        (rgb.green * 255.0) as u8,
        (rgb.blue * 255.0) as u8,
        (weight / total * 255.0).round().max(1.0) as u8,
    ])
}

fn image_palette(image: &RgbaImage) -> [u32; NUM_SWATCHES] {
    let srgb_to_lab = |pixel: &image::Rgba<u8>| {
        palette::Srgb::new(
            f32::from(pixel[0]) / 255.0,
            f32::from(pixel[1]) / 255.0,
            f32::from(pixel[2]) / 255.0,
        )
        .into_color()
    };
    let mut pixels: Vec<palette::Lab> = image
        .pixels()
        .filter(|pixel| {
            let max = pixel[0].max(pixel[1]).max(pixel[2]);
            let min = pixel[0].min(pixel[1]).min(pixel[2]);
            max - min > 30
        })
        .map(srgb_to_lab)
        .collect();
    let use_harmony = !pixels.is_empty();
    if !use_harmony {
        pixels.extend(image.pixels().map(srgb_to_lab));
    }

    let result = kmeans_colors::get_kmeans_hamerly(NUM_SWATCHES, 20, 5.0, false, &pixels, 0);
    let swatches = palette::Lab::sort_indexed_colors(&result.centroids, &result.indices);
    if swatches.is_empty() {
        return [u32::from_le_bytes([0, 0, 0, 255]); NUM_SWATCHES];
    }
    let mut colors: ArrayVec<_, NUM_SWATCHES> = swatches
        .iter()
        .map(|swatch| (swatch.centroid.into_color(), swatch.percentage))
        .collect();
    if use_harmony {
        complete_palette(&mut colors);
    }
    let total = colors.iter().map(|(_, weight)| weight).sum();
    array::from_fn(|index| pack_color(colors[index % colors.len()], total))
}
