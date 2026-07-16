use crate::{PANEL_START, model::Track};
use ab_glyph::{Font, FontArc, Glyph, GlyphId, PxScale, ScaleFont, point};
use cantus_shared::{GLYPH_ATLAS_SIZE, GlyphInstance, MAX_GLYPH_INSTANCES, pack_u16x2};
use glam::{Vec2, vec2};
use std::collections::HashMap;
use wgpu::{
    Device, Extent3d, Queue, Texture, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureView, TextureViewDescriptor,
};

const FONT_SIZE: f32 = 16.0;
const FONT_SIZE_SMALL: f32 = 14.0;
const LIGHT_WEIGHT: f32 = 0.08;

/// Size of the glyph atlas texture (square, in pixels).
const ATLAS_PADDING: u32 = 1;
const RASTER_OVERSAMPLE: f32 = 2.0;
const SCALE_STEPS: f32 = 4.0;

#[derive(Clone, Copy)]
struct AtlasEntry {
    pos: [u32; 2],
    size: [u32; 2],
    bearing: [f32; 2],
}

pub struct TextRenderer {
    panel_height: f32,
    font: FontArc,
    /// Glyph atlas texture.
    atlas: Texture,
    /// Packed glyph data keyed by glyph ID and raster size.
    atlas_cache: HashMap<(GlyphId, u16), AtlasEntry>,
    /// Current write cursor in the atlas (x, y, `row_height`).
    atlas_cursor: (u32, u32, u32),
    /// Queued glyph instances for the current frame.
    pub glyphs: Vec<GlyphInstance>,
}

impl TextRenderer {
    pub fn new(device: &Device, panel_height: f32) -> Self {
        let font =
            FontArc::try_from_slice(include_bytes!("../../../assets/NotoSans-Bold.ttf")).unwrap();

        let atlas = device.create_texture(&TextureDescriptor {
            label: Some("Glyph Atlas"),
            size: Extent3d {
                width: GLYPH_ATLAS_SIZE,
                height: GLYPH_ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        Self {
            panel_height,
            font,
            atlas,
            atlas_cache: HashMap::new(),
            atlas_cursor: (0, 0, 0),
            glyphs: Vec::with_capacity(MAX_GLYPH_INSTANCES),
        }
    }

    pub fn atlas_view(&self) -> TextureView {
        self.atlas.create_view(&TextureViewDescriptor::default())
    }

    fn rasterize_glyph(&mut self, queue: &Queue, key: (GlyphId, u16)) -> Option<AtlasEntry> {
        if let Some(&entry) = self.atlas_cache.get(&key) {
            return Some(entry);
        }

        let scale = PxScale::from(f32::from(key.1) / SCALE_STEPS);
        let glyph = Glyph {
            id: key.0,
            scale,
            position: point(0.0, 0.0),
        };
        let outlined = self.font.as_scaled(scale).outline_glyph(glyph)?;
        let bounds = outlined.px_bounds();
        let width = bounds.width() as u32;
        let height = bounds.height() as u32;

        if width == 0 || height == 0 {
            return None;
        }

        // Leave a transparent texel around glyphs so linear filtering cannot
        // sample coverage from a neighbouring atlas entry.
        // Simple row-based packing; if it doesn't fit, start a new row.
        let (mut cx, mut cy, mut row_h) = self.atlas_cursor;
        if cx + width + ATLAS_PADDING * 2 > GLYPH_ATLAS_SIZE {
            cy += row_h;
            cx = 0;
            row_h = 0;
        }
        if cy + height + ATLAS_PADDING * 2 > GLYPH_ATLAS_SIZE {
            return None;
        }
        let gx = cx + ATLAS_PADDING;
        let gy = cy + ATLAS_PADDING;
        let row_h = row_h.max(height + ATLAS_PADDING * 2);
        self.atlas_cursor = (cx + width + ATLAS_PADDING * 2, cy, row_h);

        let mut buffer = vec![0u8; (width * height) as usize];
        outlined.draw(|x, y, c| {
            buffer[y as usize * width as usize + x as usize] = (c * 255.0).round() as u8;
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.atlas,
                mip_level: 0,
                aspect: wgpu::TextureAspect::All,
                origin: wgpu::Origin3d { x: gx, y: gy, z: 0 },
            },
            &buffer,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width),
                rows_per_image: Some(height),
            },
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let entry = AtlasEntry {
            pos: [gx, gy],
            size: [width, height],
            bearing: [bounds.min.x, bounds.min.y],
        };
        self.atlas_cache.insert(key, entry);
        Some(entry)
    }

    pub fn render(&mut self, queue: &Queue, track: &Track, alpha: f32, render_scale: f32) {
        let text_start_left = track.runtime.start_x + 12.0;
        let text_start_right = track.runtime.end_x() - self.panel_height - 8.0;
        let available_width = text_start_right - text_start_left;

        if available_width <= 0.0 {
            return;
        }

        let alpha = alpha.clamp(0.0, 1.0);

        let without_suffix = track
            .name
            .split_once(" -")
            .map_or(track.name.as_str(), |(prefix, _)| prefix);
        let song_name = without_suffix
            .split_once('(')
            .map_or(without_suffix, |(prefix, _)| prefix)
            .trim();
        let song_name = if song_name.is_empty() {
            track.name.trim()
        } else {
            song_name
        };
        let top_y = PANEL_START + (self.panel_height * 0.26).floor();
        let bottom_y = PANEL_START + (self.panel_height * 0.57).floor();

        let measured_width = measure_text(&self.font, song_name, FONT_SIZE);

        let width_ratio = available_width / measured_width;
        let (x, size, align) = if width_ratio <= 1.0 {
            (
                text_start_left,
                FONT_SIZE * width_ratio.max(0.8),
                Align::Left,
            )
        } else {
            (text_start_right, FONT_SIZE, Align::Right)
        };

        let seconds_until_start = (track.runtime.start_ms / 1000.0).abs();
        let time_text = if seconds_until_start >= 60.0 {
            let seconds = seconds_until_start as u32;
            format!("{}m{}s", seconds / 60, seconds % 60)
        } else {
            format!("{}s", seconds_until_start.round())
        };

        let bottom_merged = format!("{time_text}\u{2004}•\u{2004}{}", track.artist.name);
        let measured_bottom_width = measure_text(&self.font, &bottom_merged, FONT_SIZE_SMALL);
        let bottom_ratio = available_width / measured_bottom_width;
        let split_widths = (bottom_ratio > 1.0 && track.is_current()).then(|| {
            (
                measure_text(&self.font, &time_text, FONT_SIZE_SMALL),
                measure_text(&self.font, &track.artist.name, FONT_SIZE_SMALL),
            )
        });

        let mut queue_text = |text, width, origin, size, align| {
            self.queue_glyphs(
                queue,
                text,
                width,
                origin,
                size,
                align,
                alpha,
                0.0,
                text_start_right,
                render_scale,
            );
        };
        queue_text(
            song_name,
            measured_width * size / FONT_SIZE,
            vec2(x, top_y),
            size,
            align,
        );

        if let Some((time_width, artist_width)) = split_widths {
            queue_text(
                &time_text,
                time_width,
                vec2(text_start_left, bottom_y),
                FONT_SIZE_SMALL,
                Align::Left,
            );
            queue_text(
                &track.artist.name,
                artist_width,
                vec2(text_start_right, bottom_y),
                FONT_SIZE_SMALL,
                Align::Right,
            );
        } else {
            let (x, align) = if bottom_ratio >= 1.0 {
                (text_start_right, Align::Right)
            } else {
                (text_start_left, Align::Left)
            };
            let size = FONT_SIZE_SMALL * bottom_ratio.clamp(0.8, 1.0);
            queue_text(
                &bottom_merged,
                measured_bottom_width * size / FONT_SIZE_SMALL,
                vec2(x, bottom_y),
                size,
                align,
            );
        }
    }

    pub fn render_label(
        &mut self,
        queue: &Queue,
        text: &str,
        x: f32,
        width: f32,
        max_size: f32,
        render_scale: f32,
        light: bool,
    ) {
        let measured = measure_text(&self.font, text, max_size);
        let size = max_size * ((width - 20.0) / measured.max(1.0)).min(1.0);
        let measured = measured * size / max_size;
        self.queue_glyphs(
            queue,
            text,
            measured,
            vec2(
                x + (width - measured) * 0.5,
                PANEL_START + self.panel_height * 0.46,
            ),
            size,
            Align::Left,
            1.0,
            if light { LIGHT_WEIGHT } else { 0.0 },
            x + width - 8.0,
            render_scale,
        );
    }

    fn queue_glyphs(
        &mut self,
        queue: &Queue,
        text: &str,
        total_width: f32,
        origin: Vec2,
        px_size: f32,
        align: Align,
        alpha: f32,
        weight: f32,
        clip_right: f32,
        render_scale: f32,
    ) {
        let scaled_font = self.font.as_scaled(px_size);
        let baseline_offset = (scaled_font.ascent() + scaled_font.descent()) * 0.5;

        let caret = match align {
            Align::Left => origin.x,
            Align::Right => origin.x - total_width,
        };

        let scale_quarters = (px_size * render_scale * RASTER_OVERSAMPLE * SCALE_STEPS)
            .round()
            .max(SCALE_STEPS) as u16;
        let glyph_scale = px_size * SCALE_STEPS / f32::from(scale_quarters);
        let clip_right = match align {
            Align::Left if total_width - (clip_right - origin.x) > 0.5 / render_scale => clip_right,
            _ => f32::MAX,
        };
        let baseline_y = origin.y + baseline_offset;

        let font = self.font.clone();
        for (glyph_id, glyph_x, _) in layout_glyphs(&font, text, px_size, caret) {
            if self.glyphs.len() == MAX_GLYPH_INSTANCES {
                break;
            }
            let key = (glyph_id, scale_quarters);
            let Some(glyph) = self.rasterize_glyph(queue, key) else {
                continue;
            };
            self.glyphs.push(GlyphInstance {
                pos: vec2(
                    glyph_x + glyph.bearing[0] * glyph_scale,
                    baseline_y + glyph.bearing[1] * glyph_scale,
                ),
                size: vec2(
                    glyph.size[0] as f32 * glyph_scale,
                    glyph.size[1] as f32 * glyph_scale,
                ),
                atlas: [
                    pack_u16x2(glyph.pos),
                    pack_u16x2([glyph.pos[0] + glyph.size[0], glyph.pos[1] + glyph.size[1]]),
                ],
                clip_right,
                alpha,
                weight,
                padding: 0.0,
            });
        }
    }
}

#[derive(Copy, Clone)]
enum Align {
    Left,
    Right,
}

fn measure_text(font: &FontArc, text: &str, px_size: f32) -> f32 {
    layout_glyphs(font, text, px_size, 0.0)
        .last()
        .map_or(0.0, |(_, _, end)| end)
}

fn layout_glyphs<'a>(
    font: &'a FontArc,
    text: &'a str,
    px_size: f32,
    mut caret: f32,
) -> impl Iterator<Item = (GlyphId, f32, f32)> + 'a {
    let font = font.as_scaled(px_size);
    let mut previous = None;
    text.chars().map(move |c| {
        let glyph_id = font.glyph_id(c);
        if let Some(prev) = previous {
            caret += font.kern(prev, glyph_id);
        }
        let start = caret;
        caret += font.h_advance(glyph_id);
        previous = Some(glyph_id);
        (glyph_id, start, caret)
    })
}
