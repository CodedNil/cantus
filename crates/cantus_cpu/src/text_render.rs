use crate::{PANEL_START, Track};
use ab_glyph::{Font, FontArc, Glyph, GlyphId, PxScale, ScaleFont, point};
use cantus_shared::{GlyphInstance, MAX_GLYPH_INSTANCES};
use glam::vec2;
use std::collections::HashMap;
use wgpu::{
    Device, Extent3d, Queue, Texture, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureView, TextureViewDescriptor,
};

const FONT_SIZE: f32 = 17.0;
const FONT_SIZE_SMALL: f32 = 14.0;

/// Size of the glyph atlas texture (square, in pixels).
const ATLAS_SIZE: u32 = 2048;
const ATLAS_PADDING: u32 = 1;
const POSITION_STEPS: f32 = 10.0;
const SCALE_STEPS: f32 = 4.0;

fn display_name(name: &str) -> &str {
    let without_suffix = name.split_once(" -").map_or(name, |(prefix, _)| prefix);
    let display = without_suffix
        .split_once('(')
        .map_or(without_suffix, |(prefix, _)| prefix)
        .trim();
    if display.is_empty() {
        name.trim()
    } else {
        display
    }
}

#[derive(Hash, Eq, PartialEq)]
struct AtlasKey {
    glyph_id: u16,
    scale_quarters: u16,
    phase_x: u8,
    phase_y: u8,
}

#[derive(Clone, Copy)]
struct AtlasEntry {
    pos: [u32; 2],
    size: [u32; 2],
    bearing: [i32; 2],
}

pub struct TextRenderer {
    panel_height: f32,
    font: FontArc,
    /// Glyph atlas texture.
    atlas: Texture,
    atlas_view: TextureView,
    /// Packed glyph data keyed by glyph ID, size, and subpixel phase.
    atlas_cache: HashMap<AtlasKey, AtlasEntry>,
    /// Current write cursor in the atlas (x, y, `row_height`).
    atlas_cursor: (u32, u32, u32),
    /// Queued glyph instances for the current frame.
    glyphs: Vec<GlyphInstance>,
}

impl TextRenderer {
    pub fn new(device: &Device, panel_height: f32) -> Self {
        let font =
            FontArc::try_from_slice(include_bytes!("../../../assets/NotoSans-Bold.ttf")).unwrap();

        let atlas = device.create_texture(&TextureDescriptor {
            label: Some("Glyph Atlas"),
            size: Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let atlas_view = atlas.create_view(&TextureViewDescriptor::default());

        Self {
            panel_height,
            font,
            atlas,
            atlas_view,
            atlas_cache: HashMap::new(),
            atlas_cursor: (0, 0, 0),
            glyphs: Vec::new(),
        }
    }

    pub const fn atlas_view(&self) -> &TextureView {
        &self.atlas_view
    }

    fn rasterize_glyph(&mut self, queue: &Queue, key: AtlasKey) -> Option<AtlasEntry> {
        if let Some(&entry) = self.atlas_cache.get(&key) {
            return Some(entry);
        }

        let scale = PxScale::from(f32::from(key.scale_quarters) / SCALE_STEPS);
        let glyph = Glyph {
            id: GlyphId(key.glyph_id),
            scale,
            position: point(
                f32::from(key.phase_x) / POSITION_STEPS,
                f32::from(key.phase_y) / POSITION_STEPS,
            ),
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
        if cx + width + ATLAS_PADDING * 2 > ATLAS_SIZE {
            cy += row_h;
            cx = 0;
            row_h = 0;
        }
        if cy + height + ATLAS_PADDING * 2 > ATLAS_SIZE {
            return None;
        }
        let gx = cx + ATLAS_PADDING;
        let gy = cy + ATLAS_PADDING;
        let row_h = row_h.max(height + ATLAS_PADDING * 2);
        self.atlas_cursor = (cx + width + ATLAS_PADDING * 2, cy, row_h);

        let mut buf = vec![0u8; (width * height) as usize];
        outlined.draw(|x, y, c| {
            buf[y as usize * width as usize + x as usize] = (c * 255.0).round() as u8;
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.atlas,
                mip_level: 0,
                aspect: wgpu::TextureAspect::All,
                origin: wgpu::Origin3d { x: gx, y: gy, z: 0 },
            },
            &buf,
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
            bearing: [bounds.min.x as i32, bounds.min.y as i32],
        };
        self.atlas_cache.insert(key, entry);
        Some(entry)
    }

    pub fn render(&mut self, queue: &Queue, track: &Track, render_scale: f32) {
        let text_start_left = track.runtime.start_x + 12.0;
        let text_start_right =
            track.runtime.start_x + track.runtime.width - self.panel_height - 8.0;
        let available_width = text_start_right - text_start_left;

        if available_width <= 0.0 {
            return;
        }

        let text_color: [f32; 4] = [0.94, 0.94, 0.94, (available_width / 100.0).min(1.0)];
        let color_packed = pack_color(text_color);

        let song_name = display_name(&track.name);
        let top_y = PANEL_START + (self.panel_height * 0.26).floor();
        let bottom_y = PANEL_START + (self.panel_height * 0.57).floor();

        // --- Top line: song name ---
        let measured_width = measure_text(&self.font, song_name, FONT_SIZE);

        let width_ratio = available_width / measured_width;
        let (x, size) = if width_ratio <= 1.0 {
            (text_start_left, FONT_SIZE * width_ratio.max(0.8))
        } else {
            (text_start_right, FONT_SIZE)
        };
        let align = if width_ratio <= 1.0 {
            Align::Left
        } else {
            Align::Right
        };

        queue_glyphs(
            self,
            queue,
            song_name,
            x,
            top_y,
            size,
            FONT_SIZE,
            align,
            color_packed,
            text_start_right,
            render_scale,
        );

        // --- Bottom line: time + artist ---
        let seconds_until_start = (track.runtime.start_ms / 1000.0).abs();
        let time_text = if seconds_until_start >= 60.0 {
            format!(
                "{}m{}s",
                (seconds_until_start / 60.0).floor(),
                (seconds_until_start % 60.0).floor()
            )
        } else {
            format!("{}s", seconds_until_start.round())
        };

        let bottom_merged = format!("{time_text}\u{2004}•\u{2004}{}", track.artist.name);
        let measured_bottom_width = measure_text(&self.font, &bottom_merged, FONT_SIZE_SMALL);

        let bottom_ratio = available_width / measured_bottom_width;

        let is_current = track.runtime.start_ms <= 0.0
            && track.runtime.start_ms + track.duration_ms as f32 >= 0.0;
        if bottom_ratio <= 1.0 || !is_current {
            let (x, align) = if bottom_ratio >= 1.0 {
                (text_start_right, Align::Right)
            } else {
                (text_start_left, Align::Left)
            };
            queue_glyphs(
                self,
                queue,
                &bottom_merged,
                x,
                bottom_y,
                FONT_SIZE_SMALL * bottom_ratio.clamp(0.8, 1.0),
                FONT_SIZE_SMALL,
                align,
                color_packed,
                text_start_right,
                render_scale,
            );
        } else {
            queue_glyphs(
                self,
                queue,
                &time_text,
                text_start_left,
                bottom_y,
                FONT_SIZE_SMALL,
                FONT_SIZE_SMALL,
                Align::Left,
                color_packed,
                text_start_right,
                render_scale,
            );
            queue_glyphs(
                self,
                queue,
                &track.artist.name,
                text_start_right,
                bottom_y,
                FONT_SIZE_SMALL,
                FONT_SIZE_SMALL,
                Align::Right,
                color_packed,
                text_start_right,
                render_scale,
            );
        }
    }

    pub fn begin_frame(&mut self) {
        self.glyphs.clear();
    }

    pub fn glyphs(&self) -> &[GlyphInstance] {
        &self.glyphs
    }
}

#[derive(Copy, Clone)]
enum Align {
    Left,
    Right,
}

fn measure_text(font: &FontArc, text: &str, px_size: f32) -> f32 {
    let font = font.as_scaled(px_size);
    let mut caret = 0.0f32;
    let mut last_glyph: Option<GlyphId> = None;
    for c in text.chars() {
        let glyph_id = font.glyph_id(c);
        if let Some(prev) = last_glyph {
            caret += font.kern(prev, glyph_id);
        }
        caret += font.h_advance(glyph_id);
        last_glyph = Some(glyph_id);
    }
    caret
}

fn pack_color(color: [f32; 4]) -> u32 {
    u32::from_le_bytes(color.map(|channel| (channel * 255.0).round() as u8))
}

fn queue_glyphs(
    renderer: &mut TextRenderer,
    queue: &Queue,
    text: &str,
    origin_x: f32,
    origin_y: f32,
    px_size: f32,
    raster_size: f32,
    align: Align,
    color: u32,
    clip_right: f32,
    render_scale: f32,
) {
    let total_width = measure_text(&renderer.font, text, px_size);
    let scaled_font = renderer.font.as_scaled(px_size);
    let baseline_offset = (scaled_font.ascent() + scaled_font.descent()) * 0.5;

    let caret = match align {
        Align::Left => origin_x,
        Align::Right => origin_x - total_width,
    };

    let scale_quarters = (raster_size * render_scale * SCALE_STEPS)
        .round()
        .max(SCALE_STEPS) as u16;
    let glyph_scale = px_size / raster_size;
    let clip_right = match align {
        Align::Left if total_width - (clip_right - origin_x) > 0.5 / render_scale => clip_right,
        _ => f32::MAX,
    };
    let physical_baseline_y = (origin_y + baseline_offset) * render_scale / glyph_scale;
    let (baseline_y, phase_y) = subpixel_position(physical_baseline_y);

    let font = renderer.font.clone();
    let font = font.as_scaled(px_size);
    let mut caret = caret;
    let mut last_glyph = None;
    for c in text.chars() {
        if renderer.glyphs.len() == MAX_GLYPH_INSTANCES {
            break;
        }
        let glyph_id = font.glyph_id(c);
        if let Some(previous) = last_glyph {
            caret += font.kern(previous, glyph_id);
        }
        let glyph_x = caret;
        caret += font.h_advance(glyph_id);
        last_glyph = Some(glyph_id);

        let (caret_x, phase_x) = subpixel_position(glyph_x * render_scale / glyph_scale);
        let key = AtlasKey {
            glyph_id: glyph_id.0,
            scale_quarters,
            phase_x,
            phase_y,
        };
        let Some(glyph) = renderer.rasterize_glyph(queue, key) else {
            continue;
        };
        let atlas_size = ATLAS_SIZE as f32;
        renderer.glyphs.push(GlyphInstance {
            pos: vec2(
                (caret_x + glyph.bearing[0] as f32) * glyph_scale / render_scale,
                (baseline_y + glyph.bearing[1] as f32) * glyph_scale / render_scale,
            ),
            size: vec2(
                glyph.size[0] as f32 * glyph_scale / render_scale,
                glyph.size[1] as f32 * glyph_scale / render_scale,
            ),
            atlas_min: vec2(
                glyph.pos[0] as f32 / atlas_size,
                glyph.pos[1] as f32 / atlas_size,
            ),
            atlas_max: vec2(
                (glyph.pos[0] + glyph.size[0]) as f32 / atlas_size,
                (glyph.pos[1] + glyph.size[1]) as f32 / atlas_size,
            ),
            clip_right,
            color,
        });
    }
}

fn subpixel_position(position: f32) -> (f32, u8) {
    let base = position.floor();
    let phase = ((position - base) * POSITION_STEPS).round() as u8;
    if phase == POSITION_STEPS as u8 {
        (base + 1.0, 0)
    } else {
        (base, phase)
    }
}
