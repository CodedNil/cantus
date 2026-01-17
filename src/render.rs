use crate::{
    ALBUM_PALETTE_CACHE, ARTIST_DATA_CACHE, CantusApp, CondensedPlaylist, IMAGES_CACHE,
    NUM_SWATCHES, PANEL_EXTENSION, PANEL_START, PLAYBACK_STATE, PlaylistId, Track, config::CONFIG,
};
use bytemuck::{Pod, Zeroable};
use image::RgbaImage;
use palette::IntoColor;
use std::{collections::HashMap, ops::Range, time::Instant};

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
pub struct GlobalUniforms {
    screen_size: [f32; 2], // x, y, full size of the layer shell
    bar_height: [f32; 2],  // Start y, and bars height
    mouse_pos: [f32; 2],   // x, y
    mouse_pressure: f32,   // 0 - 1 for hovered - 2 for mouse down
    playhead_x: f32,       // x position where the playhead line is drawn
    expansion_xy: [f32; 2],
    expansion_time: f32,
    time: f32,
    scale_factor: f32,
    _padding: [f32; 3],
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
    pub spawn_pos: [f32; 2], // x, y
    pub spawn_vel: [f32; 2], // x, y
    pub end_time: f32,       // The time the particle will be pruned
    pub color: u32,          // r, g, b, duration
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
        let total_width = CONFIG.width - history_width - 16.0;
        let total_height = CONFIG.height;
        let timeline_duration_ms = CONFIG.timeline_future_minutes * 60_000.0;
        let timeline_start_ms = -CONFIG.timeline_past_minutes * 60_000.0;

        let px_per_ms = total_width / timeline_duration_ms;
        let playhead_x = history_width - timeline_start_ms * px_per_ms;

        let playback_state = PLAYBACK_STATE.read();
        if playback_state.queue.is_empty() {
            return;
        }

        self.interaction.icon_hitboxes.clear();
        self.interaction.track_hitboxes.clear();

        let drag_offset_ms = if let Some(origin_pos) = self.interaction.drag_origin {
            (self.interaction.mouse_position.x - origin_pos.x) / px_per_ms
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
                Point::new(playhead_x, PANEL_START + CONFIG.height * 0.5),
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
        self.interaction.last_expansion.1.x += diff * px_per_ms * dt; // Offset the expansion so it moves with the tracks
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

        // Screen uniforms
        self.global_uniforms.time = self.start_time.elapsed().as_secs_f32();
        self.global_uniforms.screen_size =
            [CONFIG.width, CONFIG.height + PANEL_START + PANEL_EXTENSION];
        self.global_uniforms.bar_height = [PANEL_START, CONFIG.height];
        self.global_uniforms.playhead_x = playhead_x;
        self.global_uniforms.scale_factor = self.scale_factor;

        // Mouse uniforms
        self.global_uniforms.mouse_pos = [
            self.interaction.mouse_position.x,
            self.interaction.mouse_position.y,
        ];
        move_towards(
            &mut self.global_uniforms.mouse_pressure,
            self.interaction.mouse_pressure,
            5.0 * dt,
        );

        // Get expansion animation variables
        let (interaction_inst, interaction_point) = self.interaction.last_expansion;
        self.global_uniforms.expansion_xy = [interaction_point.x, interaction_point.y];
        self.global_uniforms.expansion_time = interaction_inst
            .duration_since(self.start_time)
            .as_secs_f32();

        // Render the tracks
        let mut current_track = None;
        for track_render in &track_renders {
            if track_render.width <= 0.0 || track_render.start_x + track_render.width <= 0.0 {
                continue;
            }
            self.draw_track(track_render, playhead_x, &playback_state.playlists);
            if playhead_x >= track_render.start_x
                && playhead_x <= track_render.start_x + track_render.width
            {
                current_track = Some(track_render.track);
            }
        }

        // Draw the particles
        self.render_playhead_particles(
            dt,
            current_track.unwrap_or(&playback_state.queue[cur_idx]),
            playhead_x,
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

        let image_index = track_render
            .track
            .album
            .image
            .as_deref()
            .map(|path| self.get_image_index(path))
            .unwrap_or_default();
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
                && self.interaction.mouse_pressure > 0.0
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
        let time = self.global_uniforms.time;

        for particle in &mut self.particles {
            if emit_count > 0 && time > particle.end_time {
                let y_fraction = fastrand::f32();

                particle.spawn_pos = [
                    playhead_x,
                    PANEL_START + CONFIG.height * (0.1 + (y_fraction * 0.85)), // Map to 0.1..0.95 range
                ];
                particle.spawn_vel = [
                    fastrand::usize(SPARK_VELOCITY_X) as f32 * horizontal_bias,
                    (y_fraction - 0.5) * 2.0 * SPARK_VELOCITY_Y,
                ];
                let duration = lerpf32(fastrand::f32(), SPARK_LIFETIME.start, SPARK_LIFETIME.end);
                let packed_duration = (duration * 100.0).min(255.0) as u8;
                let base_color = palette[fastrand::usize(0..palette.len())];
                particle.color = (base_color & 0x00FF_FFFF) | (u32::from(packed_duration) << 24);
                particle.end_time = time + duration;
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
        let playhead_hovered = interaction.play_hitbox.contains(interaction.mouse_position)
            && interaction.mouse_pressure > 0.0;
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

fn do_kmeans(pixels: &[palette::Lab]) -> Vec<palette::Lab> {
    kmeans_colors::get_kmeans_hamerly(NUM_SWATCHES, 20, 5.0, false, pixels, 0).centroids
}

fn convert_to_swatches(centroids: &[palette::Lab]) -> Vec<[u8; 3]> {
    centroids
        .iter()
        .map(|c: &palette::Lab| {
            let rgb: palette::Srgb = (*c).into_color();
            [
                (rgb.red * 255.0) as u8,
                (rgb.green * 255.0) as u8,
                (rgb.blue * 255.0) as u8,
            ]
        })
        .collect()
}

/// Gathers the 4 primary colours for each album image.
pub fn update_color_palettes() {
    for track in &PLAYBACK_STATE.read().queue {
        if ALBUM_PALETTE_CACHE.contains_key(&track.album.id) {
            continue;
        }

        let Some(image_ref) = track.album.image.as_ref().and_then(|p| IMAGES_CACHE.get(p)) else {
            continue;
        };
        let Some(album_image) = image_ref.as_ref() else {
            continue;
        };
        ALBUM_PALETTE_CACHE.insert(track.album.id, None);

        let (album_pixels, album_is_colourful) = extract_lab_pixels(album_image);
        let mut result = do_kmeans(&album_pixels);

        if !album_is_colourful {
            let artist_img = ARTIST_DATA_CACHE
                .get(&track.artist.id)
                .and_then(|e| e.value().clone())
                .and_then(|url| IMAGES_CACHE.get(&url))
                .and_then(|img| img.as_ref().cloned());

            if let Some(img) = artist_img {
                let (artist_pixels, artist_is_colourful) = extract_lab_pixels(&img);
                if artist_is_colourful {
                    result = do_kmeans(&artist_pixels);
                }
            } else {
                ALBUM_PALETTE_CACHE.remove(&track.album.id);
                continue;
            }
        }

        let primary_colors: [u32; 4] = convert_to_swatches(&result)
            .iter()
            .take(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], 255]))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Result should have exactly 4 colors");
        ALBUM_PALETTE_CACHE.insert(track.album.id, Some(primary_colors));
    }
}
