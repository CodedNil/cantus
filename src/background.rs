use crate::config::CONFIG;
use crate::lerpf64;
use crate::spotify::{
    ALBUM_DATA_CACHE, ARTIST_DATA_CACHE, AlbumData, IMAGES_CACHE, PLAYBACK_STATE,
};
use auto_palette::Palette;
use image::imageops::colorops;
use image::{RgbaImage, imageops};
use itertools::Itertools;
use palette::{Hsl, IntoColor, Srgb};

use vello::peniko::{Blob, ImageAlphaType, ImageBrush, ImageData, ImageFormat};

/// Number of swatches to use in colour palette generation.
const NUM_SWATCHES: usize = 4;

/// Dimensions of the generated palette-based textures.
fn palette_image_height() -> u32 {
    CONFIG.height as u32 * 2
}

fn palette_image_width() -> u32 {
    palette_image_height() * 3
}

/// Number of refinement passes when synthesising the background texture.
const PALETTE_PASS_COUNT: usize = 3;
/// Maximum number of brush placements per pass.
const PALETTE_STROKES_PER_PASS: usize = 20;

/// Downloads and caches an image from the given URL.
pub fn update_color_palettes() {
    let state = PLAYBACK_STATE.read();
    for track in &state.queue {
        if ALBUM_DATA_CACHE.contains_key(&track.album.id) {
            continue;
        }
        let Some(image_ref) = IMAGES_CACHE.get(&track.album.image) else {
            continue;
        };
        let Some(image) = image_ref.as_ref() else {
            continue;
        };
        let Some(artist_image_url_ref) = ARTIST_DATA_CACHE
            .get(&track.artist.id)
            .map(|entry| entry.value().clone())
        else {
            continue;
        };
        ALBUM_DATA_CACHE.insert(track.album.id, None);

        let width = image.image.width;
        let height = image.image.height;
        let album_image =
            RgbaImage::from_raw(width, height, image.image.data.data().to_vec()).unwrap();

        let get_swatches = |img_data| {
            let palette: Palette<f64> = Palette::builder()
                .algorithm(auto_palette::Algorithm::SLIC)
                .filter(ChromaFilter { threshold: 30 })
                .build(&img_data)
                .unwrap();
            palette
                .find_swatches_with_theme(NUM_SWATCHES, auto_palette::Theme::Light)
                .or_else(|_| palette.find_swatches(NUM_SWATCHES))
                .unwrap()
        };

        let mut swatches =
            get_swatches(auto_palette::ImageData::new(width, height, &album_image).unwrap());
        if swatches.len() < NUM_SWATCHES
            && let Some(artist_image_url) = artist_image_url_ref.as_ref()
        {
            let Some(artist_image_ref) = IMAGES_CACHE.get(artist_image_url) else {
                ALBUM_DATA_CACHE.remove(&track.album.id);
                continue;
            };
            let Some(artist_image) = artist_image_ref.as_ref() else {
                ALBUM_DATA_CACHE.remove(&track.album.id);
                continue;
            };
            let artist_new_width = (width as f32 * 0.1).round() as u32;
            let mut new_img = RgbaImage::new(width + artist_new_width, height);
            image::imageops::overlay(&mut new_img, &album_image, 0, 0);
            let artist_img_resized = image::imageops::resize(
                &image::RgbaImage::from_raw(
                    artist_image.image.width,
                    artist_image.image.height,
                    artist_image.image.data.data().to_vec(),
                )
                .unwrap(),
                artist_new_width,
                height,
                image::imageops::FilterType::Nearest,
            );
            image::imageops::overlay(&mut new_img, &artist_img_resized, i64::from(width), 0);

            swatches = get_swatches(
                auto_palette::ImageData::new(new_img.width(), new_img.height(), &new_img).unwrap(),
            );
        }

        let total_ratio_sum: f64 = swatches.iter().map(auto_palette::Swatch::ratio).sum();
        let primary_colors = swatches
            .iter()
            .map(|s| {
                let rgb = s.color().to_rgb();
                let ratio = ((s.ratio() / total_ratio_sum) as f32 * 255.0).round() as u8;
                [rgb.r, rgb.g, rgb.b, ratio]
            })
            .sorted_by(|a, b| b[3].cmp(&a[3]))
            .collect::<Vec<_>>();

        let palette_image = ImageData {
            data: Blob::from(generate_palette_image(&album_image, &primary_colors)),
            format: ImageFormat::Rgba8,
            alpha_type: ImageAlphaType::Alpha,
            width: palette_image_width(),
            height: palette_image_height(),
        };
        ALBUM_DATA_CACHE.insert(
            track.album.id,
            Some(AlbumData {
                primary_colors,
                palette_image: ImageBrush::new(palette_image),
            }),
        );
    }
    drop(state);
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

fn generate_palette_image(album_image: &RgbaImage, colors: &[[u8; 4]]) -> Vec<u8> {
    let palette_width = palette_image_width();
    let palette_height = palette_image_height();

    if colors.is_empty() {
        return RgbaImage::from_pixel(
            palette_width,
            palette_height,
            image::Rgba([50, 50, 50, 255]),
        )
        .into_raw();
    }

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

    // Start with the base image
    let mut canvas = image::imageops::resize(
        album_image,
        palette_width,
        palette_height,
        image::imageops::FilterType::Triangle,
    );

    // Refinement passes will rebalance ratios.
    let total_pixels = (palette_width * palette_height) as f32;
    let mut coverage = vec![0.0; colors.len()];
    let mut per_color_strokes = vec![0; colors.len()];
    let mut available_indices = Vec::with_capacity(colors.len());
    for pass in 0..PALETTE_PASS_COUNT {
        let base_height =
            f64::from(palette_height) * lerpf64(pass as f64 / PALETTE_PASS_COUNT as f64, 0.5, 0.1);

        // Blur the image slightly on each pass
        canvas = imageops::blur(&canvas, 8.0);

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
            let index_to_pick = fastrand::usize(0..available_indices.len());
            let color_index = available_indices[index_to_pick];
            let strokes_left = &mut per_color_strokes[color_index];
            *strokes_left = strokes_left.saturating_sub(1);
            if *strokes_left == 0 {
                available_indices.swap_remove(index_to_pick);
            }
            let color = colors[color_index];

            // Create the brush
            let brush_height = (base_height * lerpf64(fastrand::f64(), 0.75, 1.2))
                .round()
                .clamp(6.0, f64::from(palette_height)) as u32;
            let brush_width = brush_height * 4;

            // Overlay the brush onto the canvas
            let fade_factor = lerpf64(fastrand::f64(), 0.55, 0.9);
            let x = fastrand::i64(0..=i64::from(palette_width)) - i64::from(brush_height / 2);
            let y = fastrand::i64(0..=i64::from(palette_height)) - i64::from(brush_height / 2);

            // Crop our top image if we're going out of bounds
            let origin_bottom_x = x.clamp(0, i64::from(palette_width)) as u32;
            let origin_bottom_y = y.clamp(0, i64::from(palette_height)) as u32;

            let range_width = (x
                .saturating_add(i64::from(brush_width))
                .clamp(0, i64::from(palette_width)) as u32)
                .saturating_sub(origin_bottom_x);
            let range_height = (y
                .saturating_add(i64::from(brush_height))
                .clamp(0, i64::from(palette_height)) as u32)
                .saturating_sub(origin_bottom_y);

            let raw_bottom: &mut [u8] = canvas.as_mut();
            let bottom_stride = palette_width as usize * 4;

            let center_x = x as f64 + f64::from(brush_width) / 2.0;
            let center_y = y as f64 + f64::from(brush_height) / 2.0;
            let inv_half_w = 2.0 / f64::from(brush_width);
            let inv_half_h = 2.0 / f64::from(brush_height);

            for cy in origin_bottom_y..origin_bottom_y + range_height {
                let dy = (f64::from(cy) - center_y) * inv_half_h;
                let dy_sq = dy * dy;
                let row_start = (cy as usize) * bottom_stride;

                for cx in origin_bottom_x..origin_bottom_x + range_width {
                    let dx = (f64::from(cx) - center_x) * inv_half_w;
                    let dist_sq = dx * dx + dy_sq;

                    if dist_sq >= 1.0 {
                        continue;
                    }

                    let alpha = (1.0 - dist_sq.sqrt()) * fade_factor;
                    let src_a = (alpha * 255.0) as u32;
                    if src_a == 0 {
                        continue;
                    }

                    let idx = row_start + (cx as usize) * 4;
                    let inv_a = 255 - src_a;
                    let dst_a = u32::from(raw_bottom[idx + 3]);
                    let out_a = src_a + (dst_a * inv_a / 255);

                    for i in 0..3 {
                        raw_bottom[idx + i] = ((u32::from(color[i]) * src_a
                            + u32::from(raw_bottom[idx + i]) * dst_a * inv_a / 255)
                            / out_a) as u8;
                    }
                    raw_bottom[idx + 3] = out_a as u8;
                }
            }
        }
    }

    // Blur the image, and adjust its brightness, contrast & vibrancy
    colorops::brighten_in_place(&mut canvas, -70);
    let mut canvas = imageops::blur(&canvas, 16.0);
    colorops::contrast_in_place(&mut canvas, -30.0);
    let mut raw_data = canvas.into_raw();
    apply_vibrancy(&mut raw_data, 4.0, 3.0);
    raw_data
}

/// Apply vibrancy to an image in place.
/// boost: The overall saturation increase factor (e.g., 1.5 for a 50% increase).
/// weight: A value > 0 that controls the curve of the effect.
///   - < 1.0: Reduces the preference for dull colors (more uniform boost).
///   - > 1.0: Increases the preference for dull colors (stronger effect on low saturation).
fn apply_vibrancy(raw_data: &mut [u8], boost: f32, weight: f32) {
    for chunk in raw_data.chunks_exact_mut(4) {
        // Convert Rgba<u8> to Srgba<f32>
        let mut srgb: Srgb<f32> = Srgb::new(
            f32::from(chunk[0]) / 255.0,
            f32::from(chunk[1]) / 255.0,
            f32::from(chunk[2]) / 255.0,
        );

        // Apply vibrancy boost
        let mut hsl: Hsl = srgb.into_color();
        let boost_factor = 1.0 + (boost - 1.0) * (1.0 - hsl.saturation).powf(weight);
        hsl.saturation = (hsl.saturation * boost_factor).min(1.0);

        // Convert back to Srgba<f32>
        srgb = hsl.into_color();

        // Update the raw Vec<u8> data in place
        chunk[0] = (srgb.red * 255.0).round() as u8;
        chunk[1] = (srgb.green * 255.0).round() as u8;
        chunk[2] = (srgb.blue * 255.0).round() as u8;
        chunk[3] = 255;
    }
}
