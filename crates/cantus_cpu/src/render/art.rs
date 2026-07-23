use crate::{CantusApp, render::pipelines::IMAGE_SIZE};
use arrayvec::ArrayVec;
use image::{DynamicImage, RgbaImage, imageops};
use palette::{Clamp, IntoColor, Lch, color_theory::Analogous};
use std::{array, ops::Range, sync::Arc, time::Instant};

const NUM_SWATCHES: usize = 4;

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
    palette: [u32; NUM_SWATCHES],
}

pub fn prepare(image: &DynamicImage) -> AlbumArt {
    let image = image
        .resize_to_fill(IMAGE_SIZE, IMAGE_SIZE, imageops::FilterType::Lanczos3)
        .to_rgba8();
    AlbumArt {
        palette: image_palette(&image),
        pixels: image.into_raw().into_boxed_slice(),
    }
}

impl ArtState {
    pub const fn ready(&self) -> Option<&Arc<AlbumArt>> {
        match self {
            Self::Ready(art) => Some(art),
            _ => None,
        }
    }

    pub fn palette(&self) -> [u32; NUM_SWATCHES] {
        self.ready().map_or([0; NUM_SWATCHES], |art| art.palette)
    }

    fn needs_fetch(&self, now: Instant) -> bool {
        matches!(self, Self::Missing) || matches!(self, Self::RetryAt(at) if *at <= now)
    }
}

impl CantusApp {
    /// Every art slot in the queue and playlists, with the URL it wants.
    fn art_slots(&mut self) -> impl Iterator<Item = (&str, &mut ArtState)> {
        let tracks = self.playback.queue.iter_mut();
        let playlists = self.playback.playlists.iter_mut();
        tracks
            .filter_map(|track| Some((track.album.image.as_deref()?, &mut track.runtime.art)))
            .chain(
                playlists.filter_map(|playlist| {
                    Some((playlist.image_url.as_deref()?, &mut playlist.art))
                }),
            )
    }

    pub fn start_missing_art_downloads(&mut self) {
        let now = Instant::now();
        loop {
            let url = self
                .art_slots()
                .find_map(|(url, state)| state.needs_fetch(now).then(|| url.to_owned()));
            let Some(url) = url else { break };
            // Share art another slot already holds for the same URL, else fetch it.
            let existing = self
                .art_slots()
                .find_map(|(other, state)| (other == url).then(|| state.ready().cloned())?);
            let state = if let Some(art) = existing {
                ArtState::Ready(art)
            } else {
                self.spotify.download_image(url.clone());
                ArtState::Fetching
            };
            self.set_art_state(&url, &state);
        }
    }

    pub fn set_art_state(&mut self, url: &str, state: &ArtState) {
        self.art_slots()
            .filter(|(slot_url, _)| *slot_url == url)
            .for_each(|(_, slot)| *slot = state.clone());
    }
}

fn complete_palette(colors: &mut ArrayVec<(Lch, f32), NUM_SWATCHES>) {
    colors.sort_by(|a, b| b.1.total_cmp(&a.1));
    let mut index = 1;
    while index < colors.len() {
        let (color, weight) = colors[index];
        if let Some(duplicate) = colors[..index]
            .iter()
            .position(|(other, _)| (color.hue - other.hue).into_degrees().abs() < 20.0)
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

const fn component(color: &palette::Lab, channel: usize) -> f32 {
    [color.l, color.a, color.b][channel]
}

/// A small median-cut quantizer tailored to the four swatches the renderer needs.
fn dominant_colors(pixels: &mut [palette::Lab]) -> ArrayVec<(Lch, f32), NUM_SWATCHES> {
    let mut buckets = ArrayVec::<Range<usize>, NUM_SWATCHES>::new();
    buckets.push(0..pixels.len());

    while buckets.len() < NUM_SWATCHES {
        let Some((bucket_index, channel)) = buckets
            .iter()
            .enumerate()
            .filter(|(_, range)| range.len() > 1)
            .map(|(index, range)| {
                let mut min = [f32::INFINITY; 3];
                let mut max = [f32::NEG_INFINITY; 3];
                for color in &pixels[range.clone()] {
                    for channel in 0..3 {
                        min[channel] = min[channel].min(component(color, channel));
                        max[channel] = max[channel].max(component(color, channel));
                    }
                }
                let (channel, spread) = (0..3)
                    .map(|channel| (channel, max[channel] - min[channel]))
                    .max_by(|a, b| a.1.total_cmp(&b.1))
                    .unwrap();
                (index, channel, spread * range.len() as f32)
            })
            .max_by(|a, b| a.2.total_cmp(&b.2))
            .map(|(index, channel, _)| (index, channel))
        else {
            break;
        };

        let range = buckets.swap_remove(bucket_index);
        pixels[range.clone()]
            .sort_unstable_by(|a, b| component(a, channel).total_cmp(&component(b, channel)));
        let middle = range.start + range.len() / 2;
        buckets.push(range.start..middle);
        buckets.push(middle..range.end);
    }

    buckets
        .into_iter()
        .map(|range| {
            let weight = range.len() as f32;
            let sum = pixels[range].iter().fold([0.0; 3], |mut sum, color| {
                sum[0] += color.l;
                sum[1] += color.a;
                sum[2] += color.b;
                sum
            });
            (
                palette::Lab::new(sum[0] / weight, sum[1] / weight, sum[2] / weight).into_color(),
                weight,
            )
        })
        .collect()
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
            pixel[3] >= 128 && max - min > 30
        })
        .map(srgb_to_lab)
        .collect();
    let use_harmony = !pixels.is_empty();
    if !use_harmony {
        pixels.extend(
            image
                .pixels()
                .filter(|pixel| pixel[3] >= 128)
                .map(srgb_to_lab),
        );
    }

    if pixels.is_empty() {
        return [u32::from_le_bytes([0, 0, 0, 255]); NUM_SWATCHES];
    }
    let mut colors = dominant_colors(&mut pixels);
    if use_harmony {
        complete_palette(&mut colors);
    }
    let total = colors.iter().map(|(_, weight)| weight).sum();
    array::from_fn(|index| pack_color(colors[index % colors.len()], total))
}
