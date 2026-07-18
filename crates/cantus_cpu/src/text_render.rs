use crate::{PANEL_START, model::Track};
use cantus_shared::{GLYPH_ATLAS_SIZE, GlyphInstance, MAX_GLYPH_INSTANCES, pack_u16x2};
use glam::{Vec2, vec2};
use std::collections::HashMap;
use swash::{
    FontRef, GlyphId,
    scale::{Render, ScaleContext, Source},
    shape::ShapeContext,
};
use wgpu::{
    Device, Extent3d, Queue, Texture, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureView, TextureViewDescriptor,
};

const FONT_DATA: &[u8] = include_bytes!("../../../assets/NotoSans-Variable.ttf");

/// Size of the glyph atlas texture (square, in pixels).
const ATLAS_PADDING: u32 = 2;
const RASTER_OVERSAMPLE: f32 = 2.5;
const SCALE_STEPS: f32 = 4.0;

#[derive(Clone, Copy)]
pub struct TextStyle {
    size: f32,
    weight: u16,
}

impl TextStyle {
    pub const PRIMARY: Self = Self::new(16.0, 700);
    pub const TODAY: Self = Self::new(16.0, 900);
    pub const DETAILS: Self = Self::new(14.0, 700);
    pub const WEATHER: Self = Self::new(24.0, 600);

    const fn new(size: f32, weight: u16) -> Self {
        Self { size, weight }
    }
}

#[derive(Clone, Copy)]
struct AtlasEntry {
    pos: [u32; 2],
    size: [u32; 2],
    bearing: [f32; 2],
}

pub struct TextRenderer {
    panel_height: f32,
    font: FontRef<'static>,
    height_to_em: f32,
    scale_context: ScaleContext,
    shape_context: ShapeContext,
    /// Glyph atlas texture.
    atlas: Texture,
    /// Packed glyph data keyed by glyph ID and raster size.
    atlas_cache: HashMap<(u16, GlyphId, u16), AtlasEntry>,
    /// Current write cursor in the atlas (x, y, `row_height`).
    atlas_cursor: (u32, u32, u32),
    shaped: Vec<(GlyphId, Vec2)>,
    /// Queued glyph instances for the current frame.
    pub glyphs: Vec<GlyphInstance>,
}

impl TextRenderer {
    pub fn new(device: &Device, panel_height: f32) -> Self {
        let font = FontRef::from_index(FONT_DATA, 0).unwrap();
        let metrics = font.metrics(&[]);
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
            height_to_em: f32::from(metrics.units_per_em) / (metrics.ascent + metrics.descent),
            scale_context: ScaleContext::new(),
            shape_context: ShapeContext::new(),
            atlas,
            atlas_cache: HashMap::new(),
            atlas_cursor: (0, 0, 0),
            shaped: Vec::with_capacity(128),
            glyphs: Vec::with_capacity(MAX_GLYPH_INSTANCES),
        }
    }

    pub fn atlas_view(&self) -> TextureView {
        self.atlas.create_view(&TextureViewDescriptor::default())
    }

    pub fn track_width(&mut self, track: &Track) -> f32 {
        let text_width = self
            .shape(song_name(track), TextStyle::PRIMARY)
            .0
            .max(self.shape(&track_details(track), TextStyle::DETAILS).0);
        text_width + self.panel_height + 20.0
    }

    fn shape(&mut self, text: &str, style: TextStyle) -> (f32, f32) {
        self.shaped.clear();
        let (context, output) = (&mut self.shape_context, &mut self.shaped);
        let mut shaper = context
            .builder(self.font)
            .size(style.size * self.height_to_em)
            .variations([("wght", f32::from(style.weight))])
            .build();
        let metrics = shaper.metrics();
        shaper.add_str(text);
        let mut x = 0.0;
        shaper.shape_with(|cluster| {
            for glyph in cluster.glyphs {
                output.push((glyph.id, vec2(x + glyph.x, -glyph.y)));
                x += glyph.advance;
            }
        });
        (x, (metrics.ascent - metrics.descent) * 0.5)
    }

    fn rasterize_glyph(&mut self, queue: &Queue, key: (u16, GlyphId, u16)) -> Option<AtlasEntry> {
        if let Some(&entry) = self.atlas_cache.get(&key) {
            return Some(entry);
        }

        let mut scaler = self
            .scale_context
            .builder(self.font)
            .size(f32::from(key.2) / SCALE_STEPS * self.height_to_em)
            .variations([("wght", f32::from(key.0))])
            .build();
        let image = Render::new(&[Source::Outline]).render(&mut scaler, key.1)?;
        let (width, height) = (image.placement.width, image.placement.height);

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

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.atlas,
                mip_level: 0,
                aspect: wgpu::TextureAspect::All,
                origin: wgpu::Origin3d { x: gx, y: gy, z: 0 },
            },
            &image.data,
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
            bearing: [image.placement.left as f32, -image.placement.top as f32],
        };
        self.atlas_cache.insert(key, entry);
        Some(entry)
    }

    pub fn render(&mut self, queue: &Queue, track: &Track, alpha: f32, render_scale: f32) {
        let left = track.runtime.start_x + 12.0;
        let right = track.runtime.start_x + track.runtime.width - self.panel_height - 8.0;
        let available_width = right - left;
        if available_width <= 0.0 {
            return;
        }

        let alpha = alpha.clamp(0.0, 1.0);
        let top = PANEL_START + (self.panel_height * 0.26).floor();
        let bottom = PANEL_START + (self.panel_height * 0.57).floor();
        let mut line = |text: &str, y, style| {
            let (width, baseline) = self.shape(text, style);
            let fits = width <= available_width;
            self.queue_glyphs(
                queue,
                vec2(
                    if fits {
                        (left + right - width) * 0.5
                    } else {
                        left
                    },
                    y,
                ),
                style,
                baseline,
                alpha,
                if fits { f32::MAX } else { right },
                render_scale,
            );
        };
        line(song_name(track), top, TextStyle::PRIMARY);
        line(&track_details(track), bottom, TextStyle::DETAILS);
    }

    pub fn render_centered_label(
        &mut self,
        queue: &Queue,
        text: &str,
        position: Vec2,
        style: TextStyle,
        alpha: f32,
        render_scale: f32,
    ) {
        let (measured, baseline) = self.shape(text, style);
        self.queue_glyphs(
            queue,
            position - vec2(measured * 0.5, 0.0),
            style,
            baseline,
            alpha,
            f32::MAX,
            render_scale,
        );
    }

    fn queue_glyphs(
        &mut self,
        queue: &Queue,
        origin: Vec2,
        style: TextStyle,
        baseline: f32,
        alpha: f32,
        clip_right: f32,
        render_scale: f32,
    ) {
        let scale_quarters = (style.size * render_scale * RASTER_OVERSAMPLE * SCALE_STEPS)
            .round()
            .max(SCALE_STEPS) as u16;
        let glyph_scale = style.size * SCALE_STEPS / f32::from(scale_quarters);
        let baseline_y = origin.y + baseline;

        for index in 0..self.shaped.len() {
            if self.glyphs.len() == MAX_GLYPH_INSTANCES {
                break;
            }
            let (id, offset) = self.shaped[index];
            let key = (style.weight, id, scale_quarters);
            let Some(glyph) = self.rasterize_glyph(queue, key) else {
                continue;
            };
            self.glyphs.push(GlyphInstance {
                pos: vec2(
                    origin.x + offset.x + glyph.bearing[0] * glyph_scale,
                    baseline_y + offset.y + glyph.bearing[1] * glyph_scale,
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
            });
        }
    }
}

fn song_name(track: &Track) -> &str {
    let name = track
        .name
        .split_once(" -")
        .map_or(track.name.as_str(), |x| x.0);
    let name = name.split_once('(').map_or(name, |x| x.0).trim();
    if name.is_empty() {
        track.name.trim()
    } else {
        name
    }
}

fn track_details(track: &Track) -> String {
    let seconds = (track.runtime.start_ms / 1000.0).abs();
    let time = if seconds >= 60.0 {
        let seconds = seconds as u32;
        format!("{}m{}s", seconds / 60, seconds % 60)
    } else {
        format!("{}s", seconds.round())
    };
    format!("{time}\u{2004}•\u{2004}{}", track.artist)
}
