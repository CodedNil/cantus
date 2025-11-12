use crate::config::CONFIG;
use crate::spotify::{
    ARTIST_DATA_CACHE, IMAGES_CACHE, PLAYBACK_STATE, TRACK_DATA_CACHE, TrackData,
};
use anyhow::Result;
use auto_palette::Palette;
use image::{GrayImage, LumaA, RgbaImage, imageops};
use itertools::Itertools;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::LazyLock,
};
use vello::peniko::{Blob, ImageAlphaType, ImageData, ImageFormat};

/// Number of swatches to use in colour palette generation.
const NUM_SWATCHES: usize = 4;

/// Dimensions of the generated palette-based textures.
fn palette_image_height() -> u32 {
    CONFIG.height as u32
}

fn palette_image_width() -> u32 {
    palette_image_height() * 3
}

/// Number of refinement passes when synthesising the background texture.
const PALETTE_PASS_COUNT: usize = 6;
/// Maximum number of brush placements per pass.
const PALETTE_STROKES_PER_PASS: usize = 30;

static BRUSHES: LazyLock<[GrayImage; 5]> = LazyLock::new(|| {
    // Helper function to load and extract the alpha channel
    let load_and_extract_alpha = |bytes: &[u8]| -> GrayImage {
        let luma_alpha_image: image::ImageBuffer<LumaA<u8>, Vec<u8>> =
            image::load_from_memory(bytes).unwrap().to_luma_alpha8();
        GrayImage::from_raw(
            luma_alpha_image.width(),
            luma_alpha_image.height(),
            luma_alpha_image
                .pixels()
                .map(|p| p.0[1]) // p.0 is the array [Luma, Alpha]. We take index 1 (Alpha).
                .collect(),
        )
        .expect("Failed to create GrayImage from extracted alpha data")
    };

    [
        load_and_extract_alpha(include_bytes!("../assets/brushes/brush1.png")),
        load_and_extract_alpha(include_bytes!("../assets/brushes/brush2.png")),
        load_and_extract_alpha(include_bytes!("../assets/brushes/brush3.png")),
        load_and_extract_alpha(include_bytes!("../assets/brushes/brush4.png")),
        load_and_extract_alpha(include_bytes!("../assets/brushes/brush5.png")),
    ]
});

/// Downloads and caches an image from the given URL.
pub fn update_color_palettes() -> Result<()> {
    let state = PLAYBACK_STATE.read();
    let mut pending_palettes = Vec::new();
    for track in &state.queue {
        if !TRACK_DATA_CACHE.contains_key(&track.id)
            && let Some(image) = IMAGES_CACHE.get(&track.image_url)
            && let Some(artist_image_ref) = ARTIST_DATA_CACHE.get(&track.artist_id)
        {
            // Merge the images side by side
            let width = image.width;
            let height = image.height;
            let album_image =
                RgbaImage::from_raw(width, height, image.data.data().to_vec()).unwrap();

            // Get palette, try on the album image or if that doesn't get enough colours include the artist image
            let swatches = {
                let palette: Palette<f64> = Palette::builder()
                    .algorithm(auto_palette::Algorithm::SNIC)
                    .filter(ChromaFilter { threshold: 20 })
                    .build(&auto_palette::ImageData::new(width, height, &album_image)?)?;
                let swatches = palette
                    .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                    .or_else(|_| palette.find_swatches(NUM_SWATCHES))?;
                if swatches.len() < NUM_SWATCHES {
                    // Generate a new image with the artist image
                    let new_img = if let Some(artist_image_ref) = &*artist_image_ref {
                        let Some(artist_image) = IMAGES_CACHE.get(artist_image_ref) else {
                            // Wait for the image to be cached.
                            continue;
                        };
                        let artist_new_width = (width as f32 * 0.1).round() as u32;
                        let mut new_img = RgbaImage::new(width + artist_new_width, height);
                        image::imageops::overlay(&mut new_img, &album_image, 0, 0);
                        let artist_img_resized = image::imageops::resize(
                            &image::RgbaImage::from_raw(
                                artist_image.width,
                                artist_image.height,
                                artist_image.data.data().to_vec(),
                            )
                            .unwrap(),
                            artist_new_width,
                            height,
                            image::imageops::FilterType::Nearest,
                        );
                        image::imageops::overlay(
                            &mut new_img,
                            &artist_img_resized,
                            i64::from(width),
                            0,
                        );
                        new_img
                    } else {
                        let mut new_img = RgbaImage::new(width, height);
                        image::imageops::overlay(&mut new_img, &album_image, 0, 0);
                        new_img
                    };

                    let palette: Palette<f64> = Palette::builder()
                        .algorithm(auto_palette::Algorithm::SLIC)
                        .filter(ChromaFilter { threshold: 20 })
                        .build(&auto_palette::ImageData::new(
                            new_img.width(),
                            new_img.height(),
                            &new_img,
                        )?)?;
                    palette
                        .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                        .or_else(|_| palette.find_swatches(NUM_SWATCHES))?
                } else {
                    swatches
                }
            };

            // Sort out the ratios
            let total_ratio_sum: f64 = swatches.iter().map(auto_palette::Swatch::ratio).sum();
            let primary_colors = swatches
                .iter()
                .map(|s| {
                    let rgb = s.color().to_rgb();
                    // Sometimes ratios can be tiny like 0.05%, this brings them a little closer to even
                    let lerped_ratio = lerp(
                        0.5,
                        (s.ratio() / total_ratio_sum) as f32,
                        1.0 / swatches.len() as f32,
                    );
                    [rgb.r, rgb.g, rgb.b, (lerped_ratio * 255.0).round() as u8]
                })
                .sorted_by(|a, b| b[3].cmp(&a[3]))
                .collect::<Vec<_>>();

            let palette_seed = {
                let mut hasher = DefaultHasher::new();
                track.id.hash(&mut hasher);
                hasher.finish()
            };
            pending_palettes.push((track.id.clone(), primary_colors, palette_seed));
        }
    }
    drop(state);

    for (track_id, primary_colors, palette_seed) in pending_palettes {
        let palette_image = ImageData {
            data: Blob::from(generate_palette_image(&primary_colors, palette_seed)),
            format: ImageFormat::Rgba8,
            alpha_type: ImageAlphaType::Alpha,
            width: palette_image_width(),
            height: palette_image_height(),
        };
        TRACK_DATA_CACHE.insert(
            track_id,
            TrackData {
                primary_colors,
                palette_image,
            },
        );
    }

    Ok(())
}

/// A filter that filters chroma values.
#[derive(Debug)]
pub struct ChromaFilter {
    threshold: u8,
}
impl auto_palette::Filter for ChromaFilter {
    fn test(&self, pixel: &auto_palette::Rgba) -> bool {
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        (max - min) > self.threshold
    }
}

fn generate_palette_image(colors: &[[u8; 4]], seed: u64) -> Vec<u8> {
    let palette_width = palette_image_width();
    let palette_height = palette_image_height();

    if colors.is_empty() {
        return RgbaImage::new(palette_width, palette_height).into_raw();
    }

    let mut rng = SmallRng::seed_from_u64(seed);

    let mut targets = colors
        .iter()
        .map(|c| f32::from(c[3]).max(1.0))
        .collect::<Vec<_>>();
    let total_target = targets.iter().copied().sum::<f32>().max(1.0);
    for weight in &mut targets {
        *weight /= total_target;
    }

    let color_vectors = colors
        .iter()
        .map(|[r, g, b, _]| [f32::from(*r), f32::from(*g), f32::from(*b)])
        .collect::<Vec<_>>();

    let mut canvas = RgbaImage::from_pixel(
        palette_width,
        palette_height,
        image::Rgba([colors[0][0], colors[0][1], colors[0][2], 255]),
    );

    // Fill with the first colour; refinement passes will rebalance ratios.
    let total_pixels = (palette_width * palette_height) as f32;
    let mut coverage = vec![0.0; colors.len()];
    let mut per_color_strokes = vec![0; colors.len()];
    let mut available_indices = Vec::with_capacity(colors.len());
    for pass in 0..PALETTE_PASS_COUNT {
        let base_height = lerp(
            pass as f32 / PALETTE_PASS_COUNT as f32,
            palette_height as f32 * 0.5,
            palette_height as f32 * 0.2,
        );

        // Count pixels for each color, to get ratios
        let mut counts = vec![0u32; colors.len()];
        for pixel in canvas.pixels() {
            let pr = f32::from(pixel[0]);
            let pg = f32::from(pixel[1]);
            let pb = f32::from(pixel[2]);
            let mut best_index = 0usize;
            let mut best_distance = f32::MAX;
            for (index, color) in color_vectors.iter().enumerate() {
                let dr = pr - color[0];
                let dg = pg - color[1];
                let db = pb - color[2];
                let distance = dr * dr + dg * dg + db * db;
                if distance < best_distance {
                    best_distance = distance;
                    best_index = index;
                }
            }
            counts[best_index] += 1;
        }
        for (index, ratio) in coverage.iter_mut().enumerate() {
            *ratio = counts[index] as f32 / total_pixels;
        }

        // Get how far we are off in total
        let total_coverage_diff = coverage
            .iter()
            .zip(targets.iter())
            .map(|(&c, &t)| (c - t).abs())
            .sum::<f32>()
            .abs();
        if total_coverage_diff <= f32::EPSILON {
            per_color_strokes.fill(0);
        } else {
            for (index, strokes) in per_color_strokes.iter_mut().enumerate() {
                *strokes = (((coverage[index] - targets[index]).abs() / total_coverage_diff)
                    * PALETTE_STROKES_PER_PASS as f32)
                    .floor() as u8;
            }
        }
        available_indices.clear();
        for (index, &count) in per_color_strokes.iter().enumerate() {
            if count > 0 {
                available_indices.push(index);
            }
        }

        // Add strokes to the canvas to balance coverage
        for _ in 0..PALETTE_STROKES_PER_PASS {
            if available_indices.is_empty() {
                break;
            }

            // Randomly select an index from the available candidates
            let index_to_pick = rng.random_range(0..available_indices.len());
            let color_index = available_indices[index_to_pick];
            let strokes_left = &mut per_color_strokes[color_index];
            *strokes_left = strokes_left.saturating_sub(1);
            if *strokes_left == 0 {
                available_indices.swap_remove(index_to_pick);
            }
            let color = colors[color_index];

            // Pick a random brush
            let brush_factor = rng.random_range(0.75..1.2);
            let brush_size = (base_height * brush_factor)
                .round()
                .clamp(6.0, palette_height as f32) as u32;
            let stamp = image::imageops::resize(
                &BRUSHES[rng.random_range(0..BRUSHES.len())],
                brush_size,
                brush_size,
                image::imageops::FilterType::Nearest,
            );

            // Overlay the stamp onto the canvas
            let fade_factor = rng.random_range(0.55..0.9);
            let x = i64::from(rng.random_range(0..=palette_width)) - i64::from(brush_size / 2);
            let y = i64::from(rng.random_range(0..=palette_height)) - i64::from(brush_size / 2);
            let (bottom_width, bottom_height) = canvas.dimensions();
            let (top_width, top_height) = stamp.dimensions();

            // Crop our top image if we're going out of bounds
            let origin_bottom_x = x.clamp(0, i64::from(bottom_width)) as u32;
            let origin_bottom_y = y.clamp(0, i64::from(bottom_height)) as u32;

            let range_width = (x
                .saturating_add(i64::from(top_width))
                .clamp(0, i64::from(bottom_width)) as u32)
                .saturating_sub(origin_bottom_x);
            let range_height = (y
                .saturating_add(i64::from(top_height))
                .clamp(0, i64::from(bottom_height)) as u32)
                .saturating_sub(origin_bottom_y);
            let origin_top_x = x.saturating_neg().clamp(0, i64::from(top_width)) as u32;
            let origin_top_y = y.saturating_neg().clamp(0, i64::from(top_height)) as u32;

            let raw_bottom: &mut [u8] = canvas.as_mut();
            let bottom_stride = bottom_width as usize * 4;
            let top_stride = top_width as usize;
            for y_offset in 0..range_height {
                let bottom_row_start = ((origin_bottom_y + y_offset) as usize) * bottom_stride;
                let top_row_start = ((origin_top_y + y_offset) as usize) * top_stride;
                for x_offset in 0..range_width {
                    let alpha = stamp.as_raw()[top_row_start + (origin_top_x + x_offset) as usize];
                    let adjusted_alpha =
                        (f32::from(alpha) * fade_factor).round().clamp(0.0, 255.0) as u8;
                    if adjusted_alpha == 0 {
                        continue;
                    }
                    let bottom_idx = bottom_row_start + ((origin_bottom_x + x_offset) as usize) * 4;
                    let src_a = u32::from(adjusted_alpha);
                    let inv_a = 255 - src_a;
                    let dst_a = u32::from(raw_bottom[bottom_idx + 3]);
                    let out_a = src_a + (dst_a * inv_a / 255);
                    let blend = |src: u8, dst: u8| {
                        (((u32::from(src) * src_a) + (u32::from(dst) * dst_a * inv_a / 255))
                            / out_a) as u8
                    };
                    raw_bottom[bottom_idx] = blend(color[0], raw_bottom[bottom_idx]);
                    raw_bottom[bottom_idx + 1] = blend(color[1], raw_bottom[bottom_idx + 1]);
                    raw_bottom[bottom_idx + 2] = blend(color[2], raw_bottom[bottom_idx + 2]);
                    raw_bottom[bottom_idx + 3] = out_a as u8;
                }
            }
        }
    }

    // Blur the image
    imageops::contrast(&imageops::brighten(&imageops::blur(&canvas, 8.0), -30), 0.5).into_raw()
}

fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
    (1.0 - t).mul_add(v0, t * v1)
}
