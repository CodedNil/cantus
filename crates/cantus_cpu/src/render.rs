use crate::{
    CantusApp, CondensedPlaylist, NUM_SWATCHES, PANEL_START, TRACK_SPACING_MS, Track, art::ArtState,
};
use cantus_shared::{BackgroundPill, MAX_PILL_PLAYLIST_ICONS};
use glam::{Vec2, vec2};
use std::{mem, ops::Range, time::Instant};

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

/// Particles emitted per second when playback is active.
const SPARK_EMISSION: f32 = 20.0;
/// Horizontal velocity range applied at spawn.
const SPARK_VELOCITY_X: Range<usize> = 40..60;
/// Vertical velocity range applied at spawn.
const SPARK_VELOCITY_Y: f32 = 5.0;
/// Lifetime range for individual particles, in seconds.
const SPARK_LIFETIME: Range<f32> = 1.2..1.5;

const PLAYHEAD_START_DURATION: f32 = 0.7;
const PLAYHEAD_TRANSITION_SPEED: f32 = 5.5;

pub struct RenderState {
    pub last_update: Instant,
    pub track_offset: f32,
    pub hovered_track: Option<usize>,
    pub recent_speeds: [f32; 8],
    pub speed_idx: usize,
}
impl Default for RenderState {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            track_offset: 0.0,
            hovered_track: None,
            recent_speeds: [0.0; 8],
            speed_idx: 0,
        }
    }
}

fn album_palette(track: &Track) -> [u32; NUM_SWATCHES] {
    match &track.art {
        ArtState::Ready(art) => art.palette,
        _ => [0; NUM_SWATCHES],
    }
}

/// Build the scene for rendering.
impl CantusApp {
    pub fn create_scene(&mut self) {
        let now = Instant::now();
        let dt = now
            .duration_since(self.render_state.last_update)
            .as_secs_f32()
            .min(0.1);
        self.render_state.last_update = now;

        let history_width = self.config.history_width;
        let total_width = self.config.timeline_width();
        let total_height = self.config.height;
        let timeline_duration_ms = self.config.timeline_duration_ms();
        let timeline_start_ms = self.config.timeline_start_ms();
        let timeline_end_ms = timeline_start_ms + timeline_duration_ms;

        let px_per_ms = self.config.px_per_ms();
        let playhead_x = self.config.playhead_x();

        let mut playback_state = mem::take(&mut self.playback_state);
        if playback_state.queue.is_empty() {
            self.background_pills.clear();
            self.playback_state = playback_state;
            return;
        }

        for track in &mut playback_state.queue {
            track.runtime.width = 0.0;
        }

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

        let mut current_ms = -playback_elapsed
            - playback_state.queue[cur_idx].runtime.queue_offset_ms
            + drag_offset_ms;
        let diff = current_ms - self.render_state.track_offset;
        self.global_uniforms.expansion_xy.x += diff * px_per_ms * dt;
        if !self.global_uniforms.expansion_xy.x.is_finite() {
            self.global_uniforms.expansion_xy.x = playhead_x;
        }
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
        let compact_gap = TRACK_SPACING_MS * px_per_ms;
        let mut compact_count = 0;
        let mut transition_t = 0.0f32;
        for track in &mut playback_state.queue {
            let ms = current_ms + track.runtime.queue_offset_ms;
            let dur = track.duration_ms as f32;
            if ms > timeline_end_ms {
                break;
            }
            let natural_start_x = playhead_x + ms * px_per_ms;
            let natural_end_x = natural_start_x + dur * px_per_ms;
            let runtime = &mut track.runtime;
            runtime.start_ms = ms;
            if natural_end_x >= history_width + total_height {
                let start_x = natural_start_x.max(history_width);
                let end_x = natural_end_x.min(history_width + total_width);
                runtime.start_x = start_x;
                runtime.width = end_x - start_x;
                runtime.art_only = false;
            } else if natural_end_x >= history_width {
                transition_t = (history_width + total_height - natural_end_x) / total_height;
                runtime.start_x = natural_end_x - total_height;
                runtime.width = total_height;
                runtime.art_only = true;
            } else {
                compact_count += 1;
                runtime.art_only = true;
            }
        }

        for (index, track) in playback_state.queue[..compact_count].iter_mut().enumerate() {
            let slot = compact_count - index - 1;
            let right = history_width - compact_gap - (slot as f32 + transition_t) * compact_stride;
            track.runtime.start_x = right - total_height;
            track.runtime.width = total_height;
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

        let hovered_track = if !self.interaction.dragging && self.interaction.mouse_pressure > 0.0 {
            self.hovered_track(&playback_state.queue, &playback_state.playlists)
        } else {
            None
        };
        self.render_state.hovered_track = hovered_track;

        // Render the tracks
        self.background_pills.clear();
        let current_track = playback_state.queue.iter().position(|track| {
            playhead_x >= track.runtime.start_x
                && playhead_x <= track.runtime.start_x + track.runtime.width
        });
        let playlists = &playback_state.playlists;

        for (queue_index, track) in playback_state.queue.iter_mut().enumerate() {
            if hovered_track == Some(queue_index) {
                continue;
            }
            let runtime = &track.runtime;
            if runtime.width <= 0.0 || runtime.start_x + runtime.width <= 0.0 {
                continue;
            }
            self.draw_track(track, playhead_x, false, dt, px_per_ms, playlists);
        }

        if let Some(queue_index) = hovered_track {
            let track = &mut playback_state.queue[queue_index];
            if track.runtime.width > 0.0 && track.runtime.start_x + track.runtime.width > 0.0 {
                self.draw_track(track, playhead_x, true, dt, px_per_ms, playlists);
            }
        }

        // Draw the particles
        self.render_playhead_particles(
            dt,
            &playback_state.queue[current_track.unwrap_or(cur_idx)],
            playhead_x,
            avg_speed,
            playback_state.playing,
        );
        self.playback_state = playback_state;
    }

    fn hovered_track(&self, queue: &[Track], playlists: &[CondensedPlaylist]) -> Option<usize> {
        let mouse_pos = self.global_uniforms.mouse_pos;
        let in_track = |track: &Track| {
            track
                .runtime
                .rect(self.config.height)
                .is_some_and(|rect| rect.contains(mouse_pos))
                || self
                    .icon_row_rects(track, playlists)
                    .into_iter()
                    .flatten()
                    .any(|rect| rect.contains(mouse_pos))
        };

        if let Some(index) = self.render_state.hovered_track
            && queue.get(index).is_some_and(in_track)
        {
            return Some(index);
        }

        queue
            .iter()
            .enumerate()
            .rev()
            .find(|(_, track)| in_track(track))
            .map(|(index, _)| index)
    }

    fn draw_track(
        &mut self,
        track: &mut Track,
        origin_x: f32,
        hovered: bool,
        dt: f32,
        px_per_ms: f32,
        playlists: &[CondensedPlaylist],
    ) {
        let width = track.runtime.width;
        let start_x = track.runtime.start_x;
        // If dragging, set the drag target to this track, and the position within the track
        if self.interaction.dragging
            && track.runtime.start_ms <= 0.0
            && track.runtime.start_ms + track.duration_ms as f32 >= 0.0
        {
            let (hit_start, hit_end) = track.natural_x_range(origin_x, px_per_ms);
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

        let image_index = self.get_image_index(&track.art);
        let colors = album_palette(track);
        let show_details = !track.runtime.art_only;
        let mut playlist_expansion = track.runtime.playlist_expansion;
        move_towards(
            &mut playlist_expansion,
            if hovered && show_details { 1.0 } else { 0.0 },
            dt.min(0.1) * 6.0,
        );
        track.runtime.playlist_expansion = playlist_expansion;
        let mut pill = BackgroundPill {
            x: start_x,
            width,
            colors,
            alpha: fade_alpha,
            image_index,
            rating: -1,
            primary_playlist_count: 0,
            secondary_playlist_count: 0,
            secondary_expansion: 0.0,
            playlist_images: [-1; MAX_PILL_PLAYLIST_ICONS],
        };

        // --- TEXT ---
        if show_details
            && fade_alpha >= 1.0
            && width > self.config.height
            && let Some(gpu) = &mut self.gpu_resources
        {
            gpu.text_renderer
                .render(&gpu.queue, track, self.render_scale);
        }

        // Expand the hitbox vertically so it includes the playlist buttons
        if show_details
            && self.draw_playlist_buttons(track, playlist_expansion, playlists, &mut pill)
            && pill.secondary_playlist_count > 0
        {
            pill.secondary_expansion = playlist_expansion;
        }
        self.background_pills.push(pill);
    }

    fn render_playhead_particles(
        &mut self,
        dt: f32,
        track: &Track,
        playhead_x: f32,
        avg_speed: f32,
        playing: bool,
    ) {
        let palette = album_palette(track);

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
            .filter(|particle| time > particle.end_time)
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
        let playbutton_hsize = self.config.height * 0.25;
        let speed = PLAYHEAD_TRANSITION_SPEED * dt;
        let play_hitbox = Rect::new(
            playhead_x - playbutton_hsize,
            PANEL_START,
            playhead_x + playbutton_hsize,
            PANEL_START + self.config.height,
        );
        // Get playhead states
        let playhead_hovered = play_hitbox.contains(self.global_uniforms.mouse_pos)
            && interaction.mouse_pressure > 0.0;
        let last_toggle =
            self.last_toggle_playing.elapsed().as_secs_f32() / PLAYHEAD_START_DURATION;

        let play_intro_active = !playhead_hovered && playing && last_toggle < 1.0;
        if play_intro_active {
            self.playhead_info.bar_split = 1.0 - last_toggle;
            self.playhead_info.icon_presence = 1.0 - last_toggle;
            move_towards(&mut self.playhead_info.icon_morph, 1.0, speed * 1.5);
            self.playhead_info.icon_scale = 1.0 + last_toggle;
        } else {
            let show_icon = u32::from(playhead_hovered || !playing) as f32;
            let play_icon = u32::from(playhead_hovered && !playing) as f32;
            move_towards(&mut self.playhead_info.bar_split, show_icon, speed);
            if show_icon > self.playhead_info.icon_presence {
                self.playhead_info.icon_presence = show_icon;
            } else {
                move_towards(&mut self.playhead_info.icon_presence, show_icon, speed);
            }
            move_towards(&mut self.playhead_info.icon_morph, play_icon, speed);
            move_towards(&mut self.playhead_info.icon_scale, 1.0, speed);
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
