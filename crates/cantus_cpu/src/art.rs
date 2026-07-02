use crate::{CantusApp, NUM_SWATCHES, pipelines::IMAGE_SIZE, spotify};
use image::{DynamicImage, RgbaImage, imageops};
use palette::IntoColor;
use std::{array, sync::Arc, time::Instant};

const LAYER_BYTES: usize = (IMAGE_SIZE * IMAGE_SIZE * 4) as usize;

#[derive(Clone, Default)]
pub enum ArtState {
    #[default]
    Missing,
    Fetching,
    RetryAt(Instant),
    Ready(Arc<AlbumArt>),
}

pub struct AlbumArt {
    /// Original RGBA layer followed by its blurred RGBA layer.
    pub pixels: Box<[u8]>,
    pub palette: [u32; NUM_SWATCHES],
}

pub fn prepare(image: &DynamicImage) -> AlbumArt {
    let image = image.resize_to_fill(IMAGE_SIZE, IMAGE_SIZE, imageops::FilterType::Lanczos3);
    let image = image.to_rgba8();
    let palette = image_palette(&image);
    let blurred = imageops::blur(&image, 3.0);
    let mut pixels = Vec::with_capacity(LAYER_BYTES * 2);
    pixels.extend_from_slice(&image);
    pixels.extend_from_slice(&blurred);
    AlbumArt {
        pixels: pixels.into_boxed_slice(),
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
    if matches!(state, ArtState::Missing) || matches!(state, ArtState::RetryAt(at) if *at <= now) {
        url.map(str::to_owned)
    } else {
        None
    }
}

fn image_palette(image: &RgbaImage) -> [u32; NUM_SWATCHES] {
    let srgb_to_lab = |pixel: &image::Rgba<u8>| {
        palette::FromColor::from_color(palette::Srgb::new(
            f32::from(pixel[0]) / 255.0,
            f32::from(pixel[1]) / 255.0,
            f32::from(pixel[2]) / 255.0,
        ))
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
    if pixels.is_empty() {
        pixels.extend(image.pixels().map(srgb_to_lab));
    }

    let centroids =
        kmeans_colors::get_kmeans_hamerly(NUM_SWATCHES, 20, 5.0, false, &pixels, 0).centroids;
    if centroids.is_empty() {
        return [u32::from_le_bytes([0, 0, 0, 255]); NUM_SWATCHES];
    }
    array::from_fn(|index| {
        let rgb: palette::Srgb = centroids[index % centroids.len()].into_color();
        u32::from_le_bytes([
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
            255,
        ])
    })
}
