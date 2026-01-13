use crate::{
    CantusApp, PANEL_EXTENSION, PANEL_START,
    config::CONFIG,
    spotify::{ALBUM_PALETTE_CACHE, CondensedPlaylist, PLAYBACK_STATE, PlaylistId, Track},
    text_render::{ATLAS_MSDF_SCALE, ATLAS_RANGE, MSDFAtlas, TextInstance},
};
use bytemuck::{Pod, Zeroable};
use std::{collections::HashMap, ops::Range, sync::LazyLock, time::Instant};
use ttf_parser::{Face, Tag};

const FONT_SIZE: f32 = 12.0;
const FONT_SIZE_SMALL: f32 = 10.5;

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
    pub alpha: f32,
    pub variant: f32,
    pub param: f32,
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
const SPARK_LIFETIME: Range<f32> = 0.4..0.9;

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

pub struct FontEngine {
    pub face: Face<'static>,
    pub atlas: MSDFAtlas,
}

pub struct TextLayout {
    glyphs: Vec<(u32, f32)>, // gid, x_offset
    width: f32,
    line_height: f32,
    font_size: f32,
}

impl Default for FontEngine {
    fn default() -> Self {
        let bytes = include_bytes!("../assets/NotoSans.ttf");
        let mut face = Face::parse(bytes, 0).expect("failed to parse font");
        if let Some(axis) = face
            .variation_axes()
            .into_iter()
            .find(|a| a.tag == Tag::from_bytes(b"wght"))
        {
            face.set_variation(axis.tag, 700.0f32.clamp(axis.min_value, axis.max_value));
        }
        let atlas = MSDFAtlas::new(&face, 48);
        Self { face, atlas }
    }
}

pub struct ParticlesState {
    pub particles: [Particle; 64],
    pub accumulator: f32,
}

impl Default for ParticlesState {
    fn default() -> Self {
        Self {
            particles: [Particle::default(); 64],
            accumulator: 0.0,
        }
    }
}

pub struct TrackRender<'a> {
    track: &'a Track,
    is_current: bool,
    seconds_until_start: f32,
    start_x: f32,
    width: f32,
    hitbox_range: (f32, f32),
    art_only: bool,
    image_index: i32,
}

/// Build the scene for rendering.
impl CantusApp {
    pub fn create_scene(&mut self, image_map: &HashMap<String, i32>) {
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
                image_index: self.get_image_index(&track.album.image, image_map),
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
            self.draw_track(track_render, origin_x, &playback_state.playlists, image_map);
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
        image_map: &HashMap<String, i32>,
    ) {
        if track_render.width <= 0.0 {
            return;
        }
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

        self.background_pills.push(BackgroundPill {
            rect: [start_x, width],
            colors: ALBUM_PALETTE_CACHE
                .get(&track.album.id)
                .and_then(|data_ref| data_ref.as_ref().copied())
                .unwrap_or_default(),
            alpha: fade_alpha,
            image_index: track_render.image_index,
        });

        // --- TEXT ---
        if !track_render.art_only && fade_alpha >= 1.0 && width > CONFIG.height {
            // Get available width for text
            let text_start_left = start_x + 12.0;
            let text_start_right = start_x + width - CONFIG.height - 8.0;
            let available_width = (text_start_right - text_start_left).max(0.0);
            let text_alpha = (available_width / 100.0).min(1.0);
            let text_color = [0.94, 0.94, 0.94, text_alpha];

            // Render the songs title (strip anything beyond a - or ( in the song title)
            let song_name = track
                .name
                .split(['(', '-'])
                .next()
                .unwrap_or(&track.name)
                .trim();
            let text_height = PANEL_START + (CONFIG.height * 0.2).floor();
            let song_layout = self.layout_text(song_name, FONT_SIZE);
            let width_ratio = available_width / song_layout.width;
            if width_ratio <= 1.0 {
                self.draw_text(
                    &self.layout_text(song_name, FONT_SIZE * width_ratio.max(0.8)),
                    text_start_left,
                    text_height,
                    0.0,
                    text_color,
                );
            } else {
                self.draw_text(&song_layout, text_start_right, text_height, 1.0, text_color);
            }

            // Get text layouts for bottom row of text
            let text_height = PANEL_START + (CONFIG.height * 0.52).floor();

            let artist_text = &track.artist.name;
            let time_text = if track_render.seconds_until_start >= 60.0 {
                format!(
                    "{}m{}s",
                    (track_render.seconds_until_start / 60.0).floor(),
                    (track_render.seconds_until_start % 60.0).floor()
                )
            } else {
                format!("{}s", track_render.seconds_until_start.round())
            };
            let dot_text = "\u{2004}•\u{2004}"; // Use thin spaces on either side of the bullet point

            let bottom_text = format!("{time_text}{dot_text}{artist_text}");
            let mut layout = self.layout_text(&bottom_text, FONT_SIZE_SMALL);
            let width_ratio = available_width / layout.width;
            if width_ratio <= 1.0 || !track_render.is_current {
                if width_ratio < 1.0 {
                    layout = self
                        .layout_text(&bottom_text, FONT_SIZE_SMALL * width_ratio.clamp(0.8, 1.0));
                }
                self.draw_text(
                    &layout,
                    if width_ratio >= 1.0 {
                        text_start_right
                    } else {
                        text_start_left
                    },
                    text_height,
                    if width_ratio >= 1.0 { 1.0 } else { 0.0 },
                    text_color,
                );
            } else {
                self.draw_text(
                    &self.layout_text(&time_text, FONT_SIZE_SMALL),
                    start_x + 12.0,
                    text_height,
                    0.0,
                    text_color,
                );
                self.draw_text(
                    &self.layout_text(artist_text, FONT_SIZE_SMALL),
                    text_start_right,
                    text_height,
                    1.0,
                    text_color,
                );
            }
        }

        // Expand the hitbox vertically so it includes the playlist buttons
        if !track_render.art_only {
            let hovered = !self.interaction.dragging
                && self.interaction.mouse_position.x >= hitbox.x0
                && self.interaction.mouse_position.x <= hitbox.x1;
            self.draw_playlist_buttons(track, hovered, playlists, width, start_x, image_map);
        }
    }

    /// Creates the text layout for a single-line string.
    fn layout_text(&self, text: &str, size: f32) -> TextLayout {
        let face = &self.font.face;
        let scale = size / f32::from(face.units_per_em());
        let mut px = 0.0f32;
        let mut glyphs = Vec::with_capacity(text.len());

        for ch in text.chars() {
            let gid = u32::from(face.glyph_index(ch).map_or(0, |g| g.0));
            let advance = face
                .glyph_hor_advance(ttf_parser::GlyphId(gid as u16))
                .unwrap_or(0);

            glyphs.push((gid, px));
            px += f32::from(advance) * scale;
        }

        TextLayout {
            glyphs,
            width: px,
            line_height: size,
            font_size: size,
        }
    }

    fn draw_text(&mut self, l: &TextLayout, px: f32, py: f32, x_align: f32, color: [f32; 4]) {
        let start_x = px - (l.width * x_align);
        let start_y = py - l.line_height * 0.5;
        let scale = l.font_size / f32::from(self.font.face.units_per_em());
        let ascender = f32::from(self.font.face.ascender()) * scale;

        for (gid, x_off) in &l.glyphs {
            if let Some(info) = self.font.atlas.glyphs.get(gid) {
                let gx = (start_x + x_off + (f32::from(info.metrics.x_min) * scale))
                    - ((ATLAS_RANGE + 1.0) / ATLAS_MSDF_SCALE * scale);
                let gy = (start_y + ascender - (f32::from(info.metrics.y_max) * scale))
                    - ((ATLAS_RANGE + 1.0) / ATLAS_MSDF_SCALE * scale);

                let gw =
                    (info.uv_rect[2] * self.font.atlas.width as f32) * (scale / ATLAS_MSDF_SCALE);
                let gh =
                    (info.uv_rect[3] * self.font.atlas.height as f32) * (scale / ATLAS_MSDF_SCALE);

                self.text_instances.push(TextInstance {
                    rect: [gx, gy, gw, gh],
                    uv_rect: info.uv_rect,
                    color,
                });
            }
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

        // We use a monotonic time for the GPU to calculate displacements
        let time = START_TIME.elapsed().as_secs_f32();

        // Get expansion animation variables
        let (interaction_inst, interaction_point) = self.interaction.last_expansion;
        let (expansion_xy, expansion_time) = (
            [interaction_point.x, PANEL_START + interaction_point.y],
            interaction_inst.duration_since(*START_TIME).as_secs_f32(),
        );

        self.gpu_uniforms = ScreenUniforms {
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
            self.particles.accumulator += dt * SPARK_EMISSION;
            let count = self.particles.accumulator.floor() as u8;
            self.particles.accumulator -= f32::from(count);
            count
        } else {
            self.particles.accumulator = 0.0;
            0
        };

        let spawn_offset = avg_speed.signum() * 2.0;
        let horizontal_bias = (avg_speed.abs().powf(0.2) * spawn_offset * 0.5).clamp(-3.0, 3.0);

        for particle in &mut self.particles.particles {
            // Emit a new particle
            if emit_count > 0 && time > particle.spawn_time + particle.duration {
                particle.spawn_y =
                    PANEL_START + CONFIG.height * lerpf32(fastrand::f32(), 0.1, 0.95);
                particle.spawn_vel = [
                    fastrand::usize(SPARK_VELOCITY_X) as f32 * self.scale_factor * horizontal_bias,
                    fastrand::usize(SPARK_VELOCITY_Y) as f32 * -self.scale_factor,
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
