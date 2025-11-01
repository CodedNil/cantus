use crate::{
    CantusLayer, PANEL_HEIGHT_BASE, PANEL_WIDTH,
    background::WarpBackground,
    spotify::{IMAGES_CACHE, PLAYBACK_STATE, RATING_PLAYLISTS, TRACK_DATA_CACHE, Track},
};
use parley::{
    Alignment, FontFamily, FontStack, FontWeight, Layout, layout::PositionedLayoutItem,
    style::StyleProperty,
};
use rand::Rng;
use rspotify::model::TrackId;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    sync::LazyLock,
    time::Instant,
};
use vello::{
    Glyph,
    kurbo::{Affine, Rect, RoundedRect, RoundedRectRadii},
    peniko::{Color, Fill, ImageBrush, ImageData},
};
use vello_svg::usvg;

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
const SPARK_LENGTH_RANGE: Range<f64> = 6.0..10.0;
/// Rendered spark thickness range, in logical pixels.
const SPARK_THICKNESS_RANGE: Range<f64> = 2.0..4.0;

/// Star images
static STAR_IMAGES: LazyLock<[usvg::Tree; 3]> = LazyLock::new(|| {
    let full_svg = include_str!("../assets/star.svg");
    let half_svg = include_str!("../assets/star-half.svg");
    let options = usvg::Options::default();

    let full_gray_svg = full_svg.replace("fill=\"none\"", "fill=\"#808080\"");
    let full_yellow_svg = full_svg.replace("fill=\"none\"", "fill=\"#FFD700\"");
    let half_yellow_svg = half_svg.replace("fill=\"none\"", "fill=\"#FFD700\"");

    [
        usvg::Tree::from_data(full_gray_svg.as_bytes(), &options).unwrap(),
        usvg::Tree::from_data(full_yellow_svg.as_bytes(), &options).unwrap(),
        usvg::Tree::from_data(half_yellow_svg.as_bytes(), &options).unwrap(),
    ]
});

/// Build the scene for rendering.
impl CantusLayer {
    pub fn create_scene(&mut self, device_id: usize) {
        let total_width = (PANEL_WIDTH * self.scale_factor).ceil();
        let total_height = (PANEL_HEIGHT_BASE * self.scale_factor).ceil();

        let playback_state = PLAYBACK_STATE.lock().clone();
        let queue = &playback_state.queue;
        if queue.is_empty() {
            return;
        }

        let timeline_end_ms = TIMELINE_START_MS + TIMELINE_DURATION_MS;
        let px_per_ms = total_width / TIMELINE_DURATION_MS;
        let current_index = playback_state.queue_index.min(queue.len() - 1);

        // Get playlists data as a HashMap
        let playlists: HashMap<String, HashSet<TrackId<'static>>> = playback_state
            .playlists
            .into_iter()
            .map(|p| (p.name.clone(), p.tracks))
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
        if let (Some(shader), Some(renderer)) = (
            self.shader_backgrounds.get_mut(&device_id),
            self.renderers.get_mut(&device_id),
        ) {
            shader.purge_stale(renderer, self.frame_index);
        }
    }

    fn draw_track(
        &mut self,
        device_id: usize,
        track: &Track,
        is_current: bool,
        track_start_ms: f64,
        track_end_ms: f64,
        timeline_end_ms: f64,
        px_per_ms: f64,
        height: f64,
        seconds_until_start: f64,
        playlists: &HashMap<String, HashSet<TrackId<'static>>>,
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
        let background_image = self.render_background(
            device_id,
            &track.image_url,
            &track_data.palette_image.clone(),
        );

        // --- BACKGROUND ---
        self.scene.push_clip_layer(
            Affine::translate((pos_x, 0.0)),
            &RoundedRect::new(
                0.0,
                0.0,
                width - height * 0.5, // Don't need to render all the way to the edge since the album art
                height,
                radii,
            ),
        );
        let image_width = f64::from(background_image.width);
        let image_height = f64::from(background_image.height);
        self.scene.fill(
            Fill::NonZero,
            Affine::translate((pos_x, 0.0))
                * Affine::scale((uncropped_width - height * 0.5) / image_width),
            &ImageBrush::new(background_image),
            None,
            &Rect::new(0.0, 0.0, image_width, image_height),
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

        // --- Star ratings and favourite playlists ---
        if is_current {
            let track_ratings: Vec<f32> = RATING_PLAYLISTS
                .iter()
                .filter_map(|&rating_key| {
                    let is_rated = playlists
                        .get(rating_key)
                        .is_some_and(|playlist| playlist.contains(&track.id));
                    if is_rated {
                        rating_key.parse::<f32>().ok()
                    } else {
                        None
                    }
                })
                .collect();
            let max_rating = track_ratings.iter().copied().fold(0.0f32, f32::max);
            let full_stars = max_rating.floor() as usize;
            let has_half = max_rating.fract() >= 0.5;

            let star_size = 12.0 * self.scale_factor;
            let star_spacing = 2.0 * self.scale_factor;
            let stars_y = height * 0.9 - star_size / 2.0; // Center vertically at bottom

            let svg_base_size = f64::from(STAR_IMAGES[0].size().width());

            for i in 0..5 {
                let star_x = text_start_left + (i as f64 * (star_size + star_spacing));
                let transform =
                    Affine::translate((star_x, stars_y)) * Affine::scale(star_size / svg_base_size);

                let is_full = i < full_stars;
                let is_half = (i == full_stars) && has_half;

                if is_full {
                    self.scene
                        .append(&vello_svg::render_tree(&STAR_IMAGES[1]), Some(transform));
                } else {
                    self.scene
                        .append(&vello_svg::render_tree(&STAR_IMAGES[0]), Some(transform));
                }
                if is_half {
                    self.scene
                        .append(&vello_svg::render_tree(&STAR_IMAGES[2]), Some(transform));
                }
            }
        }

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
    }

    fn render_background(
        &mut self,
        device_id: usize,
        key: &str,
        palette_image: &ImageData,
    ) -> ImageData {
        let device_handle = &self.render_context.devices[device_id];
        let shader = self
            .shader_backgrounds
            .entry(device_id)
            .or_insert_with(|| WarpBackground::new(&device_handle.device));
        let renderer = self.renderers.get_mut(&device_id).unwrap();
        shader.render(
            key,
            device_handle,
            renderer,
            palette_image,
            self.time_origin.elapsed().as_secs_f32(),
            self.frame_index,
        )
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
        let offset_x = match horizontal_align {
            Alignment::Right => f64::from(layout.width()),
            Alignment::Center => f64::from(layout.width()) * 0.5,
            Alignment::Start | Alignment::End | Alignment::Left | Alignment::Justify => 0.0,
        };
        let offset_y = match vertical_align {
            Alignment::End => f64::from(layout.height()),
            Alignment::Center => f64::from(layout.height()) * 0.5,
            Alignment::Start | Alignment::Left | Alignment::Right | Alignment::Justify => 0.0,
        };
        let text_transform = Affine::translate((pos_x - offset_x, pos_y - offset_y));

        for glyph_run in
            layout
                .lines()
                .flat_map(|line| line.items())
                .filter_map(|item| match item {
                    PositionedLayoutItem::GlyphRun(run) => Some(run),
                    PositionedLayoutItem::InlineBox(_) => None,
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
