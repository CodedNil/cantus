use crate::{
    AppCaches, CantusApp, CondensedPlaylist, NUM_SWATCHES, PANEL_START, PlaylistId, Track,
};
pub use cantus_shared::{BackgroundPill, GlobalUniforms, IconInstance, Particle, PlayheadUniforms};
use glam::{Vec2, vec2, vec4};
use image::RgbaImage;
use itertools::Itertools;
use palette::IntoColor;
use std::{collections::HashMap, ops::Range, time::Instant};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl Rect {
    pub const fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self { x0, y0, x1, y1 }
    }

    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.x0 && p.x <= self.x1 && p.y >= self.y0 && p.y <= self.y1
    }
}

/// Spacing between tracks in ms
const TRACK_SPACING_MS: f32 = 4000.0;
/// Particles emitted per second when playback is active.
const SPARK_EMISSION: f32 = 20.0;
/// Horizontal velocity range applied at spawn.
const SPARK_VELOCITY_X: Range<usize> = 40..60;
/// Vertical velocity range applied at spawn.
const SPARK_VELOCITY_Y: f32 = 5.0;
/// Lifetime range for individual particles, in seconds.
const SPARK_LIFETIME: Range<f32> = 1.2..1.5;

/// Duration for animation events
const ANIMATION_DURATION: f32 = 2.0;

pub struct RenderState {
    pub last_update: Instant,
    pub track_offset: f32,
    pub recent_speeds: [f32; 8],
    pub speed_idx: usize,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            track_offset: 0.0,
            recent_speeds: [0.0; 8],
            speed_idx: 0,
        }
    }
}

pub struct TrackRender<'a> {
    pub queue_index: usize,
    pub track: &'a Track,
    pub is_current: bool,
    pub seconds_until_start: f32,
    pub start_x: f32,
    pub width: f32,
    pub hitbox_range: (f32, f32),
    pub art_only: bool,
}

fn album_palette(caches: &AppCaches, track: &Track) -> [u32; NUM_SWATCHES] {
    track
        .album
        .id
        .and_then(|id| caches.album_palettes.get(&id))
        .and_then(|r| r.as_ref().copied())
        .unwrap_or_default()
}

/// Build the scene for rendering.
impl CantusApp {
    pub fn create_scene(&mut self) {
        let now = Instant::now();
        let dt = now
            .duration_since(self.render_state.last_update)
            .as_secs_f32();
        self.render_state.last_update = now;

        let history_width = self.config.history_width;
        let total_width = self.config.width - history_width - 16.0;
        let total_height = self.config.height;
        let timeline_duration_ms = self.config.timeline_future_minutes * 60_000.0;
        let timeline_start_ms = -self.config.timeline_past_minutes * 60_000.0;
        let timeline_end_ms = timeline_start_ms + timeline_duration_ms;

        let px_per_ms = total_width / timeline_duration_ms;
        let playhead_x = history_width - timeline_start_ms * px_per_ms;

        let playback_state = std::mem::take(&mut self.playback_state);
        if playback_state.queue.is_empty() {
            self.background_pills.clear();
            self.playback_state = playback_state;
            return;
        }

        self.interaction.icon_hitboxes.clear();
        self.interaction.track_hitboxes.clear();

        let drag_offset_ms = if let Some(origin_pos) = self.interaction.drag_origin {
            (self.global_uniforms.mouse_pos.x - origin_pos.x) / px_per_ms
        } else {
            0.0
        };
        let cur_idx = playback_state
            .queue_index
            .min(playback_state.queue.len() - 1);

        if self.interaction.dragging {
            self.interaction.drag_track = None;
        }

        // Lerp the progress based on when the data was last updated, get the start time of the current track
        let playback_elapsed = playback_state.progress as f32
            + if playback_state.playing {
                playback_state.last_progress_update.elapsed().as_millis() as f32
            } else {
                0.0
            };

        // Lerp track start based on the target and current start time
        let past_tracks_duration: f32 = playback_state
            .queue
            .iter()
            .take(cur_idx)
            .map(|t| t.duration_ms as f32)
            .sum();

        let mut current_ms = -playback_elapsed - past_tracks_duration + drag_offset_ms
            - TRACK_SPACING_MS * cur_idx as f32;
        let diff = current_ms - self.render_state.track_offset;
        self.global_uniforms.expansion_xy.x += diff * px_per_ms * dt;
        if !self.interaction.dragging && diff.abs() > 200.0 {
            current_ms = self.render_state.track_offset + diff * 3.5 * dt;
        }

        // Add the new move speed to the array move_speeds, trim the previous ones
        let frame_move_speed = (current_ms - self.render_state.track_offset) * dt;
        self.render_state.track_offset = current_ms;
        let s_idx = self.render_state.speed_idx;
        self.render_state.recent_speeds[s_idx] = frame_move_speed;
        self.render_state.speed_idx = (s_idx + 1) % 8;
        let avg_speed = self.render_state.recent_speeds.iter().sum::<f32>() / 8.0;

        // Each past track is clipped at history_width. Historical ones are stacked
        let compact_stride = total_height * 0.55;
        let mut track_renders: Vec<TrackRender> = Vec::with_capacity(playback_state.queue.len());
        let mut compact_stack: Vec<(usize, &Track, f32, f32, f32, f32)> = Vec::new();
        let mut transition_t = 0.0f32;
        let mut ms = current_ms;

        for (queue_index, track) in playback_state.queue.iter().enumerate() {
            let dur = track.duration_ms as f32;
            let end_ms = ms + dur;
            if ms > timeline_end_ms {
                break;
            }
            let natural_start_x = playhead_x + ms * px_per_ms;
            let natural_end_x = natural_start_x + dur * px_per_ms;

            if natural_end_x >= history_width + total_height {
                let start_x = natural_start_x.max(history_width);
                let end_x = natural_end_x.min(history_width + total_width);
                track_renders.push(TrackRender {
                    queue_index,
                    track,
                    is_current: ms <= 0.0 && end_ms >= 0.0,
                    seconds_until_start: (ms / 1000.0).abs(),
                    start_x,
                    width: end_x - start_x,
                    hitbox_range: (natural_start_x, natural_end_x),
                    art_only: false,
                });
            } else if natural_end_x >= history_width {
                transition_t = (history_width + total_height - natural_end_x) / total_height;
                track_renders.push(TrackRender {
                    queue_index,
                    track,
                    is_current: ms <= 0.0 && end_ms >= 0.0,
                    seconds_until_start: (ms / 1000.0).abs(),
                    start_x: natural_end_x - total_height,
                    width: total_height,
                    hitbox_range: (natural_start_x, natural_end_x),
                    art_only: true,
                });
            } else {
                compact_stack.push((
                    queue_index,
                    track,
                    ms,
                    end_ms,
                    natural_start_x,
                    natural_end_x,
                ));
            }
            ms = end_ms + TRACK_SPACING_MS;
        }

        for (slot, (queue_index, track, start_ms, end_ms, natural_start_x, natural_end_x)) in
            compact_stack.iter().rev().enumerate()
        {
            let right = history_width - (slot as f32 + transition_t) * compact_stride;
            track_renders.push(TrackRender {
                queue_index: *queue_index,
                track,
                is_current: *start_ms <= 0.0 && *end_ms >= 0.0,
                seconds_until_start: (*start_ms / 1000.0).abs(),
                start_x: right - total_height,
                width: total_height,
                hitbox_range: (*natural_start_x, *natural_end_x),
                art_only: true,
            });
        }

        // Screen uniforms
        self.global_uniforms.time = self.start_time.elapsed().as_secs_f32();
        let (screen_width, screen_height) = self.logical_surface_size();
        self.global_uniforms.screen_size = vec2(screen_width, screen_height);
        self.global_uniforms.bar_height = vec2(PANEL_START, self.config.height);
        self.global_uniforms.playhead_x = playhead_x;

        // Mouse uniforms
        move_towards(
            &mut self.global_uniforms.mouse_pressure,
            self.interaction.mouse_pressure,
            5.0 * dt,
        );

        track_renders.sort_unstable_by(|a, b| a.start_x.total_cmp(&b.start_x));

        let hovered_track = if !self.interaction.dragging && self.interaction.mouse_pressure > 0.0 {
            let mouse_x = self.global_uniforms.mouse_pos.x;
            track_renders
                .iter()
                .rev()
                .find(|render| {
                    !render.art_only
                        && mouse_x >= render.start_x
                        && mouse_x <= render.start_x + render.width
                })
                .map(|render| render.queue_index)
        } else {
            None
        };
        // Render the tracks
        let mut current_track = None;
        let mut background_count = 0;
        for track_render in &track_renders {
            if track_render.width <= 0.0 || track_render.start_x + track_render.width <= 0.0 {
                continue;
            }
            self.draw_track(
                track_render,
                playhead_x,
                background_count,
                hovered_track == Some(track_render.queue_index),
                dt,
                &playback_state.playlists,
            );
            background_count += 1;
            if playhead_x >= track_render.start_x
                && playhead_x <= track_render.start_x + track_render.width
            {
                current_track = Some(track_render.track);
            }
        }
        self.background_pills.truncate(background_count);

        // Draw the particles
        self.render_playhead_particles(
            dt,
            current_track.unwrap_or(&playback_state.queue[cur_idx]),
            playhead_x,
            avg_speed,
            playback_state.volume,
            playback_state.playing,
        );
        self.playback_state = playback_state;
    }

    fn draw_track(
        &mut self,
        track_render: &TrackRender,
        origin_x: f32,
        background_index: usize,
        hovered: bool,
        dt: f32,
        playlists: &HashMap<PlaylistId, CondensedPlaylist>,
    ) {
        let track = track_render.track;
        let width = track_render.width;
        let start_x = track_render.start_x;
        let hitbox = Rect::new(
            start_x,
            PANEL_START,
            start_x + width,
            PANEL_START + self.config.height,
        );

        let (hit_start, hit_end) = track_render.hitbox_range;
        self.interaction
            .track_hitboxes
            .push((track.id, hitbox, track_render.hitbox_range));
        // If dragging, set the drag target to this track, and the position within the track
        if self.interaction.dragging && track_render.is_current {
            self.interaction.drag_track = Some((
                track.id,
                (origin_x.max(start_x) - hit_start) / (hit_end - hit_start),
            ));
        }

        // --- BACKGROUND ---
        let fade_alpha = if width < self.config.height {
            ((width / self.config.height) - 0.9).max(0.0) * 10.0
        } else {
            1.0
        };

        let image_index = track
            .album
            .image
            .as_deref()
            .map(|path| self.get_image_index(path))
            .unwrap_or_default();
        let colors = album_palette(&self.caches, track);
        if background_index == self.background_pills.len() {
            self.background_pills.push(BackgroundPill::default());
        }
        let key = (track_render.queue_index + 1) as f32;
        if self.background_pills[background_index].icon_span.w as usize
            != track_render.queue_index + 1
        {
            self.background_pills[background_index] = BackgroundPill::default();
        }
        let mut playlist_expansion = self.background_pills[background_index].icon_span.z;
        move_towards(
            &mut playlist_expansion,
            if hovered && !track_render.art_only {
                1.0
            } else {
                0.0
            },
            dt.min(0.1) * 6.0,
        );
        let pill = &mut self.background_pills[background_index];
        pill.rect = vec2(start_x, width);
        pill.icon_span = vec4(0.0, 0.0, playlist_expansion, key);
        pill.color0 = colors[0];
        pill.color1 = colors[1];
        pill.color2 = colors[2];
        pill.color3 = colors[3];
        pill.alpha = fade_alpha;
        pill.image_index = image_index;

        // --- TEXT ---
        if let Some(text_renderer) = &mut self.text_renderer
            && !track_render.art_only
            && fade_alpha >= 1.0
            && width > self.config.height
        {
            text_renderer.render(track_render);
        }

        // Expand the hitbox vertically so it includes the playlist buttons
        if !track_render.art_only
            && let Some(icon_rows) =
                self.draw_playlist_buttons(track_render, hovered, playlist_expansion, playlists)
        {
            self.background_pills[background_index].icon_span = icon_rows;
            self.background_pills[background_index].icon_span.w += key;
        }
    }

    fn render_playhead_particles(
        &mut self,
        dt: f32,
        track: &Track,
        playhead_x: f32,
        avg_speed: f32,
        volume: Option<u8>,
        playing: bool,
    ) {
        let palette = album_palette(&self.caches, track);

        // Emit new particles while playing
        let emit_count = if avg_speed.abs() > 0.00001 {
            self.particles_accumulator += dt * SPARK_EMISSION;
            let count = self.particles_accumulator.floor() as u8;
            self.particles_accumulator -= f32::from(count);
            count
        } else {
            self.particles_accumulator = 0.0;
            0
        };

        // Cache active particle Y positions to avoid borrow checker conflicts
        let spawn_offset = avg_speed.signum() * 2.0;
        let horizontal_bias = (avg_speed.abs().powf(0.2) * spawn_offset * 0.5).clamp(-3.0, 3.0);
        let time = self.global_uniforms.time;

        for particle in self
            .particles
            .iter_mut()
            .filter(|p| time > p.end_time)
            .take(emit_count as usize)
        {
            let y_fraction = fastrand::f32();

            particle.spawn_pos = vec2(
                playhead_x,
                PANEL_START + self.config.height * (0.1 + (y_fraction * 0.85)), // Map to 0.1..0.95 range
            );
            particle.spawn_vel = vec2(
                fastrand::usize(SPARK_VELOCITY_X) as f32 * horizontal_bias,
                (y_fraction - 0.5) * 2.0 * SPARK_VELOCITY_Y,
            );
            let duration = lerpf32(fastrand::f32(), SPARK_LIFETIME.start, SPARK_LIFETIME.end);
            let packed_duration = (duration * 100.0).min(255.0) as u8;
            let base_color = palette[fastrand::usize(0..palette.len())];
            particle.color = (base_color & 0x00FF_FFFF) | (u32::from(packed_duration) << 24);
            particle.end_time = time + duration;
        }

        // Playhead
        let interaction = &mut self.interaction;
        self.playhead_info.volume = f32::from(volume.unwrap_or(100)) / 100.0;
        let playbutton_hsize = self.config.height * 0.25;
        let speed = 2.2 * dt;
        let play_hitbox = Rect::new(
            playhead_x - playbutton_hsize,
            PANEL_START,
            playhead_x + playbutton_hsize,
            PANEL_START + self.config.height,
        );
        // Get playhead states
        let playhead_hovered = play_hitbox.contains(self.global_uniforms.mouse_pos)
            && interaction.mouse_pressure > 0.0;
        let last_toggle = self.last_toggle_playing.elapsed().as_secs_f32() / ANIMATION_DURATION;

        // Determine the intended state for the bar
        let bar_target = u32::from(playhead_hovered || !playing || last_toggle < 1.0) as f32;
        move_towards(&mut self.playhead_info.bar_lerp, bar_target, speed);

        // Determine which icon (if any) is currently active
        let (mut play_active, mut pause_active) = (false, false);
        if playhead_hovered {
            if playing {
                pause_active = true;
            } else {
                play_active = true;
            }
        } else if !playing {
            pause_active = true;
        } else if last_toggle < 1.0 {
            self.playhead_info.play_lerp = last_toggle; // Hard set for the "start" animation
            play_active = true;
        }

        for (val, is_active) in [
            (&mut self.playhead_info.play_lerp, play_active),
            (&mut self.playhead_info.pause_lerp, pause_active),
        ] {
            if is_active {
                move_towards(val, 0.5, speed);
            } else if *val > 0.0 {
                move_towards(val, 1.0, speed);
                if *val >= 1.0 {
                    *val = 0.0;
                }
            }
        }
    }
}

fn move_towards(current: &mut f32, target: f32, speed: f32) {
    let delta = target - *current;
    if delta.abs() <= speed {
        *current = target;
    } else {
        *current += delta.signum() * speed;
    }
}

pub fn lerpf32(t: f32, v0: f32, v1: f32) -> f32 {
    v0 + t * (v1 - v0)
}

fn extract_lab_pixels(img: &RgbaImage) -> (Vec<palette::Lab>, bool) {
    let saturation_threshold = 30u8;
    let srgb_to_lab = |p: &image::Rgba<u8>| {
        palette::FromColor::from_color(palette::Srgb::new(
            f32::from(p[0]) / 255.0,
            f32::from(p[1]) / 255.0,
            f32::from(p[2]) / 255.0,
        ))
    };

    let colourful: Vec<palette::Lab> = img
        .pixels()
        .filter(|p| {
            let max = p[0].max(p[1]).max(p[2]);
            let min = p[0].min(p[1]).min(p[2]);
            (max - min) > saturation_threshold
        })
        .map(srgb_to_lab)
        .collect();

    if colourful.is_empty() {
        (img.pixels().map(srgb_to_lab).collect(), false)
    } else {
        (colourful, true)
    }
}

fn load_image_pixels(caches: &AppCaches, url: &str) -> Option<(Vec<palette::Lab>, bool)> {
    let img = caches.images.get(url)?.as_ref()?.clone();
    Some(extract_lab_pixels(&img))
}

fn lab_pixels_to_palette(pixels: &[palette::Lab]) -> [u32; NUM_SWATCHES] {
    kmeans_colors::get_kmeans_hamerly(NUM_SWATCHES, 20, 5.0, false, pixels, 0)
        .centroids
        .iter()
        .take(NUM_SWATCHES)
        .map(|c: &palette::Lab| {
            let rgb: palette::Srgb = (*c).into_color();
            u32::from_le_bytes([
                (rgb.red * 255.0) as u8,
                (rgb.green * 255.0) as u8,
                (rgb.blue * 255.0) as u8,
                255,
            ])
        })
        .collect_vec()
        .try_into()
        .unwrap_or([0; NUM_SWATCHES])
}

/// Gathers the 4 primary colours for each album image.
pub fn update_color_palettes(caches: &AppCaches, queue: &[Track]) {
    for track in queue {
        let album_id = track.album.id.unwrap_or_default();
        let artist_id = track.artist.id.unwrap_or_default();
        if caches.album_palettes.contains_key(&album_id) {
            continue;
        }
        let Some((pixels, is_colourful)) = track
            .album
            .image
            .as_ref()
            .and_then(|u| load_image_pixels(caches, u))
        else {
            continue;
        };
        caches.album_palettes.insert(album_id, None);

        let pixels = if is_colourful {
            pixels
        } else {
            let Some(url) = caches
                .artist_images
                .get(&artist_id)
                .and_then(|e| e.value().clone())
            else {
                caches.album_palettes.remove(&album_id);
                continue;
            };
            match load_image_pixels(caches, &url) {
                Some((p, true)) => p,
                _ => pixels,
            }
        };

        caches
            .album_palettes
            .insert(album_id, Some(lab_pixels_to_palette(&pixels)));
    }
}
