use crate::{
    CantusApp, PANEL_HEIGHT_START,
    config::CONFIG,
    interaction::InteractionEvent,
    lerpf32, lerpf64,
    render_types::{Particle, ParticleUniforms},
    rspotify::{PlaylistId, Track},
    spotify::{ALBUM_DATA_CACHE, CondensedPlaylist, IMAGES_CACHE, PLAYBACK_STATE},
};
use std::{
    collections::HashMap,
    ops::Range,
    sync::LazyLock,
    time::{Duration, Instant},
};
use ttf_parser::{Face, Tag};
use vello::{
    Glyph,
    kurbo::{Affine, BezPath, Circle, Point, Rect, RoundedRect, RoundedRectRadii, Shape},
    peniko::{Blob, Color, Fill, FontData, ImageBrush},
};

static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

/// Spacing between tracks in ms
const TRACK_SPACING_MS: f64 = 4000.0;
/// Particles emitted per second when playback is active.
const SPARK_EMISSION: f32 = 60.0;
/// Horizontal velocity range applied at spawn.
const SPARK_VELOCITY_X: Range<usize> = 75..100;
/// Vertical velocity range applied at spawn.
const SPARK_VELOCITY_Y: Range<usize> = 30..70;
/// Lifetime range for individual particles, in seconds.
const SPARK_LIFETIME: Range<f32> = 0.4..0.9;

/// Duration for animation events
const ANIMATION_DURATION: Duration = Duration::from_millis(3500);

static PLAY_SVG: LazyLock<BezPath> =
    LazyLock::new(|| BezPath::from_svg(include_str!("../assets/play.path")).unwrap());

pub struct RenderState {
    pub last_update: Instant,
    pub track_offset: f64,
    pub recent_speeds: [f64; 16],
    pub speed_idx: usize,
    pub speed_sum: f64,
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
    pub font_data: FontData,
    pub face: Face<'static>,
    pub coords: Vec<i16>,
}

pub struct TextLayout {
    glyphs: Vec<Glyph>,
    width: f64,
    height: f64,
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
        Self {
            font_data: FontData::new(Blob::from(bytes.to_vec()), 0),
            coords: face
                .variation_coordinates()
                .iter()
                .map(|c| c.get())
                .collect(),
            face,
        }
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
    seconds_until_start: f64,
    start_x: f64,
    width: f64,
    hitbox_range: (f64, f64),
    art_only: bool,
}

/// Build the scene for rendering.
impl CantusApp {
    pub fn create_scene(&mut self) {
        let now = Instant::now();
        let dt = now
            .duration_since(self.render_state.last_update)
            .as_secs_f64();
        self.render_state.last_update = now;

        let scale = self.scale_factor;
        let history_width = (CONFIG.history_width * scale).ceil();
        let total_width = (CONFIG.width * scale - history_width).ceil();
        let total_height = (CONFIG.height * scale).ceil();
        let timeline_duration_ms = CONFIG.timeline_future_minutes * 60_000.0;
        let timeline_start_ms = -CONFIG.timeline_past_minutes * 60_000.0;

        let px_per_ms = total_width / timeline_duration_ms;
        let timeline_origin_x = history_width - timeline_start_ms * px_per_ms;

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

        // Play button hitbox
        let playbutton_hsize = total_height * 0.25;
        self.interaction.play_hitbox = Rect::new(
            timeline_origin_x - playbutton_hsize,
            PANEL_HEIGHT_START,
            timeline_origin_x + playbutton_hsize,
            PANEL_HEIGHT_START + total_height,
        );
        let play_button_hovered = self
            .interaction
            .play_hitbox
            .contains(self.interaction.mouse_position);
        if play_button_hovered {
            if playback_state.playing
                && !matches!(self.interaction.last_event, InteractionEvent::PauseHover(_))
            {
                self.interaction.last_event = InteractionEvent::PauseHover(now);
            }
            if !playback_state.playing
                && !matches!(self.interaction.last_event, InteractionEvent::PlayHover(_))
            {
                self.interaction.last_event = InteractionEvent::PlayHover(now);
            }
        }

        // Update interaction events
        match self.interaction.last_event {
            InteractionEvent::Pause(_) => {
                if playback_state.playing {
                    self.interaction.last_event = InteractionEvent::Play(now);
                }
            }
            InteractionEvent::Play(_) => {
                if !playback_state.playing {
                    self.interaction.last_event = InteractionEvent::Pause(now);
                }
            }
            InteractionEvent::PauseHover(_) | InteractionEvent::PlayHover(_) => {
                if !play_button_hovered {
                    let instant = now.checked_sub(Duration::from_secs(5)).unwrap();
                    self.interaction.last_event = if playback_state.playing {
                        InteractionEvent::Play(instant)
                    } else {
                        InteractionEvent::Pause(instant)
                    }
                }
            }
        }
        if self.interaction.dragging {
            self.interaction.drag_track = None;
        }

        // Lerp the progress based on when the data was last updated, get the start time of the current track
        let playback_elapsed = f64::from(playback_state.progress)
            + if playback_state.playing {
                playback_state.last_progress_update.elapsed().as_millis() as f64
            } else {
                0.0
            };

        // Lerp track start based on the target and current start time
        let past_tracks_duration: f64 = playback_state
            .queue
            .iter()
            .take(cur_idx)
            .map(|t| f64::from(t.duration_ms))
            .sum();

        let mut current_ms = -playback_elapsed - past_tracks_duration + drag_offset_ms
            - TRACK_SPACING_MS * cur_idx as f64;
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
            let end = start + f64::from(track.duration_ms);
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
            self.draw_track(
                track_render,
                total_height,
                timeline_origin_x,
                &playback_state.playlists,
            );
        }

        // Draw the particles
        self.render_playing_particles(
            dt as f32,
            &playback_state.queue[cur_idx],
            timeline_origin_x,
            total_height,
            avg_speed as f32,
            playback_state.volume,
        );
    }

    fn draw_track(
        &mut self,
        track_render: &TrackRender,
        height: f64,
        timeline_origin_x: f64,
        playlists: &HashMap<PlaylistId, CondensedPlaylist>,
    ) {
        if track_render.width <= 0.0 {
            return;
        }
        let width = track_render.width;
        let track = track_render.track;
        let start_x = track_render.start_x;
        let hitbox = Rect::new(
            start_x,
            PANEL_HEIGHT_START,
            start_x + width,
            PANEL_HEIGHT_START + height,
        );
        let start_translation = Affine::translate((start_x, PANEL_HEIGHT_START));

        // Fade out based on width
        let fade_alpha = if width < height {
            ((width / height) as f32 * 1.5 - 0.5).max(0.0)
        } else {
            1.0
        };

        // How much of the width is to the left of the current position
        let dark_width = (timeline_origin_x - start_x).max(0.0);

        // Add hitbox
        let (hit_start, hit_end) = track_render.hitbox_range;
        let full_width = hit_end - hit_start;
        let crop_left = start_x - hit_start;
        let crop_right = hit_end - (start_x + width);
        self.interaction
            .track_hitboxes
            .push((track.id, hitbox, track_render.hitbox_range));
        // If dragging, set the drag target to this track, and the position within the track
        if self.interaction.dragging && track_render.is_current {
            let position_within_track = (start_x + dark_width - hit_start) / full_width;
            self.interaction.drag_track = Some((track.id, position_within_track));
        }

        let (Some(album_image_ref), Some(album_data_ref)) = (
            IMAGES_CACHE.get(&track.album.image),
            ALBUM_DATA_CACHE.get(&track.album.id),
        ) else {
            return;
        };
        let Some(album_image) = album_image_ref.as_ref() else {
            return;
        };
        let Some(album_data) = album_data_ref.as_ref() else {
            return;
        };

        let rounding = 14.0 * self.scale_factor;
        let buffer_px = 20.0;
        let left_rounding = rounding * lerpf64((crop_left / buffer_px).clamp(0.0, 1.0), 1.0, 0.3);
        let right_rounding = rounding * lerpf64((crop_right / buffer_px).clamp(0.0, 1.0), 1.0, 0.3);
        let radii =
            RoundedRectRadii::new(left_rounding, right_rounding, right_rounding, left_rounding);

        // --- BACKGROUND ---
        if !track_render.art_only && width > height {
            // Don't need to render all the way to the edge since the album art is at the right edge
            let background_width = width - height * 0.25;
            let extra_fade_alpha = fade_alpha * ((width - height) / 30.0).min(1.0) as f32;

            // Add a drop shadow
            self.scene.draw_blurred_rounded_rect(
                start_translation,
                Rect::new(0.0, 0.0, width, height),
                Color::from_rgba8(0, 0, 0, (150.0 * extra_fade_alpha).round() as u8),
                rounding,
                6.0,
            );

            // Start clipping
            self.scene.push_clip_layer(
                start_translation,
                &RoundedRect::new(0.0, 0.0, background_width, height, radii),
            );
            let image_width = f64::from(album_data.palette_image.image.width);
            let background_aspect_ratio = background_width / height;
            self.scene.fill(
                Fill::EvenOdd,
                start_translation * Affine::scale(full_width / image_width),
                ImageBrush {
                    image: &album_data.palette_image.image,
                    sampler: album_data
                        .palette_image
                        .sampler
                        .with_alpha(extra_fade_alpha),
                },
                None,
                &Rect::new(0.0, 0.0, image_width, image_width * background_aspect_ratio),
            );
            // Add a white glow above for a vignette effect
            self.scene.draw_blurred_rounded_rect(
                start_translation,
                Rect::new(0.0, 0.0, width, height),
                Color::from_rgba8(255, 255, 255, (30.0 * extra_fade_alpha).round() as u8),
                rounding,
                15.0,
            );
            self.scene.pop_layer();
        }

        // --- Render things within the track bounds ---
        self.scene.push_clip_layer(
            start_translation,
            &RoundedRect::new(0.0, 0.0, width, height, radii),
        );

        // Make the track dark to the left of the current time
        if dark_width > 0.0 {
            let extra_fade_alpha = ((width - height) / 30.0).min(1.0) as f32;
            self.scene.fill(
                Fill::EvenOdd,
                start_translation,
                Color::from_rgb8(0, 0, 0).with_alpha(0.5 * fade_alpha * extra_fade_alpha),
                None,
                &Rect::new(0.0, 0.0, dark_width, height),
            );
        }

        // During animations add an expanding circle behind the line
        if track_render.is_current {
            let anim_lerp = match self.interaction.last_event {
                InteractionEvent::Pause(start) | InteractionEvent::Play(start) => {
                    start.elapsed().as_millis() as f64
                        / (ANIMATION_DURATION.as_millis() as f64 * 0.3)
                }
                InteractionEvent::PauseHover(_) | InteractionEvent::PlayHover(_) => 1.0,
            };
            if anim_lerp < 1.0 {
                self.scene.fill(
                    Fill::EvenOdd,
                    Affine::translate((timeline_origin_x, height * 0.5)),
                    Color::from_rgb8(255, 224, 210)
                        .with_alpha(1.0 - (anim_lerp + 0.4).min(1.0) as f32),
                    None,
                    &Circle::new(Point::default(), 500.0 * anim_lerp),
                );
            }
        }
        // After a click, add an expanding circle behind the click point
        if let Some((start, track_id, point)) = &self.interaction.last_click
            && track_id == &track.id
            && let anim_lerp =
                start.elapsed().as_millis() as f64 / (ANIMATION_DURATION.as_millis() as f64 * 0.3)
            && anim_lerp < 1.0
        {
            self.scene.fill(
                Fill::EvenOdd,
                start_translation * Affine::translate((point.x, point.y)),
                Color::from_rgb8(255, 224, 210).with_alpha(1.0 - (anim_lerp + 0.4).min(1.0) as f32),
                None,
                &Circle::new(Point::default(), 500.0 * anim_lerp),
            );
        }
        self.scene.pop_layer();

        // --- ALBUM ART SQUARE ---
        if fade_alpha > 0.0 {
            // Add a drop shadow
            self.scene.draw_blurred_rounded_rect(
                start_translation * Affine::translate((width - height, 0.0)),
                Rect::new(0.0, 0.0, height, height),
                Color::from_rgba8(0, 0, 0, (100.0 * fade_alpha).round() as u8),
                rounding,
                5.0,
            );

            self.scene.push_clip_layer(
                start_translation,
                &RoundedRect::new(0.0, 0.0, width, height, radii),
            );
            self.scene.fill(
                Fill::EvenOdd,
                start_translation * Affine::translate((width - height, 0.0)),
                ImageBrush {
                    image: &album_image.image,
                    sampler: album_image.sampler.with_alpha(fade_alpha),
                },
                Some(Affine::scale(height / f64::from(album_image.image.height))),
                &RoundedRect::new(
                    0.0,
                    0.0,
                    height,
                    height,
                    RoundedRectRadii::new(rounding, right_rounding, right_rounding, rounding),
                ),
            );
            self.scene.pop_layer();
        }

        // --- TEXT ---
        if !track_render.art_only && fade_alpha >= 1.0 && width > height {
            // Clipping mask to the edge of the background rectangle, shrunk by a margin
            self.scene.push_clip_layer(
                start_translation,
                &RoundedRect::new(
                    4.0,
                    4.0,
                    width - height - 4.0,
                    height - 4.0,
                    6.0 * self.scale_factor,
                ),
            );
            // Get available width for text
            let text_start_left = start_x + 12.0;
            let text_start_right = start_x + width - height - 8.0;
            let available_width = (text_start_right - text_start_left).max(0.0);
            let text_brush = Color::from_rgba8(
                240,
                240,
                240,
                ((available_width / 100.0).min(1.0) * 255.0) as u8,
            );

            // Render the songs title (strip anything beyond a - or ( in the song title)
            let song_name = track.name[..track
                .name
                .find(" (")
                .or_else(|| track.name.find(" -"))
                .unwrap_or(track.name.len())]
                .trim();
            let font_size = 12.0;
            let text_height = PANEL_HEIGHT_START + (height * 0.2).floor();
            let song_layout = self.layout_text(song_name, font_size);
            let width_ratio = available_width / song_layout.width;
            if width_ratio <= 1.0 {
                self.draw_text(
                    &self.layout_text(song_name, font_size * width_ratio.max(0.8)),
                    text_start_left,
                    text_height,
                    false,
                    text_brush,
                );
            } else {
                self.draw_text(
                    &song_layout,
                    text_start_right,
                    text_height,
                    true,
                    text_brush,
                );
            }

            // Get text layouts for bottom row of text
            let font_size = 10.5;
            let text_height = PANEL_HEIGHT_START + (height * 0.52).floor();

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
                    width_ratio >= 1.0,
                    text_brush,
                );
            } else {
                self.draw_text(
                    &self.layout_text(&time_text, font_size),
                    start_x + 12.0,
                    text_height,
                    false,
                    text_brush,
                );
                self.draw_text(
                    &self.layout_text(artist_text, font_size),
                    text_start_right,
                    text_height,
                    true,
                    text_brush,
                );
            }

            // Release clipping mask
            self.scene.pop_layer();
        }

        // Expand the hitbox vertically so it includes the playlist buttons
        if !track_render.art_only {
            let hovered = !self.interaction.dragging
                && hitbox
                    .inflate(0.0, 20.0)
                    .contains(self.interaction.mouse_position);
            self.draw_playlist_buttons(track, hovered, playlists, width, height, start_x);
        }
    }

    /// Creates the text layout for a single-line string.
    fn layout_text(&self, text: &str, size: f64) -> TextLayout {
        let face = &self.font.face;
        let psize = (size * self.scale_factor) as f32;
        let scale = psize / f32::from(face.units_per_em());
        let (mut px, mut glyphs, mut prev) = (0.0f32, Vec::with_capacity(text.len()), None);
        for ch in text.chars() {
            let gid = face.glyph_index(ch);
            if let (Some(l), Some(r)) = (prev, gid) {
                px += face.tables().kern.map_or(0.0, |t| {
                    let mut adj = 0.0f32;
                    for s in t.subtables {
                        if s.horizontal
                            && !s.has_cross_stream
                            && !s.has_state_machine
                            && let Some(v) = s.glyphs_kerning(l, r)
                        {
                            adj += f32::from(v);
                        }
                    }
                    adj
                }) * scale;
            }
            let adv = gid
                .and_then(|g| face.glyph_hor_advance(g))
                .map_or_else(|| f32::from(face.units_per_em()) * 0.5, f32::from);
            if let Some(g) = gid {
                glyphs.push(Glyph {
                    id: u32::from(g.0),
                    x: px,
                    y: f32::from(face.ascender()) * scale,
                });
                prev = Some(g);
            } else {
                prev = None;
            }
            px += adv * scale;
        }
        TextLayout {
            glyphs,
            width: f64::from(px),
            height: f64::from(
                f32::from(face.ascender() + face.descender() + face.line_gap()) * scale,
            ),
            font_size: psize,
        }
    }

    fn draw_text(&mut self, l: &TextLayout, px: f64, py: f64, align_e: bool, brush: Color) {
        self.scene
            .draw_glyphs(&self.font.font_data)
            .font_size(l.font_size)
            .normalized_coords(&self.font.coords)
            .transform(Affine::translate((
                px - (l.width * f64::from(u8::from(align_e))),
                py - l.height * 0.5,
            )))
            .hint(true)
            .brush(brush)
            .draw(Fill::EvenOdd, l.glyphs.iter().copied());
    }

    fn render_playing_particles(
        &mut self,
        dt: f32,
        track: &Track,
        x: f64,
        height: f64,
        track_move_speed: f32,
        volume: Option<u8>,
    ) {
        let scale = self.scale_factor as f32;
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

        self.gpu_uniforms = Some(ParticleUniforms {
            screen_size: [CONFIG.width as f32 * scale, height as f32],
            time,
            _padding: 0.0,
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
                    x as f32,
                    PANEL_HEIGHT_START as f32 + height as f32 * lerpf32(fastrand::f32(), 0.1, 0.7),
                ];
                particle.spawn_vel = [
                    fastrand::usize(SPARK_VELOCITY_X) as f32 * scale * horizontal_bias,
                    fastrand::usize(SPARK_VELOCITY_Y) as f32 * -scale,
                ];
                particle.color = palette[fastrand::usize(0..palette.len())];
                particle.spawn_time = time;
                particle.duration =
                    lerpf32(fastrand::f32(), SPARK_LIFETIME.start, SPARK_LIFETIME.end);
                emit_count -= 1;
            }
        }

        // Line and Volume logic
        let line_width = 4.0 * self.scale_factor;
        let line_color = Color::from_rgb8(255, 224, 210);
        let line_x = x - line_width * 0.5;
        let anim_lerp = match self.interaction.last_event {
            InteractionEvent::Play(start) => (start.elapsed().as_millis() as f64
                / ANIMATION_DURATION.as_millis() as f64)
                .clamp(0.0, 1.0),
            InteractionEvent::Pause(start) => (start.elapsed().as_millis() as f64
                / ANIMATION_DURATION.as_millis() as f64)
                .clamp(0.0, 0.5),
            InteractionEvent::PauseHover(start) | InteractionEvent::PlayHover(start) => {
                (start.elapsed().as_millis() as f64
                    / (ANIMATION_DURATION.as_millis() as f64 * 0.25))
                    .clamp(0.0, 0.5)
            }
        };
        if anim_lerp < 1.0 {
            // Start with the lines split, then 3/4 way through close them again
            let line_height = height * lerpf64(((anim_lerp - 0.75) * 4.0).max(0.0), 0.2, 0.5);
            self.scene.fill(
                Fill::EvenOdd,
                Affine::translate((line_x, PANEL_HEIGHT_START)),
                line_color,
                None,
                &RoundedRect::new(0.0, 0.0, line_width, line_height, 100.0),
            );
            self.scene.fill(
                Fill::EvenOdd,
                Affine::translate((line_x, PANEL_HEIGHT_START + height - line_height)),
                line_color,
                None,
                &RoundedRect::new(0.0, 0.0, line_width, line_height, 100.0),
            );

            let icon_height = PANEL_HEIGHT_START + height * 0.3;
            let is_paused = matches!(
                self.interaction.last_event,
                InteractionEvent::Pause(_) | InteractionEvent::PauseHover(_)
            );
            if is_paused || anim_lerp < 0.5 {
                // Two lines for pause, during a pause its always there, when on play it fades out
                let anim_lerp = if is_paused {
                    anim_lerp
                } else {
                    (anim_lerp + 0.5) * 1.5
                };
                let icon_fade = ((anim_lerp - 0.75) * 4.0).clamp(0.0, 1.0);
                let icon_color = line_color.with_alpha(1.0 - icon_fade as f32);
                let icon_offset = 5.0 * (anim_lerp * 4.0).min(1.0) + 4.0 * icon_fade;
                self.scene.fill(
                    Fill::EvenOdd,
                    Affine::translate((line_x - icon_offset, icon_height)),
                    icon_color,
                    None,
                    &RoundedRect::new(0.0, 0.0, line_width, icon_height, 100.0),
                );
                self.scene.fill(
                    Fill::EvenOdd,
                    Affine::translate((line_x + icon_offset, icon_height)),
                    icon_color,
                    None,
                    &RoundedRect::new(0.0, 0.0, line_width, icon_height, 100.0),
                );
            }
            if !is_paused || anim_lerp < 0.5 {
                // Render out the play icon svg, grow it in the first quarter, then keep in place for half, then expand out with a fade in final quarter
                let anim_lerp = if is_paused {
                    (anim_lerp + 0.5) * 2.0
                } else {
                    anim_lerp
                };
                let icon_fade = ((anim_lerp - 0.75) * 4.0).clamp(0.0, 1.0);
                let icon_color = line_color.with_alpha(1.0 - icon_fade as f32);
                let play_icon_width = PLAY_SVG.bounding_box().width();
                let icon_scale = icon_height * (anim_lerp * 4.0).min(1.0) + 0.5 * icon_fade;
                self.scene.fill(
                    Fill::EvenOdd,
                    Affine::translate((
                        line_x - icon_scale * 0.3,
                        PANEL_HEIGHT_START + height * 0.5 - icon_scale * 0.5,
                    )) * Affine::scale(icon_scale / play_icon_width),
                    icon_color,
                    None,
                    &*PLAY_SVG,
                );
            }
        } else {
            // Volume display bar
            let volume = f64::from(volume.unwrap_or(100)) / 100.0;
            if volume < 1.0 {
                self.scene.fill(
                    Fill::EvenOdd,
                    Affine::translate((line_x, PANEL_HEIGHT_START)),
                    Color::from_rgb8(150, 150, 150),
                    None,
                    &RoundedRect::new(0.0, 0.0, line_width, height, 100.0),
                );
            }
            self.scene.fill(
                Fill::EvenOdd,
                Affine::translate((line_x, PANEL_HEIGHT_START + height * (1.0 - volume))),
                line_color,
                None,
                &RoundedRect::new(0.0, 0.0, line_width, height * volume, 100.0),
            );
        }
    }
}
