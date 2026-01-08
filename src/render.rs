use crate::{
    CantusApp, PANEL_EXTENSION, PANEL_START,
    config::CONFIG,
    lerpf32,
    render_types::{BackgroundPill, Particle, PlayheadUniforms, Rect, ScreenUniforms},
    rspotify::{PlaylistId, Track},
    spotify::{ALBUM_DATA_CACHE, CondensedPlaylist, PLAYBACK_STATE},
    text_render::{ATLAS_MSDF_SCALE, ATLAS_RANGE, MSDFAtlas, TextInstance},
};
use std::{collections::HashMap, ops::Range, sync::LazyLock, time::Instant};
use ttf_parser::{Face, Tag};

static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

/// Spacing between tracks in ms
const TRACK_SPACING_MS: f32 = 4000.0;
/// Particles emitted per second when playback is active.
const SPARK_EMISSION: f32 = 60.0;
/// Horizontal velocity range applied at spawn.
const SPARK_VELOCITY_X: Range<usize> = 75..100;
/// Vertical velocity range applied at spawn.
const SPARK_VELOCITY_Y: Range<usize> = 30..70;
/// Lifetime range for individual particles, in seconds.
const SPARK_LIFETIME: Range<f32> = 0.4..0.9;

/// Duration for animation events
const ANIMATION_DURATION: f32 = 2.0;

pub struct RenderState {
    pub last_update: Instant,
    pub track_offset: f32,
    pub recent_speeds: [f32; 16],
    pub speed_idx: usize,
    pub speed_sum: f32,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            track_offset: 0.0,
            recent_speeds: [0.0; 16],
            speed_idx: 0,
            speed_sum: 0.0,
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
        let scale = self.scale_factor;
        let history_width = (CONFIG.history_width * scale).ceil();
        let total_width = (CONFIG.width * scale - history_width).ceil();
        let total_height = (CONFIG.height * scale).ceil();
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
            self.interaction.last_event = Instant::now();
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
        self.render_state.speed_sum += frame_move_speed - self.render_state.recent_speeds[s_idx];
        self.render_state.recent_speeds[s_idx] = frame_move_speed;
        self.render_state.speed_idx = (s_idx + 1) % 16;
        // Get new average
        let avg_speed = self.render_state.speed_sum / 16.0;

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
            self.draw_track(
                track_render,
                total_height,
                origin_x,
                &playback_state.playlists,
                image_map,
            );
        }

        // Draw the particles
        self.render_playhead_particles(
            dt,
            &playback_state.queue[cur_idx],
            origin_x,
            total_height,
            avg_speed,
            playback_state.volume,
        );
    }

    fn draw_track(
        &mut self,
        track_render: &TrackRender,
        height: f32,
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
        let hitbox = Rect::new(start_x, PANEL_START, start_x + width, PANEL_START + height);

        // Fade out based on width
        let fade_alpha = if width < height {
            ((width / height) * 1.5 - 0.5).max(0.0)
        } else {
            1.0
        };

        // How much of the width is to the left of the current position
        let dark_width = (origin_x - start_x).max(0.0);

        // Add hitbox
        let (hit_start, hit_end) = track_render.hitbox_range;
        let full_width = hit_end - hit_start;
        self.interaction
            .track_hitboxes
            .push((track.id, hitbox, track_render.hitbox_range));
        // If dragging, set the drag target to this track, and the position within the track
        if self.interaction.dragging && track_render.is_current {
            let position_within_track = (start_x + dark_width - hit_start) / full_width;
            self.interaction.drag_track = Some((track.id, position_within_track));
        }

        let Some(album_data_ref) = ALBUM_DATA_CACHE.get(&track.album.id) else {
            return;
        };
        let Some(album_data) = album_data_ref.as_ref() else {
            return;
        };

        // --- BACKGROUND ---
        let mut colors = [0u32; 4];
        for (i, c) in album_data.primary_colors.iter().take(4).enumerate() {
            colors[i] =
                (u32::from(c[0])) | (u32::from(c[1]) << 8) | (u32::from(c[2]) << 16) | (255 << 24);
        }

        // Determine which animation to show: specific track click or global playhead event
        let (expansion_time, expansion_pos) = {
            let (c_inst, c_track, c_pt) = self.interaction.last_click;
            let c_time = c_inst.duration_since(*START_TIME).as_secs_f32();
            let e_time = self
                .interaction
                .last_event
                .duration_since(*START_TIME)
                .as_secs_f32();

            if c_track == track.id && (c_time > e_time || !track_render.is_current) {
                (c_time, [(start_x + c_pt.x), (PANEL_START + c_pt.y)])
            } else {
                (e_time, [origin_x, (PANEL_START + height * 0.5)])
            }
        };

        self.background_pills.push(BackgroundPill {
            rect: [start_x, PANEL_START, width, height],
            dark_width,
            alpha: fade_alpha,
            colors,
            expansion_pos,
            expansion_time,
            image_index: track_render.image_index,
            _padding: [0.0; 2],
        });

        // --- TEXT ---
        if !track_render.art_only && fade_alpha >= 1.0 && width > height {
            // Get available width for text
            let text_start_left = start_x + 12.0;
            let text_start_right = start_x + width - height - 8.0;
            let available_width = (text_start_right - text_start_left).max(0.0);
            let text_alpha = (available_width / 100.0).min(1.0);
            let text_color = [0.94, 0.94, 0.94, text_alpha];

            // Render the songs title (strip anything beyond a - or ( in the song title)
            let song_name = track.name[..track
                .name
                .find(" (")
                .or_else(|| track.name.find(" -"))
                .unwrap_or(track.name.len())]
                .trim();
            let font_size = 12.0;
            let text_height = PANEL_START + (height * 0.2).floor();
            let song_layout = self.layout_text(song_name, font_size);
            let width_ratio = available_width / song_layout.width;
            if width_ratio <= 1.0 {
                self.draw_text(
                    &self.layout_text(song_name, font_size * width_ratio.max(0.8)),
                    text_start_left,
                    text_height,
                    0.0,
                    text_color,
                );
            } else {
                self.draw_text(&song_layout, text_start_right, text_height, 1.0, text_color);
            }

            // Get text layouts for bottom row of text
            let font_size = 10.5;
            let text_height = PANEL_START + (height * 0.52).floor();

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
            let mut layout = self.layout_text(&bottom_text, font_size);
            let width_ratio = available_width / layout.width;
            if width_ratio <= 1.0 || !track_render.is_current {
                if width_ratio < 1.0 {
                    layout =
                        self.layout_text(&bottom_text, font_size * width_ratio.clamp(0.8, 1.0));
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
                    &self.layout_text(&time_text, font_size),
                    start_x + 12.0,
                    text_height,
                    0.0,
                    text_color,
                );
                self.draw_text(
                    &self.layout_text(artist_text, font_size),
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
                && hitbox
                    .inflate(0.0, 20.0)
                    .contains(self.interaction.mouse_position);
            self.draw_playlist_buttons(
                track, hovered, playlists, width, height, start_x, image_map,
            );
        }
    }

    /// Creates the text layout for a single-line string.
    fn layout_text(&self, text: &str, size: f32) -> TextLayout {
        let face = &self.font.face;
        let psize = size * self.scale_factor;
        let scale = psize / f32::from(face.units_per_em());
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
            line_height: psize,
            font_size: psize,
        }
    }

    fn draw_text(&mut self, l: &TextLayout, px: f32, py: f32, x_align: f32, color: [f32; 4]) {
        let start_x = px - (l.width * x_align);
        let start_y = py - l.line_height * 0.5;
        let scale = l.font_size / f32::from(self.font.face.units_per_em());
        let ascender = f32::from(self.font.face.ascender()) * scale;

        for (gid, x_off) in &l.glyphs {
            if let Some(info) = self.font.atlas.glyphs.get(gid) {
                let msdf_scale = ATLAS_MSDF_SCALE;
                let range = ATLAS_RANGE;

                // Position relative to baseline
                let gx = (start_x + x_off + (f32::from(info.metrics.x_min) * scale))
                    / self.scale_factor
                    - ((range + 1.0) / msdf_scale * scale);
                let gy = (start_y + ascender - (f32::from(info.metrics.y_max) * scale))
                    / self.scale_factor
                    - ((range + 1.0) / msdf_scale * scale);

                let gw = (info.uv_rect[2] * self.font.atlas.width as f32) * (scale / msdf_scale)
                    / self.scale_factor;
                let gh = (info.uv_rect[3] * self.font.atlas.height as f32) * (scale / msdf_scale)
                    / self.scale_factor;

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
        origin_x: f32,
        height: f32,
        track_move_speed: f32,
        volume: Option<u8>,
    ) {
        let Some(track_data_ref) = ALBUM_DATA_CACHE.get(&track.album.id) else {
            return;
        };
        let Some(track_data) = track_data_ref.as_ref() else {
            return;
        };

        let mut palette: Vec<u32> = track_data
            .primary_colors
            .iter()
            .map(|[r, g, b, _]| {
                // Pack as RGBA (little-endian u32) for WGSL unpack4x8unorm
                (u32::from(*r)) | (u32::from(*g) << 8) | (u32::from(*b) << 16) | (255 << 24)
            })
            .collect();
        if palette.is_empty() {
            palette.extend_from_slice(&[
                102 | (102 << 8) | (102 << 16),
                153 | (153 << 8) | (153 << 16),
                204 | (204 << 8) | (204 << 16),
            ]);
        }

        // We use a monotonic time for the GPU to calculate displacements
        let time = START_TIME.elapsed().as_secs_f32();

        self.gpu_uniforms = Some(ScreenUniforms {
            screen_size: [
                (CONFIG.width * self.scale_factor),
                ((CONFIG.height + PANEL_START + PANEL_EXTENSION) * self.scale_factor),
            ],
            time,
            scale_factor: self.scale_factor,
            mouse_pos: [
                self.interaction.mouse_position.x,
                self.interaction.mouse_position.y,
            ],
        });

        // Emit new particles while playing
        let mut emit_count = if track_move_speed.abs() > 0.000_001 {
            self.particles.accumulator += dt * SPARK_EMISSION;
            let count = self.particles.accumulator.floor() as u8;
            self.particles.accumulator -= f32::from(count);
            count
        } else {
            self.particles.accumulator = 0.0;
            0
        };

        let spawn_offset = track_move_speed.signum() * 2.0;
        let horizontal_bias =
            (track_move_speed.abs().powf(0.2) * spawn_offset * 0.5).clamp(-3.0, 3.0);

        for particle in &mut self.particles.particles {
            // Emit a new particle
            if emit_count > 0 && time > particle.spawn_time + particle.duration {
                particle.spawn_pos = [
                    origin_x,
                    PANEL_START + height * lerpf32(fastrand::f32(), 0.1, 0.95),
                ];
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
        let playbutton_hsize = height * 0.25;
        let speed = 2.2 * dt;
        interaction.play_hitbox = Rect::new(
            origin_x - playbutton_hsize,
            PANEL_START,
            origin_x + playbutton_hsize,
            PANEL_START + height,
        );
        // Get playhead states
        let playhead_hovered = interaction.play_hitbox.contains(interaction.mouse_position);
        let last_event = interaction.last_event.elapsed().as_secs_f32() / ANIMATION_DURATION;

        // Determine the intended state for the bar
        let bar_target =
            u32::from(playhead_hovered || !interaction.playing || last_event < 1.0) as f32;
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
        } else if interaction.playing && last_event < 1.0 {
            interaction.playhead_play = last_event; // Hard set for the "start" animation
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

        self.playhead_info = Some(PlayheadUniforms {
            origin_x,
            panel_start: PANEL_START,
            height,
            volume: f32::from(volume.unwrap_or(100)) / 100.0,
            bar_lerp: interaction.playhead_bar,
            play_lerp: interaction.playhead_play,
            pause_lerp: interaction.playhead_pause,
            _padding: 0.0,
        });
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
