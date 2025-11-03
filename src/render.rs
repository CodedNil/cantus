use crate::{
    CantusLayer, PANEL_HEIGHT_BASE, PANEL_WIDTH,
    spotify::{IMAGES_CACHE, PLAYBACK_STATE, Playlist, TRACK_DATA_CACHE, Track},
};
use rand::Rng;
use std::{collections::HashMap, ops::Range, time::Instant};
use ttf_parser::{Face, GlyphId, NormalizedCoordinate, Tag, VariationAxis};
use vello::{
    Glyph,
    kurbo::{Affine, Rect, RoundedRect, RoundedRectRadii},
    peniko::{Blob, Color, Fill, FontData, ImageBrush},
};

/// Spacing between tracks in ms
const TRACK_SPACING_MS: f64 = 4000.0;
/// How many ms to show in the timeline
const TIMELINE_DURATION_MS: f64 = 15.0 * 60.0 * 1000.0;
/// Starting position of the timeline in ms, if negative then it shows the history too
const TIMELINE_START_MS: f64 = -3.0 * 60.0 * 1000.0;

/// Corner radius applied to rendered track pills.
const ROUNDING_RADIUS: f64 = 14.0;

/// Particles emitted per second when playback is active.
const SPARK_EMISSION: f32 = 60.0;
/// Downward acceleration applied to each particle (scaled by DPI).
const SPARK_GRAVITY: f32 = 620.0;
/// Normalized vertical spawn range along the divider (0.0 = top, 1.0 = bottom).
const SPARK_SPAWN_Y: Range<f32> = 0.05..0.95;
/// Horizontal velocity range applied at spawn (negative moves sparks left).
const SPARK_VELOCITY_X: Range<f32> = -120.0..-70.0;
/// Vertical velocity range applied at spawn (negative moves sparks upward).
const SPARK_VELOCITY_Y: Range<f32> = -70.0..-40.0;
/// Lifetime range for individual particles, in seconds.
const SPARK_LIFETIME: Range<f32> = 0.3..0.6;
/// Rendered spark segment length range, in logical pixels.
const SPARK_LENGTH_RANGE: Range<f64> = 6.0..10.0;
/// Rendered spark thickness range, in logical pixels.
const SPARK_THICKNESS_RANGE: Range<f64> = 2.0..4.0;

#[derive(Clone)]
pub struct FontEngine {
    font_data: FontData,
    base_face: Face<'static>,
    base_metrics: FontMetrics,
    axes: Vec<VariationAxis>,
    weight_axis_index: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
struct FontMetrics {
    units_per_em: f32,
    ascender: f32,
    descender: f32,
    line_gap: f32,
    space_advance: f32,
}

impl FontMetrics {
    fn from_face(face: &Face<'_>) -> Self {
        let units_per_em = f32::from(face.units_per_em()).max(1.0);
        let ascender = f32::from(face.ascender()).max(0.0);
        let descender = f32::from(-face.descender()).max(0.0);
        let line_gap = f32::from(face.line_gap()).max(0.0);
        let space_advance = face
            .glyph_index(' ')
            .and_then(|gid| face.glyph_hor_advance(gid))
            .map_or(units_per_em * 0.5, f32::from);
        Self {
            units_per_em,
            ascender,
            descender,
            line_gap,
            space_advance,
        }
    }

    const fn line_height_units(&self) -> f32 {
        self.ascender + self.descender + self.line_gap
    }
}

fn axis_normalized_value(axis: &VariationAxis, value: f32) -> i16 {
    let mut v = value.clamp(axis.min_value, axis.max_value);
    if (v - axis.def_value).abs() < f32::EPSILON {
        return 0;
    }
    if v < axis.def_value {
        let denom = axis.def_value - axis.min_value;
        if denom.abs() < f32::EPSILON {
            return 0;
        }
        v = (v - axis.def_value) / denom;
    } else {
        let denom = axis.max_value - axis.def_value;
        if denom.abs() < f32::EPSILON {
            return 0;
        }
        v = (v - axis.def_value) / denom;
    }
    NormalizedCoordinate::from(v).get()
}

#[derive(Clone, Copy)]
enum Align {
    Start,
    Center,
    End,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum FontWeight {
    Regular,
    Bold,
    Value(f32),
}

impl FontWeight {
    const fn value_for(self, axis: &VariationAxis) -> f32 {
        let target = match self {
            Self::Regular => axis.def_value,
            Self::Bold => 700.0,
            Self::Value(v) => v,
        };
        target.clamp(axis.min_value, axis.max_value)
    }
}

pub struct TextLayout {
    glyphs: Vec<Glyph>,
    width: f32,
    height: f32,
    font_size: f32,
    coords: Vec<i16>,
}

impl FontEngine {
    pub fn new(bytes: &'static [u8]) -> Self {
        let blob = Blob::from(bytes.to_vec());
        let font_data = FontData::new(blob, 0);
        let face = Face::parse(bytes, 0).expect("failed to parse embedded font");
        let base_metrics = FontMetrics::from_face(&face);
        let axes = face.variation_axes().into_iter().collect::<Vec<_>>();
        let weight_axis_index = axes
            .iter()
            .position(|axis| axis.tag == Tag::from_bytes(b"wght"));

        Self {
            font_data,
            base_face: face,
            base_metrics,
            axes,
            weight_axis_index,
        }
    }

    fn kerning_units(face: &Face<'_>, left: GlyphId, right: GlyphId) -> f32 {
        let Some(kern_table) = face.tables().kern else {
            return 0.0;
        };
        let mut adjustment = 0.0f32;
        for subtable in kern_table.subtables {
            if subtable.horizontal
                && !subtable.has_cross_stream
                && !subtable.has_state_machine
                && let Some(value) = subtable.glyphs_kerning(left, right)
            {
                adjustment += f32::from(value);
            }
        }
        adjustment
    }
}

/// Build the scene for rendering.
impl CantusLayer {
    pub fn create_scene(&mut self, device_id: usize) {
        let start = Instant::now();
        let total_width = (PANEL_WIDTH * self.scale_factor).ceil();
        let total_height = (PANEL_HEIGHT_BASE * self.scale_factor).ceil();

        let playback_state = PLAYBACK_STATE.lock();
        let queue = &playback_state.queue;
        self.icon_hitboxes.clear();
        self.track_hitboxes.clear();
        if queue.is_empty() {
            return;
        }

        let timeline_end_ms = TIMELINE_START_MS + TIMELINE_DURATION_MS;
        let px_per_ms = total_width / TIMELINE_DURATION_MS;
        let current_index = playback_state.queue_index.min(queue.len() - 1);

        // Borrow playlists for quick lookups without cloning each entry.
        let playlists: HashMap<&str, &Playlist> = playback_state
            .playlists
            .iter()
            .map(|playlist| (playlist.name.as_str(), playlist))
            .collect();

        // Lerp the progress based on when the data was last updated, get the start time of the current track
        let playback_elapsed = if playback_state.playing {
            playback_state.last_updated.elapsed().as_millis() as f64
        } else {
            0.0
        };
        let mut track_start_target = -f64::from(playback_state.progress) - playback_elapsed;
        track_start_target -= queue[..current_index]
            .iter()
            .map(|track| f64::from(track.milliseconds))
            .sum::<f64>();
        let mut track_spacing_target = -TRACK_SPACING_MS * current_index as f64;

        // Lerp track start based on the target and current start time
        if (track_start_target - self.track_start_ms).abs() > 200.0 {
            track_start_target =
                self.track_start_ms + (track_start_target - self.track_start_ms) * 0.1;
        }
        self.track_start_ms = track_start_target;

        // Lerp track spacing based on the target and current spacing
        if (track_spacing_target - self.track_spacing).abs() > 200.0 {
            track_spacing_target =
                self.track_spacing + (track_spacing_target - self.track_spacing) * 0.1;
        }
        self.track_spacing = track_spacing_target;

        let mut track_start_ms = self.track_start_ms;
        let mut track_spacing = self.track_spacing;

        // Iterate over the currently playing track followed by the queued tracks.
        for (index, track) in queue.iter().enumerate() {
            let track_start_ms_spaced = track_start_ms + track_spacing;
            if track_start_ms_spaced >= timeline_end_ms {
                break;
            }

            let track_end_ms = track_start_ms_spaced + f64::from(track.milliseconds);
            if track_end_ms <= TIMELINE_START_MS {
                track_start_ms += f64::from(track.milliseconds);
                track_spacing += TRACK_SPACING_MS;
                continue;
            }

            // Draw the track, trimming to the visible window if it spills off either side.
            self.draw_track(
                device_id,
                track,
                index == current_index,
                index < current_index,
                track_start_ms_spaced,
                track_end_ms,
                timeline_end_ms,
                px_per_ms,
                total_height,
                (track_start_ms / 1000.0).abs(),
                &playlists,
            );

            track_start_ms += f64::from(track.milliseconds);
            track_spacing += TRACK_SPACING_MS;
        }

        // Draw the particles
        self.render_playing_particles(
            &queue[current_index],
            -TIMELINE_START_MS * px_per_ms,
            total_height,
            playback_state.playing,
        );

        // Purge the stale background cache entries.
        if let Some(bundle) = self.render_devices.get_mut(&device_id) {
            bundle
                .background
                .purge_stale(&mut bundle.renderer, self.frame_index);
        }
        drop(playback_state);
        tracing::info!("Render took {:?}", start.elapsed());
    }

    fn draw_track(
        &mut self,
        device_id: usize,
        track: &Track,
        is_current: bool,
        is_past: bool,
        track_start_ms: f64,
        track_end_ms: f64,
        timeline_end_ms: f64,
        px_per_ms: f64,
        height: f64,
        seconds_until_start: f64,
        playlists: &HashMap<&str, &Playlist>,
    ) {
        let visible_start_ms = track_start_ms.max(TIMELINE_START_MS);
        let visible_end_ms = track_end_ms.min(timeline_end_ms);
        let start_trimmed = track_start_ms >= TIMELINE_START_MS;
        let end_trimmed = track_end_ms <= timeline_end_ms;

        let pos_x = (visible_start_ms - TIMELINE_START_MS) * px_per_ms;
        let width = (visible_end_ms - visible_start_ms) * px_per_ms;
        if width <= 0.0 {
            self.track_hitboxes.remove(&track.id);
            return;
        }
        let uncropped_width = (track_end_ms - track_start_ms) * px_per_ms;

        // How much of the width is to the left of the current position
        let dark_width = if track_start_ms < 0.0 {
            track_start_ms.max(TIMELINE_START_MS) * -px_per_ms
        } else {
            0.0
        };

        // Add hitbox
        self.track_hitboxes.insert(
            track.id.clone(),
            Rect::new(
                ((track_start_ms - TIMELINE_START_MS) * px_per_ms) / self.scale_factor,
                0.0,
                ((track_end_ms - TIMELINE_START_MS) * px_per_ms) / self.scale_factor,
                height / self.scale_factor,
            ),
        );

        let rounding = ROUNDING_RADIUS * self.scale_factor;
        let left_rounding = rounding * if start_trimmed { 1.0 } else { 0.3 };
        let right_rounding = rounding * if end_trimmed { 1.0 } else { 0.3 };
        let radii =
            RoundedRectRadii::new(left_rounding, right_rounding, right_rounding, left_rounding);

        let Some(image) = IMAGES_CACHE.get(&track.image_url) else {
            return;
        };
        let Some(track_data) = TRACK_DATA_CACHE.get(&track.id) else {
            return;
        };
        let background_image = {
            let bundle = self
                .render_devices
                .get_mut(&device_id)
                .expect("render device must exist");
            bundle.background.render(
                &track.image_url,
                &self.render_context.devices[device_id],
                &mut bundle.renderer,
                &track_data.palette_image,
                self.time_origin.elapsed().as_secs_f32(),
                self.frame_index,
            )
        };

        // --- BACKGROUND ---
        let background_aspect_ratio = (width - height * 0.5) / height;
        self.scene.push_clip_layer(
            Affine::translate((pos_x, 0.0)),
            &RoundedRect::new(
                0.0,
                0.0,
                width - height * 0.25, // Don't need to render all the way to the edge since the album art
                height,
                radii,
            ),
        );
        let image_width = f64::from(background_image.width);
        self.scene.fill(
            Fill::NonZero,
            Affine::translate((pos_x, 0.0))
                * Affine::scale((uncropped_width - height * 0.25) / image_width),
            &ImageBrush::new(background_image),
            None,
            &Rect::new(0.0, 0.0, image_width, image_width * background_aspect_ratio),
        );
        self.scene.pop_layer();

        // --- ALBUM ART SQUARE ---
        let image_height = f64::from(image.height);
        let transform = Affine::translate((pos_x + width - height, 0.0));
        self.scene.push_clip_layer(
            transform,
            &RoundedRect::new(
                0.0,
                0.0,
                height,
                height,
                RoundedRectRadii::new(rounding, right_rounding, right_rounding, rounding),
            ),
        );
        self.scene.fill(
            Fill::NonZero,
            transform * Affine::scale(height / image_height),
            &ImageBrush::new(image.clone()),
            None,
            &Rect::new(0.0, 0.0, image_height, image_height),
        );
        self.scene.pop_layer();

        // --- TEXT ---
        // Clipping mask to the edge of the background rectangle, shrunk by a margin
        self.scene.push_clip_layer(
            Affine::translate((pos_x, 0.0)),
            &RoundedRect::new(4.0, 4.0, width - height - 4.0, height - 4.0, rounding),
        );
        // Get available width for text
        let text_start_left = pos_x + dark_width + 12.0;
        let text_start_right = pos_x + width - height - 8.0;
        let available_width = (text_start_right - text_start_left).max(0.0);

        // Render the songs title (strip anything beyond a - or ( in the song title)
        let song_name = track.title[..track
            .title
            .find(" (")
            .or_else(|| track.title.find(" -"))
            .unwrap_or(track.title.len())]
            .trim();
        let font_size = 13.0;
        let font_weight = FontWeight::Bold;
        let text_height = (height * 0.25).floor();
        let brush = Color::from_rgb8(240, 240, 240);
        let layout = self.layout_text(song_name, font_size, font_weight);
        let width_ratio = available_width / f64::from(layout.width);
        if width_ratio <= 1.0 {
            let layout = self.layout_text(song_name, font_size * width_ratio.max(0.8), font_weight);
            self.draw_text(
                layout,
                text_start_left,
                text_height,
                Align::Start,
                Align::Center,
                // Fade out when it gets too small, 0.6-0.4
                brush.with_alpha(((width_ratio - 0.4) / 0.2) as f32),
            );
        } else {
            self.draw_text(
                layout,
                text_start_right,
                text_height,
                Align::End,
                Align::Center,
                brush,
            );
        }

        // Get text layouts for bottom row of text
        let font_size = 10.5;
        let font_weight = FontWeight::Bold;
        let text_height = (height * 0.57).floor();

        let artist_text = &track.artist_name;
        let artist_layout = self.layout_text(artist_text, font_size, font_weight);
        let dot_text = "\u{2004}•\u{2004}"; // Use thin spaces on either side of the bullet point
        let dot_layout = self.layout_text(dot_text, font_size, font_weight);
        let time_text = if seconds_until_start >= 60.0 {
            format!(
                "{}m{}s",
                (seconds_until_start / 60.0).floor(),
                (seconds_until_start % 60.0).floor()
            )
        } else {
            format!("{}s", seconds_until_start.round())
        };
        let time_layout = self.layout_text(&time_text, font_size, font_weight);

        let width_ratio =
            available_width / f64::from(artist_layout.width + dot_layout.width + time_layout.width);
        if width_ratio <= 1.0 || !is_current {
            let layout = self.layout_text(
                &format!("{time_text}{dot_text}{artist_text}"),
                font_size * width_ratio.clamp(0.8, 1.0),
                font_weight,
            );
            self.draw_text(
                layout,
                if width_ratio > 1.0 {
                    text_start_right
                } else {
                    text_start_left
                },
                text_height,
                if width_ratio > 1.0 {
                    Align::End
                } else {
                    Align::Start
                },
                Align::Center,
                // Fade out when it gets too small, 0.6-0.4
                brush.with_alpha(((width_ratio - 0.4) / 0.2) as f32),
            );
        } else {
            self.draw_text(
                time_layout,
                pos_x + dark_width + 12.0,
                text_height,
                Align::Start,
                Align::Center,
                brush,
            );
            self.draw_text(
                artist_layout,
                text_start_right,
                text_height,
                Align::End,
                Align::Center,
                brush,
            );
        }

        // Release clipping mask
        self.scene.pop_layer();

        // --- Add a dark overlay for the dark_width ---
        if dark_width > 0.0 {
            self.scene.push_clip_layer(
                Affine::translate((pos_x, 0.0)),
                &RoundedRect::new(0.0, 0.0, width, height, radii),
            );
            self.scene.fill(
                Fill::NonZero,
                Affine::translate((pos_x, 0.0)),
                Color::from_rgba8(0, 0, 0, 140),
                None,
                &Rect::new(0.0, 0.0, dark_width, height),
            );
            self.scene.pop_layer();
        }

        if !is_past {
            self.draw_playlist_buttons(track, is_current, playlists, width, height, pos_x);
        }
    }

    /// Creates the text layout for a single-line string.
    fn layout_text(&self, text: &str, font_size: f64, weight: FontWeight) -> TextLayout {
        let mut face = self.font.base_face.clone();
        if let Some(index) = self.font.weight_axis_index {
            let axis = &self.font.axes[index];
            let _ = face.set_variation(axis.tag, weight.value_for(axis));
        }

        let metrics = FontMetrics::from_face(&face);
        let font_size_px = (font_size * self.scale_factor) as f32;
        let scale = font_size_px / metrics.units_per_em;
        let baseline = metrics.ascender * scale;
        let fallback_height_units = self
            .font
            .base_metrics
            .line_height_units()
            .max(self.font.base_metrics.units_per_em);
        let height_units = {
            let units = metrics.line_height_units();
            if units > 0.0 {
                units
            } else {
                fallback_height_units
            }
        }
        .max(fallback_height_units);
        let height = height_units * scale;
        let space_units = if metrics.space_advance > 0.0 {
            metrics.space_advance
        } else {
            self.font.base_metrics.space_advance
        };

        let mut pen_x = 0.0f32;
        let mut glyphs = Vec::with_capacity(text.len());
        let mut previous: Option<GlyphId> = None;

        for ch in text.chars() {
            let glyph_id = face.glyph_index(ch);
            if let (Some(left), Some(right)) = (previous, glyph_id) {
                pen_x += FontEngine::kerning_units(&face, left, right) * scale;
            }
            let advance_units = glyph_id
                .and_then(|gid| face.glyph_hor_advance(gid))
                .map_or(space_units, f32::from);
            if let Some(gid) = glyph_id {
                glyphs.push(Glyph {
                    id: u32::from(gid.0),
                    x: pen_x,
                    y: baseline,
                });
                previous = Some(gid);
            } else {
                previous = None;
            }
            pen_x += advance_units * scale;
        }

        TextLayout {
            glyphs,
            width: pen_x,
            height,
            font_size: font_size_px,
            coords: self
                .font
                .axes
                .iter()
                .map(|axis| axis_normalized_value(axis, weight.value_for(axis)))
                .collect(),
        }
    }

    /// Draw the text layout onto the scene.
    fn draw_text(
        &mut self,
        layout: TextLayout,
        pos_x: f64,
        pos_y: f64,
        horizontal_align: Align,
        vertical_align: Align,
        brush: Color,
    ) {
        self.scene
            .draw_glyphs(&self.font.font_data)
            .font_size(layout.font_size)
            .normalized_coords(&layout.coords)
            .transform(Affine::translate((
                pos_x
                    - match horizontal_align {
                        Align::Start => 0.0,
                        Align::End => f64::from(layout.width),
                        Align::Center => f64::from(layout.width) * 0.5,
                    },
                pos_y
                    - match vertical_align {
                        Align::Start => 0.0,
                        Align::End => f64::from(layout.height),
                        Align::Center => f64::from(layout.height) * 0.5,
                    },
            )))
            .hint(true)
            .brush(brush)
            .draw(Fill::NonZero, layout.glyphs.into_iter());
    }

    fn render_playing_particles(&mut self, track: &Track, x: f64, height: f64, is_playing: bool) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_particle_update).as_secs_f32();
        self.last_particle_update = now;

        let scale = self.scale_factor as f32;
        let height_f32 = height as f32;

        let lightness_boost = 50.0;
        let Some(track_data) = TRACK_DATA_CACHE.get(&track.id) else {
            return;
        };
        let primary_colors: Vec<_> = track_data
            .primary_colors
            .iter()
            .map(|[r, g, b, _]| {
                [
                    (lerp(0.3, f32::from(*r), 255.0) + lightness_boost).min(255.0) as u8,
                    (lerp(0.3, f32::from(*g), 210.0) + lightness_boost).min(255.0) as u8,
                    (lerp(0.3, f32::from(*b), 160.0) + lightness_boost).min(255.0) as u8,
                ]
            })
            .collect();
        if primary_colors.is_empty() {
            return;
        }

        // Emit new particles while playing
        if is_playing {
            self.particle_spawn_accumulator += dt * SPARK_EMISSION;
            let emit_count = self.particle_spawn_accumulator.floor() as u16;
            self.particle_spawn_accumulator -= f32::from(emit_count);
            for _ in 0..emit_count {
                let position = [x as f32, height_f32 * self.rng.random_range(SPARK_SPAWN_Y)];
                let velocity = [
                    self.rng.random_range(SPARK_VELOCITY_X) * scale,
                    self.rng.random_range(SPARK_VELOCITY_Y) * scale,
                ];
                let life = self.rng.random_range(SPARK_LIFETIME);
                if let Some(dead_particle) = self
                    .now_playing_particles
                    .iter_mut()
                    .find(|particle| !particle.alive)
                {
                    dead_particle.alive = true;
                    dead_particle.position = position;
                    dead_particle.velocity = velocity;
                    dead_particle.life = life;
                    dead_particle.color = self.rng.random_range(0..primary_colors.len());
                } else {
                    self.now_playing_particles.push(NowPlayingParticle {
                        alive: true,
                        position,
                        velocity,
                        color: self.rng.random_range(0..primary_colors.len()),
                        life,
                    });
                }
            }
        } else {
            self.particle_spawn_accumulator = 0.0;
        }

        // Delete dead particles, and update positions of others
        for particle in &mut self.now_playing_particles {
            if !particle.alive {
                continue;
            }

            particle.life -= dt;
            if particle.life <= 0.0 {
                particle.alive = false;
                continue;
            }

            particle.velocity[1] += SPARK_GRAVITY * scale * dt;
            particle.position[0] += particle.velocity[0] * dt;
            particle.position[1] += particle.velocity[1] * dt;
        }

        // Line at the now playing position to denote the cutoff
        let line_width = 4.0 * self.scale_factor;
        self.scene.fill(
            Fill::NonZero,
            Affine::translate((x - line_width * 0.5, 0.0)),
            Color::from_rgb8(255, 224, 210),
            None,
            &RoundedRect::new(0.0, 0.0, line_width, height, 100.0),
        );

        // Render out the particles
        for particle in &self.now_playing_particles {
            let fade = (particle.life / 0.6).clamp(0.0, 1.0);
            let length = lerp_range(SPARK_LENGTH_RANGE, f64::from(fade)) * self.scale_factor;
            let thickness = lerp_range(SPARK_THICKNESS_RANGE, f64::from(fade)) * self.scale_factor;
            let rgb = primary_colors
                .get(particle.color)
                .unwrap_or(&[255, 210, 160]);
            let angle = f64::from(particle.velocity[1].atan2(particle.velocity[0]));
            let opacity = (fade.powf(1.1) * 235.0).round().clamp(0.0, 255.0) as u8;
            self.scene.fill(
                Fill::NonZero,
                Affine::translate((
                    f64::from(particle.position[0]),
                    f64::from(particle.position[1]),
                )) * Affine::rotate(angle)
                    * Affine::translate((-length * 0.5, 0.0)),
                Color::from_rgba8(rgb[0], rgb[1], rgb[2], opacity),
                None,
                &RoundedRect::new(
                    0.0,
                    -thickness * 0.5,
                    length,
                    thickness * 0.5,
                    thickness * 0.5,
                ),
            );
        }
    }
}

fn lerp_range(range: Range<f64>, t: f64) -> f64 {
    range.start + (range.end - range.start) * t.clamp(0.0, 1.0)
}

fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
    (1.0 - t) * v0 + t * v1
}

pub struct NowPlayingParticle {
    alive: bool,
    position: [f32; 2],
    velocity: [f32; 2],
    color: usize,
    life: f32,
}
