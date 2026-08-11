use crate::{
    app::{CantusApp, spotify::PlaybackState},
    render::{cpu::Passes, track::PALETTE_COLORS},
};
use arrayvec::ArrayVec;
use image::{DynamicImage, RgbaImage, imageops};
use isthmus::{
    FilterableFloatFormat, SampledTexture, Texture2DArray, TextureView, Unorm8x4, glam::Vec3,
    wgpu::Extent3d,
};
use palette::{Clamp, IntoColor, Lch, color_theory::Analogous};
use std::{
    array,
    collections::HashMap,
    ops::Range,
    sync::{Arc, Weak},
    time::Instant,
};

const MAX_TEXTURE_IMAGES: u32 = 32;
pub const IMAGE_SIZE: u32 = 64;

/// Album art for one slot, held beside the track or playlist that wants it.
#[derive(Clone, Default)]
pub enum ArtState {
    #[default]
    Missing,
    Fetching,
    RetryAt(Instant),
    Ready(Arc<AlbumArt>),
}

impl ArtState {
    pub const fn ready(&self) -> Option<&Arc<AlbumArt>> {
        match self {
            Self::Ready(art) => Some(art),
            _ => None,
        }
    }

    pub fn palette(&self) -> [Unorm8x4; PALETTE_COLORS] {
        self.ready()
            .map_or_else(|| [Unorm8x4::default(); PALETTE_COLORS], |art| art.palette)
    }

    fn wanted(&self, now: Instant) -> bool {
        matches!(self, Self::Missing) || matches!(self, Self::RetryAt(at) if *at <= now)
    }
}

pub struct AlbumArt {
    /// RGBA image pixels.
    pub pixels: Box<[u8]>,
    /// RGB swatches with their relative influence.
    palette: [Unorm8x4; PALETTE_COLORS],
}

/// The album-art texture array the track shader samples from.
pub struct ImageAtlas {
    texture: SampledTexture<Texture2DArray>,
    slots: [Weak<AlbumArt>; MAX_TEXTURE_IMAGES as usize],
    used: u32,
}

impl ImageAtlas {
    pub fn new(passes: &Passes<'_>) -> Self {
        Self {
            texture: passes.sampled_texture::<Texture2DArray>(
                "Images",
                Extent3d {
                    width: IMAGE_SIZE,
                    height: IMAGE_SIZE,
                    depth_or_array_layers: MAX_TEXTURE_IMAGES,
                },
                FilterableFloatFormat::Rgba8Unorm,
            ),
            slots: [const { Weak::new() }; MAX_TEXTURE_IMAGES as usize],
            used: 0,
        }
    }

    pub const fn view(&self) -> &TextureView<Texture2DArray> {
        self.texture.view()
    }

    /// Clears the per-frame usage mask; call once before re-registering this frame's images.
    pub const fn begin_frame(&mut self) {
        self.used = 0;
    }

    /// Slot `art` is (or becomes) resident in, or -1 if it isn't ready or the atlas is full.
    pub fn index_of(&mut self, art: Option<&Arc<AlbumArt>>) -> i32 {
        let Some(art) = art else {
            return -1;
        };
        if let Some(index) = self
            .slots
            .iter()
            .position(|slot| slot.as_ptr() == Arc::as_ptr(art))
        {
            self.used |= 1 << index;
            return index as i32;
        }
        let index = (!self.used).trailing_zeros();
        if index >= MAX_TEXTURE_IMAGES
            || self
                .texture
                .write([0, 0, index], [IMAGE_SIZE; 2], &art.pixels)
                .is_err()
        {
            return -1;
        }
        self.used |= 1 << index;
        self.slots[index as usize] = Arc::downgrade(art);
        index as i32
    }
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

/// Every art slot in the queue and playlists, with the URL it wants.
fn art_slots(playback: &mut PlaybackState) -> impl Iterator<Item = (&str, &mut ArtState)> {
    playback
        .queue
        .iter_mut()
        .filter_map(|track| Some((track.album.image.as_deref()?, &mut track.runtime.art)))
        .chain(
            playback
                .playlists
                .iter_mut()
                .filter_map(|playlist| Some((playlist.image_url.as_deref()?, &mut playlist.art))),
        )
}

impl CantusApp {
    /// Starts downloads for art the queue and playlists are missing.
    pub fn refresh_art(&mut self) {
        let now = Instant::now();
        let shared = art_slots(&mut self.playback)
            .filter_map(|(url, state)| Some((url.to_owned(), Arc::clone(state.ready()?))))
            .collect::<HashMap<_, _>>();
        let mut download = Vec::new();
        for (url, state) in art_slots(&mut self.playback) {
            if !state.wanted(now) {
                continue;
            }
            *state = shared.get(url).map_or_else(
                || {
                    download.push(url.to_owned());
                    ArtState::Fetching
                },
                |art| ArtState::Ready(Arc::clone(art)),
            );
        }
        download.sort_unstable();
        download.dedup();
        for url in download {
            self.spotify.download_image(url);
        }
    }

    /// Applies a finished download to every slot that wanted that URL.
    pub fn set_art_state(&mut self, url: &str, state: &ArtState) {
        for (slot_url, slot) in art_slots(&mut self.playback) {
            if slot_url == url {
                *slot = state.clone();
            }
        }
    }
}

fn complete_palette(colors: &mut ArrayVec<(Lch, f32), PALETTE_COLORS>) {
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
    for index in 0..PALETTE_COLORS - measured {
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

/// Packs a swatch and its share of the artwork into one word; alpha is the share.
fn palette_color((color, weight): (Lch, f32), total: f32) -> Unorm8x4 {
    let rgb: palette::Srgb = color.into_color();
    let rgb = rgb.clamp();
    Unorm8x4::from_vec4(
        Vec3::new(rgb.red, rgb.green, rgb.blue).extend((weight / total).max(1.0 / 255.0)),
    )
}

const fn component(color: &palette::Lab, channel: usize) -> f32 {
    [color.l, color.a, color.b][channel]
}

/// A small median-cut quantizer tailored to the four swatches the renderer needs.
fn dominant_colors(pixels: &mut [palette::Lab]) -> ArrayVec<(Lch, f32), PALETTE_COLORS> {
    let mut buckets = ArrayVec::<Range<usize>, PALETTE_COLORS>::new();
    buckets.push(0..pixels.len());

    while buckets.len() < PALETTE_COLORS {
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

fn image_palette(image: &RgbaImage) -> [Unorm8x4; PALETTE_COLORS] {
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
        pixels.extend(image.pixels().filter(|pixel| pixel[3] >= 128).map(srgb_to_lab));
    }

    if pixels.is_empty() {
        return [Unorm8x4::default(); PALETTE_COLORS];
    }
    let mut colors = dominant_colors(&mut pixels);
    if use_harmony {
        complete_palette(&mut colors);
    }
    let total = colors.iter().map(|(_, weight)| weight).sum();
    array::from_fn(|index| palette_color(colors[index % colors.len()], total))
}
