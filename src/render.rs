use crate::{
    CantusLayer, PANEL_HEIGHT_BASE, PANEL_WIDTH,
    background::WarpBackground,
    spotify::{IMAGES_CACHE, PLAYBACK_STATE, Track},
};
use parley::{
    Alignment, FontFamily, FontStack, FontWeight, Layout, layout::PositionedLayoutItem,
    style::StyleProperty,
};
use rand::Rng;
use std::{ops::Range, time::Instant};
use vello::{
    Glyph,
    kurbo::{Affine, Rect, RoundedRect, RoundedRectRadii},
    peniko::{Color, Fill, ImageBrush},
};

/// Spacing between tracks in ms
const TRACK_SPACING_MS: f64 = 4000.0;
/// How many ms to show in the timeline
const TIMELINE_DURATION_MS: f64 = 15.0 * 60.0 * 1000.0;
/// Starting position of the timeline in ms, if negative then it shows the history too
const TIMELINE_START_MS: f64 = -60.0 * 1000.0;

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
const SPARK_LENGTH_RANGE: Range<f64> = 4.0..10.0;
/// Rendered spark thickness range, in logical pixels.
const SPARK_THICKNESS_RANGE: Range<f64> = 1.8..3.0;

/// Build the scene for rendering.
impl CantusLayer {
    pub fn create_scene(&mut self, id: usize) {
        let total_width = (PANEL_WIDTH * self.scale_factor).ceil();
        let total_height = (PANEL_HEIGHT_BASE * self.scale_factor).ceil();

        // Get current playback state
        let playback_state = PLAYBACK_STATE.lock().clone();

        // Ensure the background provider exists
        if !self.shader_backgrounds.contains_key(&id) {
            self.shader_backgrounds.insert(
                id,
                WarpBackground::new(&self.render_context.devices[id].device),
            );
        }

        let timeline_end_ms = TIMELINE_START_MS + TIMELINE_DURATION_MS;
        let px_per_ms = total_width / TIMELINE_DURATION_MS;

        // Track positions are relative to "now" (0 ms), negative values are in the past.
        let lerped_progress = u32::from(playback_state.playing)
            * playback_state.last_updated.elapsed().as_millis() as u32;
        let mut track_start_ms = -f64::from(playback_state.progress + lerped_progress);
        let mut track_spacing = 0.0;
        for i in 0..playback_state.queue_index {
            track_start_ms -= f64::from(playback_state.queue[i].milliseconds);
            track_spacing -= TRACK_SPACING_MS;
        }

        // Lerp the track_start_ms with the global state
        if (track_start_ms - self.track_start_ms).abs() > 200.0 {
            track_start_ms = self.track_start_ms + (track_start_ms - self.track_start_ms) * 0.1;
        }
        self.track_start_ms = track_start_ms;
        if (track_spacing - self.track_spacing).abs() > 200.0 {
            track_spacing = self.track_spacing + (track_spacing - self.track_spacing) * 0.1;
        }
        self.track_spacing = track_spacing;

        // Iterate over the currently playing track followed by the queued tracks.
        for (index, track) in playback_state.queue.iter().enumerate() {
            let track_start_ms_spaced = track_start_ms + track_spacing;
            if track_start_ms_spaced >= timeline_end_ms {
                break;
            }

            let track_end_ms = track_start_ms_spaced + f64::from(track.milliseconds);

            let visible_start_ms = track_start_ms_spaced.max(TIMELINE_START_MS);
            let start_trimmed = track_start_ms_spaced > TIMELINE_START_MS;
            let visible_end_ms = track_end_ms.min(timeline_end_ms);
            let end_trimmed = track_end_ms < timeline_end_ms;

            let pos_x = (visible_start_ms - TIMELINE_START_MS) * px_per_ms;
            let width = (visible_end_ms - visible_start_ms) * px_per_ms;
            // How much of the width is to the left of the current position
            let dark_width = if track_start_ms_spaced < 0.0 {
                track_start_ms_spaced.max(TIMELINE_START_MS) * -px_per_ms
            } else {
                0.0
            };

            // Draw the track, trimming to the visible window if it spills off either side.
            self.draw_track(
                id,
                track,
                index == playback_state.queue_index,
                pos_x,
                width,
                dark_width,
                (track_end_ms - track_start_ms) * px_per_ms,
                total_height,
                (track_start_ms / 1000.0).abs(),
                start_trimmed,
                end_trimmed,
            );

            track_start_ms += f64::from(track.milliseconds);
            track_spacing += TRACK_SPACING_MS;
        }

        // Draw the particles
        self.render_playing_particles(
            -TIMELINE_START_MS * px_per_ms,
            total_height,
            playback_state.playing,
        );

        // Purge the stale background cache entries.
        self.shader_backgrounds
            .get_mut(&id)
            .unwrap()
            .purge_stale(self.renderers.get_mut(&id).unwrap(), self.frame_index);
    }

    fn draw_track(
        &mut self,
        id: usize,
        track: &Track,
        is_current: bool,
        pos_x: f64,
        width: f64,
        dark_width: f64,
        uncropped_width: f64,
        height: f64,
        seconds_until_start: f64,
        start_trimmed: bool,
        end_trimmed: bool,
    ) {
        self.track_hitboxes.insert(
            track.id.clone(),
            Rect::new(
                pos_x / self.scale_factor,
                0.0,
                (pos_x + width) / self.scale_factor,
                height / self.scale_factor,
            ),
        );

        let rounding = ROUNDING_RADIUS * self.scale_factor;
        let left_rounding = rounding * if start_trimmed { 1.0 } else { 0.3 };
        let right_rounding = rounding * if end_trimmed { 1.0 } else { 0.3 };

        let Some(image) = IMAGES_CACHE.get(&track.image.url) else {
            return;
        };

        // Make sure the background image shader is ready
        let surface = self.render_surface.as_ref().unwrap();
        let Some(background_image) = self.shader_backgrounds.get_mut(&id).unwrap().render(
            &track.image.url,
            &self.render_context.devices[id],
            self.renderers.get_mut(&id).unwrap(),
            surface.config.width,
            surface.config.height,
            &image.blurred,
            self.time_origin.elapsed().as_secs_f32(),
            self.frame_index,
        ) else {
            return;
        };

        // --- BACKGROUND ---
        self.scene.push_clip_layer(
            Affine::translate((pos_x, 0.0)),
            &RoundedRect::new(
                0.0,
                0.0,
                width - height * 0.5, // Don't need to render all the way to the edge since the album art
                height,
                RoundedRectRadii::new(left_rounding, right_rounding, right_rounding, left_rounding),
            ),
        );
        let image_height = f64::from(background_image.height);
        self.scene.fill(
            Fill::NonZero,
            Affine::translate((pos_x, image_height * -0.5))
                * Affine::scale(uncropped_width / image_height),
            &ImageBrush::new(background_image),
            None,
            &Rect::new(0.0, 0.0, image_height, image_height),
        );
        self.scene.pop_layer();

        // --- ALBUM ART SQUARE ---
        let image_height = f64::from(image.original.height);
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
            &ImageBrush::new(image.original.clone()),
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
        let song_name = track.name[..track
            .name
            .find(" (")
            .or_else(|| track.name.find(" -"))
            .unwrap_or(track.name.len())]
            .trim();
        let font_size = 13.0;
        let font_weight = FontWeight::BOLD;
        let text_height = (height * 0.25).floor();
        let brush = Color::from_rgb8(240, 240, 240);
        let layout = self.layout_text(song_name, font_size, font_weight);
        let width_ratio = available_width / f64::from(layout.width());
        if width_ratio <= 1.0 {
            let layout = self.layout_text(song_name, font_size * width_ratio.max(0.8), font_weight);
            self.draw_text(
                &layout,
                text_start_left,
                text_height,
                Alignment::Left,
                Alignment::Center,
                // Fade out when it gets too small, 0.6-0.4
                brush.with_alpha(((width_ratio - 0.4) / 0.2) as f32),
            );
        } else {
            self.draw_text(
                &layout,
                text_start_right,
                text_height,
                Alignment::Right,
                Alignment::Center,
                brush,
            );
        }

        // Get text layouts for bottom row of text
        let font_size = 11.0;
        let font_weight = FontWeight::BOLD;
        let text_height = (height * 0.65).floor();

        let artist_text = track.artists.first().unwrap();
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

        let width_ratio = available_width
            / f64::from(artist_layout.width() + dot_layout.width() + time_layout.width());
        if width_ratio <= 1.0 || !is_current {
            let layout = self.layout_text(
                &format!("{time_text}{dot_text}{artist_text}"),
                font_size * width_ratio.clamp(0.8, 1.0),
                font_weight,
            );
            self.draw_text(
                &layout,
                if width_ratio > 1.0 {
                    text_start_right
                } else {
                    text_start_left
                },
                text_height,
                if width_ratio > 1.0 {
                    Alignment::Right
                } else {
                    Alignment::Left
                },
                Alignment::Center,
                // Fade out when it gets too small, 0.6-0.4
                brush.with_alpha(((width_ratio - 0.4) / 0.2) as f32),
            );
        } else {
            self.draw_text(
                &time_layout,
                pos_x + dark_width + 12.0,
                text_height,
                Alignment::Left,
                Alignment::Center,
                brush,
            );
            self.draw_text(
                &artist_layout,
                text_start_right,
                text_height,
                Alignment::Right,
                Alignment::Center,
                brush,
            );
        }

        // Release clipping mask
        self.scene.pop_layer();
    }

    /// Creates the text layout based on font properties.
    fn layout_text(&mut self, text: &str, font_size: f64, font_weight: FontWeight) -> Layout<()> {
        let mut builder =
            self.layout_context
                .ranged_builder(&mut self.font_context, text, 1.0, false);
        builder.push_default(StyleProperty::FontStack(FontStack::Single(
            FontFamily::Named("Noto Sans".into()),
        )));
        builder.push_default(StyleProperty::FontSize(
            (font_size * self.scale_factor) as f32,
        ));
        builder.push_default(StyleProperty::FontWeight(font_weight));

        let mut layout: Layout<()> = builder.build(text);
        layout.break_all_lines(None);
        layout
    }

    /// Draw the text layout onto the scene.
    fn draw_text(
        &mut self,
        layout: &Layout<()>,
        pos_x: f64,
        pos_y: f64,
        horizontal_align: Alignment,
        vertical_align: Alignment,
        brush: Color,
    ) {
        let text_transform = Affine::translate((
            pos_x
                - f64::from(if horizontal_align == Alignment::Right {
                    layout.width()
                } else if horizontal_align == Alignment::Center {
                    layout.width() * 0.5
                } else {
                    0.0
                }),
            pos_y
                - f64::from(if vertical_align == Alignment::End {
                    layout.height()
                } else if vertical_align == Alignment::Center {
                    layout.height() * 0.5
                } else {
                    0.0
                }),
        ));

        for glyph_run in layout
            .lines()
            .flat_map(|line| line.items())
            .filter_map(|item| {
                if let PositionedLayoutItem::GlyphRun(run) = item {
                    Some(run)
                } else {
                    None
                }
            })
        {
            let glyphs = glyph_run.positioned_glyphs().map(|g| Glyph {
                id: g.id,
                x: g.x,
                y: g.y,
            });
            let run = glyph_run.run();
            self.scene
                .draw_glyphs(run.font())
                .font_size(run.font_size())
                .normalized_coords(run.normalized_coords())
                .transform(text_transform)
                .hint(true)
                .brush(brush)
                .draw(Fill::NonZero, glyphs);
        }
    }

    fn render_playing_particles(&mut self, x: f64, height: f64, is_playing: bool) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_particle_update).as_secs_f32();
        self.last_particle_update = now;

        let scale = self.scale_factor as f32;
        let height_f32 = height as f32;

        // Emit new particles while playing
        if is_playing {
            self.particle_spawn_accumulator += dt * SPARK_EMISSION;
            let emit_count = self.particle_spawn_accumulator.floor() as u16;
            self.particle_spawn_accumulator -= f32::from(emit_count);
            for _ in 0..emit_count {
                // Try to take over an existing dead particle before inserting a new one
                let position = [x as f32, height_f32 * self.rng.random_range(SPARK_SPAWN_Y)];
                let velocity = [
                    self.rng.random_range(SPARK_VELOCITY_X) * scale,
                    self.rng.random_range(SPARK_VELOCITY_Y) * scale,
                ];
                let life = self.rng.random_range(SPARK_LIFETIME);
                if let Some(dead_particle) =
                    self.now_playing_particles.iter_mut().find(|p| !p.alive)
                {
                    dead_particle.alive = true;
                    dead_particle.position = position;
                    dead_particle.velocity = velocity;
                    dead_particle.life = life;
                } else {
                    self.now_playing_particles.push(NowPlayingParticle {
                        alive: true,
                        position,
                        velocity,
                        life,
                    });
                }
            }
        } else {
            self.particle_spawn_accumulator = 0.0;
        }

        // Delete dead particles, and update positions of others
        self.now_playing_particles.iter_mut().for_each(|particle| {
            if particle.alive {
                particle.life -= dt;
                if particle.life <= 0.0 {
                    particle.alive = false;
                }

                particle.velocity[1] += SPARK_GRAVITY * scale * dt;
                particle.position[0] += particle.velocity[0] * dt;
                particle.position[1] += particle.velocity[1] * dt;
            }
        });

        // Line at the now playing position to denote the cutoff
        let line_width = 4.0 * self.scale_factor;
        self.scene.fill(
            Fill::NonZero,
            Affine::translate((x - line_width * 0.5, 0.0)),
            Color::from_rgb8(255, 224, 210),
            None,
            &RoundedRect::new(0.0, 0.0, line_width, height, 100.0),
        );

        for particle in &self.now_playing_particles {
            let fade = (particle.life / 0.6).clamp(0.0, 1.0);
            let fade64 = f64::from(fade);
            let length = Self::lerp_range(SPARK_LENGTH_RANGE, fade64) * self.scale_factor;
            let thickness = Self::lerp_range(SPARK_THICKNESS_RANGE, fade64) * self.scale_factor;
            self.scene.fill(
                Fill::NonZero,
                Affine::translate((
                    f64::from(particle.position[0]),
                    f64::from(particle.position[1]),
                )) * Affine::rotate(f64::from(particle.velocity[1].atan2(particle.velocity[0])))
                    * Affine::translate((-length * 0.5, 0.0)),
                Color::from_rgba8(
                    255,
                    210,
                    160,
                    (fade.powf(1.1) * 235.0).round().clamp(0.0, 255.0) as u8,
                ),
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

    #[inline]
    fn lerp_range(range: Range<f64>, t: f64) -> f64 {
        range.start + (range.end - range.start) * t.clamp(0.0, 1.0)
    }
}

pub struct NowPlayingParticle {
    alive: bool,
    position: [f32; 2],
    velocity: [f32; 2],
    life: f32,
}
