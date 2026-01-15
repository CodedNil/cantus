use crate::{
    CantusApp, PANEL_EXTENSION, PANEL_START,
    config::CONFIG,
    spotify::{ALBUM_PALETTE_CACHE, CondensedPlaylist, PLAYBACK_STATE, PlaylistId, Track},
};
use bytemuck::{Pod, Zeroable};
use std::{collections::HashMap, ops::Range, sync::LazyLock, time::Instant};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

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

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x0 && p.x <= self.x1 && p.y >= self.y0 && p.y <= self.y1
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct ScreenUniforms {
    screen_size: [f32; 2], // Full size of the layer shell
    bar_height: [f32; 2],  // Start y and bars height
    mouse_pos: [f32; 2],
    playhead_x: f32, // x position where the playhead line is drawn
    time: f32,
    expansion_xy: [f32; 2],
    expansion_time: f32,
    scale_factor: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct PlayheadUniforms {
    volume: f32,
    bar_lerp: f32,
    play_lerp: f32,
    pause_lerp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct Particle {
    spawn_vel: [f32; 2],
    spawn_y: f32,
    spawn_time: f32,
    duration: f32,
    color: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct BackgroundPill {
    rect: [f32; 2], // pos x, width
    colors: [u32; 4],
    alpha: f32,
    image_index: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct IconInstance {
    pub pos: [f32; 2],
    // Packed 2 u16s
    // First is alpha 0-1
    // Second is 0 for dimmed icon 1 for bright icon, 2 for empty star, 3 for half star, 4 for filled star
    pub data: u32,
    pub image_index: i32,
}

static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

/// Spacing between tracks in ms
const TRACK_SPACING_MS: f32 = 4000.0;
/// Particles emitted per second when playback is active.
const SPARK_EMISSION: f32 = 60.0;
/// Horizontal velocity range applied at spawn.
const SPARK_VELOCITY_X: Range<usize> = 75..100;
/// Vertical velocity range applied at spawn.
const SPARK_VELOCITY_Y: Range<usize> = 20..55;
/// Lifetime range for individual particles, in seconds.
const SPARK_LIFETIME: Range<f32> = 0.4..0.6;

/// Duration for animation events
const ANIMATION_DURATION: f32 = 2.0;

pub struct RenderState {
    pub last_update: Instant,
    pub track_offset: f32,
    pub recent_speeds: [f32; 16],
    pub speed_idx: usize,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            track_offset: 0.0,
            recent_speeds: [0.0; 16],
            speed_idx: 0,
        }
    }
}

pub struct TrackRender<'a> {
    pub track: &'a Track,
    pub is_current: bool,
    pub seconds_until_start: f32,
    pub start_x: f32,
    pub width: f32,
    pub hitbox_range: (f32, f32),
    pub art_only: bool,
}

/// Build the scene for rendering.
impl CantusApp {
    pub fn create_scene(&mut self) {
        let now = Instant::now();
        let dt = now
            .duration_since(self.render_state.last_update)
            .as_secs_f32();
        self.render_state.last_update = now;

        self.background_pills.clear();
        let history_width = CONFIG.history_width;
        let total_width = CONFIG.width - history_width - 10.0;
        let total_height = CONFIG.height;
        let timeline_duration_ms = CONFIG.timeline_future_minutes * 60_000.0;
        let timeline_start_ms = -CONFIG.timeline_past_minutes * 60_000.0;

        let px_per_ms = total_width / timeline_duration_ms;
        let origin_x = history_width - timeline_start_ms * px_per_ms;

        let playback_state = PLAYBACK_STATE.read();
        if playback_state.queue.is_empty() {
            return;
        }

        self.interaction.icon_hitboxes.clear();
        self.interaction.track_hitboxes.clear();

        let drag_offset_ms = if self.interaction.dragging {
            self.interaction.drag_delta_pixels / px_per_ms
        } else {
            0.0
        };
        let cur_idx = playback_state
            .queue_index
            .min(playback_state.queue.len() - 1);

        if playback_state.playing != self.interaction.playing {
            self.interaction.playing = playback_state.playing;
            self.interaction.last_expansion = (
                Instant::now(),
                Point::new(origin_x, PANEL_START + CONFIG.height * 0.5),
            );
            self.interaction.last_toggle_playing = Instant::now();
        }
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
        if !self.interaction.dragging && diff.abs() > 200.0 {
            current_ms = self.render_state.track_offset + diff * 0.1;
        }

        // Add the new move speed to the array move_speeds, trim the previous ones
        let frame_move_speed = (current_ms - self.render_state.track_offset) * dt;
        self.render_state.track_offset = current_ms;
        let s_idx = self.render_state.speed_idx;
        self.render_state.recent_speeds[s_idx] = frame_move_speed;
        self.render_state.speed_idx = (s_idx + 1) % 16;
        let avg_speed = self.render_state.recent_speeds.iter().sum::<f32>() / 16.0;

        // Iterate over the tracks within the timeline.
        let mut track_renders = Vec::with_capacity(playback_state.queue.len());
        let mut cur_ms = current_ms;
        for track in &playback_state.queue {
            let start = cur_ms;
            let end = start + track.duration_ms as f32;
            cur_ms = end + TRACK_SPACING_MS;
            if start > timeline_start_ms + timeline_duration_ms {
                break;
            }

            let v_start = start.max(timeline_start_ms) * px_per_ms;
            let v_end = end.min(timeline_start_ms + timeline_duration_ms) * px_per_ms;
            track_renders.push(TrackRender {
                track,
                is_current: start <= 0.0 && end >= 0.0,
                seconds_until_start: (start / 1000.0).abs(),
                start_x: (v_start - timeline_start_ms * px_per_ms) + history_width,
                width: v_end - v_start,
                hitbox_range: (
                    (start - timeline_start_ms) * px_per_ms + history_width,
                    (end - timeline_start_ms) * px_per_ms + history_width,
                ),
                art_only: false,
            });
        }

        // Sort out past tracks so they get a fixed width and stack
        let mut current_px = 0.0;
        let mut first_found = false;
        let track_spacing = TRACK_SPACING_MS * px_per_ms;
        for track_render in track_renders.iter_mut().rev() {
            // If the end of the track (minus album width) is before the cropping zone
            let distance_before =
                history_width - (track_render.start_x + track_render.width - total_height);
            if track_render.start_x + track_render.width - total_height <= history_width {
                track_render.width = total_height;
                track_render.start_x = current_px;
                track_render.art_only = true;
                current_px -= 30.0;
                if !first_found {
                    first_found = true;
                    // Smooth out the snapping
                    current_px = history_width
                        - total_height
                        - track_spacing
                        - (distance_before - (total_height - track_spacing * 2.0)).clamp(0.0, 30.0);
                }
            } else {
                // Set the start of the track, this will be the closest to the left track before they start being cropped
                current_px = track_render.start_x - total_height - track_spacing;
            }
        }

        // Render the tracks
        for track_render in &track_renders {
            if track_render.width <= 0.0 || track_render.start_x + track_render.width <= 0.0 {
                continue;
            }
            self.draw_track(track_render, origin_x, &playback_state.playlists);
        }

        // Draw the particles
        self.render_playhead_particles(
            dt,
            &playback_state.queue[cur_idx],
            origin_x,
            avg_speed,
            playback_state.volume,
        );
    }

    fn draw_track(
        &mut self,
        track_render: &TrackRender,
        origin_x: f32,
        playlists: &HashMap<PlaylistId, CondensedPlaylist>,
    ) {
        let width = track_render.width;
        let track = track_render.track;
        let start_x = track_render.start_x;
        let hitbox = Rect::new(
            start_x,
            PANEL_START,
            start_x + width,
            PANEL_START + CONFIG.height,
        );

        // Add hitbox
        let (hit_start, hit_end) = track_render.hitbox_range;
        let full_width = hit_end - hit_start;
        self.interaction
            .track_hitboxes
            .push((track.id, hitbox, track_render.hitbox_range));
        // If dragging, set the drag target to this track, and the position within the track
        if self.interaction.dragging && track_render.is_current {
            self.interaction.drag_track = Some((
                track.id,
                (start_x + (origin_x - start_x).max(0.0) - hit_start) / full_width,
            ));
        }

        // --- BACKGROUND ---
        let fade_alpha = if width < CONFIG.height {
            ((width / CONFIG.height) - 0.9).max(0.0) * 10.0
        } else {
            1.0
        };

        let image_index = self.get_image_index(&track_render.track.album.image);
        self.background_pills.push(BackgroundPill {
            rect: [start_x, width],
            colors: ALBUM_PALETTE_CACHE
                .get(&track.album.id)
                .and_then(|data_ref| data_ref.as_ref().copied())
                .unwrap_or_default(),
            alpha: fade_alpha,
            image_index,
        });

        // --- TEXT ---
        if let Some(text_renderer) = &mut self.text_renderer
            && !track_render.art_only
            && fade_alpha >= 1.0
            && width > CONFIG.height
        {
            text_renderer.render(track_render);
        }

        // Expand the hitbox vertically so it includes the playlist buttons
        if !track_render.art_only {
            let hovered = !self.interaction.dragging
                && self.interaction.mouse_position.x >= hitbox.x0
                && self.interaction.mouse_position.x <= hitbox.x1;
            self.draw_playlist_buttons(track, hovered, playlists, width, start_x);
        }
    }

    fn render_playhead_particles(
        &mut self,
        dt: f32,
        track: &Track,
        playhead_x: f32,
        avg_speed: f32,
        volume: Option<u8>,
    ) {
        let palette = ALBUM_PALETTE_CACHE
            .get(&track.album.id)
            .and_then(|data_ref| data_ref.as_ref().copied())
            .unwrap_or_default();

        let time = START_TIME.elapsed().as_secs_f32();

        // Get expansion animation variables
        let (interaction_inst, interaction_point) = self.interaction.last_expansion;
        let (expansion_xy, expansion_time) = (
            [interaction_point.x, PANEL_START + interaction_point.y],
            interaction_inst.duration_since(*START_TIME).as_secs_f32(),
        );

        self.screen_uniforms = ScreenUniforms {
            screen_size: [CONFIG.width, CONFIG.height + PANEL_START + PANEL_EXTENSION],
            bar_height: [PANEL_START, CONFIG.height],
            mouse_pos: [
                self.interaction.mouse_position.x,
                self.interaction.mouse_position.y,
            ],
            playhead_x,
            time,
            expansion_xy,
            expansion_time,
            scale_factor: self.scale_factor,
        };

        // Emit new particles while playing
        let mut emit_count = if avg_speed.abs() > 0.00001 {
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

        for (i, particle) in self.particles.iter_mut().enumerate() {
            if emit_count > 0 && time > particle.spawn_time + particle.duration {
                // Calculate a position based on golden ratio recurrence.
                let seed = (i as f32 + time * SPARK_EMISSION) * 0.618_034;
                let y_fraction = 0.1 + (seed.fract() * 0.85); // Map to 0.1..0.95 range

                particle.spawn_y = PANEL_START + CONFIG.height * y_fraction;
                particle.spawn_vel = [
                    fastrand::usize(SPARK_VELOCITY_X) as f32 * horizontal_bias,
                    -(fastrand::usize(SPARK_VELOCITY_Y) as f32),
                ];
                particle.color = palette[fastrand::usize(0..palette.len())];
                particle.spawn_time = time;
                particle.duration =
                    lerpf32(fastrand::f32(), SPARK_LIFETIME.start, SPARK_LIFETIME.end);
                emit_count -= 1;
            }
        }

        // Playhead
        let interaction = &mut self.interaction;
        let playbutton_hsize = CONFIG.height * 0.25;
        let speed = 2.2 * dt;
        interaction.play_hitbox = Rect::new(
            playhead_x - playbutton_hsize,
            PANEL_START,
            playhead_x + playbutton_hsize,
            PANEL_START + CONFIG.height,
        );
        // Get playhead states
        let playhead_hovered = interaction.play_hitbox.contains(interaction.mouse_position);
        let last_toggle =
            interaction.last_toggle_playing.elapsed().as_secs_f32() / ANIMATION_DURATION;

        // Determine the intended state for the bar
        let bar_target =
            u32::from(playhead_hovered || !interaction.playing || last_toggle < 1.0) as f32;
        move_towards(&mut interaction.playhead_bar, bar_target, speed);

        // Determine which icon (if any) is currently active
        let (mut play_active, mut pause_active) = (false, false);
        if playhead_hovered {
            if interaction.playing {
                pause_active = true;
            } else {
                play_active = true;
            }
        } else if !interaction.playing {
            pause_active = true;
        } else if interaction.playing && last_toggle < 1.0 {
            interaction.playhead_play = last_toggle; // Hard set for the "start" animation
            play_active = true;
        }

        // If active, move toward 0.5. If inactive, finish the animation to 1.0 then reset to 0.0.
        for (val, is_active) in [
            (&mut interaction.playhead_play, play_active),
            (&mut interaction.playhead_pause, pause_active),
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

        self.playhead_info = PlayheadUniforms {
            volume: f32::from(volume.unwrap_or(100)) / 100.0,
            bar_lerp: interaction.playhead_bar,
            play_lerp: interaction.playhead_play,
            pause_lerp: interaction.playhead_pause,
        };
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

fn lerpf32(t: f32, v0: f32, v1: f32) -> f32 {
    v0 + t * (v1 - v0)
}
