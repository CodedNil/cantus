use crate::{
    CantusApp,
    config::CONFIG,
    interaction::InteractionEvent,
    lerpf64,
    spotify::{IMAGES_CACHE, PLAYBACK_STATE, Playlist, TRACK_DATA_CACHE, Track},
};
use std::{
    collections::HashMap,
    ops::Range,
    sync::LazyLock,
    time::{Duration, Instant},
};
use ttf_parser::{Face, GlyphId, NormalizedCoordinate, Tag, VariationAxis};
use vello::{
    Glyph,
    kurbo::{Affine, BezPath, Circle, Point, Rect, RoundedRect, RoundedRectRadii, Shape},
    peniko::{Blob, Color, Fill, FontData, ImageBrush},
};

/// Spacing between tracks in ms
const TRACK_SPACING_MS: f64 = 4000.0;

/// Particles emitted per second when playback is active.
const SPARK_EMISSION: f64 = 60.0;
/// Downward acceleration applied to each particle (scaled by DPI).
const SPARK_GRAVITY: f64 = 300.0;
/// Horizontal velocity range applied at spawn.
const SPARK_VELOCITY_X: Range<usize> = 75..100;
/// Vertical velocity range applied at spawn.
const SPARK_VELOCITY_Y: Range<usize> = 30..70;
/// Lifetime range for individual particles, in seconds.
const SPARK_LIFETIME: Range<f64> = 0.3..0.6;
/// Rendered spark segment length range, in logical pixels.
const SPARK_LENGTH_RANGE: Range<f64> = 6.0..10.0;
/// Rendered spark thickness range, in logical pixels.
const SPARK_THICKNESS_RANGE: Range<f64> = 2.0..4.0;

/// Duration for animation events
const ANIMATION_DURATION: Duration = Duration::from_millis(3500);

static PLAY_SVG: LazyLock<BezPath> =
    LazyLock::new(|| BezPath::from_svg(include_str!("../assets/play.path")).unwrap());

pub struct RenderState {
    last_update: Instant,
    track_offset: f64,
    move_speeds: [f64; 16],
    move_index: usize,
    move_speed_sum: f64,
    layout_cache: HashMap<TextLayoutKey, TextLayout>,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            track_offset: 0.0,
            move_speeds: [0.0; 16],
            move_index: 0,
            move_speed_sum: 0.0,
            layout_cache: HashMap::with_capacity(200),
        }
    }
}
pub struct FontEngine {
    font_data: FontData,
    base_face: Face<'static>,
    axes: Vec<VariationAxis>,
    weight_axis_index: Option<usize>,
}

#[derive(Clone, Copy)]
enum Align {
    Start,
    End,
}

#[derive(Clone)]
struct TextLayout {
    glyphs: Vec<Glyph>,
    width: f64,
    height: f64,
    font_size: f32,
    coords: Vec<i16>,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct TextLayoutKey {
    text: String,
    font_size: u32,
}

impl TextLayoutKey {
    fn new(text: &str, font_size: f64) -> Self {
        let font_size_key = (font_size * 100.0).round() as u32;
        Self {
            text: text.to_owned(),
            font_size: font_size_key,
        }
    }
}

impl Default for FontEngine {
    fn default() -> Self {
        let bytes = include_bytes!("../assets/NotoSans.ttf");
        let font_data = FontData::new(Blob::from(bytes.to_vec()), 0);
        let face = Face::parse(bytes, 0).expect("failed to parse embedded font");
        let axes = face.variation_axes().into_iter().collect::<Vec<_>>();
        let weight_axis_index = axes
            .iter()
            .position(|axis| axis.tag == Tag::from_bytes(b"wght"));

        Self {
            font_data,
            base_face: face,
            axes,
            weight_axis_index,
        }
    }
}

pub struct ParticlesState {
    particles: [Particle; 32],
    spawn_accumulator: f64,
}

impl Default for ParticlesState {
    fn default() -> Self {
        Self {
            particles: [Particle {
                life: 0.0,
                position: [0.0, 0.0],
                velocity: [0.0, 0.0],
                color: 0,
            }; 32],
            spawn_accumulator: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
struct Particle {
    life: f64,
    position: [f64; 2],
    velocity: [f64; 2],
    color: usize,
}

struct TrackRender<'a> {
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
        let dt = self.render_state.last_update.elapsed().as_secs_f64();
        self.render_state.last_update = Instant::now();

        if self.render_state.layout_cache.len() > 200 {
            self.render_state.layout_cache.clear();
        }

        let history_width = (CONFIG.history_width * self.scale_factor).ceil();
        let timeline_duration_ms = CONFIG.timeline_future_minutes * 60_000.0;
        let timeline_start_ms = -CONFIG.timeline_past_minutes * 60_000.0;
        let timeline_end_ms = timeline_start_ms + timeline_duration_ms;

        let total_width = (CONFIG.width * self.scale_factor - history_width).ceil();
        let total_height = (CONFIG.height * self.scale_factor).ceil();
        let px_per_ms = total_width / timeline_duration_ms;
        let timeline_origin_x = history_width - timeline_start_ms * px_per_ms;

        let playback_state = PLAYBACK_STATE.read();
        let queue = &playback_state.queue;
        self.interaction.icon_hitboxes.clear();
        self.interaction.track_hitboxes.clear();
        if queue.is_empty() {
            return;
        }

        let drag_offset_ms = if self.interaction.dragging {
            self.interaction.drag_delta_pixels / px_per_ms
        } else {
            0.0
        };
        let current_index = playback_state.queue_index.min(queue.len() - 1);

        // Play button hitbox
        let playbutton_center = timeline_origin_x;
        let playbutton_hsize = total_height * 0.25;
        self.interaction.play_hitbox = Rect::new(
            playbutton_center - playbutton_hsize,
            0.0,
            playbutton_center + playbutton_hsize,
            total_height,
        );
        let play_button_hovered = self
            .interaction
            .play_hitbox
            .contains(self.interaction.mouse_position);
        if play_button_hovered {
            if playback_state.playing
                && !matches!(self.interaction.last_event, InteractionEvent::PauseHover(_))
            {
                self.interaction.last_event = InteractionEvent::PauseHover(Instant::now());
            }
            if !playback_state.playing
                && !matches!(self.interaction.last_event, InteractionEvent::PlayHover(_))
            {
                self.interaction.last_event = InteractionEvent::PlayHover(Instant::now());
            }
        }

        // Update interaction events
        match self.interaction.last_event {
            InteractionEvent::Pause(_) => {
                if playback_state.playing {
                    self.interaction.last_event = InteractionEvent::Play(Instant::now());
                }
            }
            InteractionEvent::Play(_) => {
                if !playback_state.playing {
                    self.interaction.last_event = InteractionEvent::Pause(Instant::now());
                }
            }
            InteractionEvent::PauseHover(_) | InteractionEvent::PlayHover(_) => {
                if !play_button_hovered {
                    let instant = Instant::now().checked_sub(Duration::from_secs(5)).unwrap();
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
                playback_state.last_updated.elapsed().as_millis() as f64
            } else {
                0.0
            };

        // Lerp track start based on the target and current start time
        let past_tracks_duration: f64 = queue
            .iter()
            .take(current_index)
            .map(|t| f64::from(t.milliseconds))
            .sum();
        let mut current_ms = -playback_elapsed - past_tracks_duration + drag_offset_ms
            - TRACK_SPACING_MS * current_index as f64;
        let difference = current_ms - self.render_state.track_offset;
        if !self.interaction.dragging && difference.abs() > 200.0 {
            current_ms = self.render_state.track_offset + difference * 0.1;
        }

        // Add the new move speed to the array move_speeds, trim the previous ones
        let frame_move_speed = (current_ms - self.render_state.track_offset) * dt;
        self.render_state.track_offset = current_ms;
        let idx = self.render_state.move_index;
        self.render_state.move_speed_sum += frame_move_speed - self.render_state.move_speeds[idx];
        self.render_state.move_speeds[idx] = frame_move_speed;
        self.render_state.move_index = (idx + 1) % self.render_state.move_speeds.len();
        // Get new average
        let track_move_speed =
            self.render_state.move_speed_sum / self.render_state.move_speeds.len() as f64;

        // Iterate over the tracks within the timeline.
        let mut track_renders = Vec::with_capacity(queue.len());
        for track in queue {
            let track_start_ms = current_ms;
            let track_end_ms = track_start_ms + f64::from(track.milliseconds);
            current_ms = track_end_ms + TRACK_SPACING_MS;

            // Queue up the tracks positions
            let visible_start_px = track_start_ms.max(timeline_start_ms) * px_per_ms;
            let visible_end_px = track_end_ms.min(timeline_end_ms) * px_per_ms;
            let hitbox_range = (
                (track_start_ms - timeline_start_ms) * px_per_ms + history_width,
                (track_end_ms - timeline_start_ms) * px_per_ms + history_width,
            );

            let start_x = (visible_start_px - timeline_start_ms * px_per_ms) + history_width;
            let is_current = track_start_ms <= 0.0 && track_end_ms >= 0.0;
            let seconds_until_start = (track_start_ms / 1000.0).abs();
            let width = visible_end_px - visible_start_px;
            track_renders.push(TrackRender {
                track,
                is_current,
                seconds_until_start,
                start_x,
                width,
                hitbox_range,
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
                    current_px = history_width - total_height - track_spacing;

                    // Smooth out the snapping
                    current_px -=
                        (distance_before - (total_height - track_spacing * 2.0)).clamp(0.0, 30.0);
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
                history_width,
                px_per_ms,
                total_height,
                &playback_state.playlists,
            );
        }

        // Draw the particles
        self.render_playing_particles(
            dt,
            &queue[current_index],
            timeline_origin_x,
            total_height,
            track_move_speed,
            playback_state.volume,
        );
    }

    fn draw_track(
        &mut self,
        track_render: &TrackRender,
        history_width: f64,
        px_per_ms: f64,
        height: f64,
        playlists: &HashMap<String, Playlist>,
    ) {
        let width = track_render.width;
        if width <= 0.0 {
            return;
        }
        let track = track_render.track;
        let start_x = track_render.start_x;

        let timeline_origin_x =
            history_width - (-CONFIG.timeline_past_minutes * 60_000.0) * px_per_ms;
        let start_translation = Affine::translate((start_x, 0.0));

        // Fade out based on width
        let fade_alpha = if width < height {
            ((width / height) as f32 * 1.5 - 0.5).max(0.0)
        } else {
            1.0
        };

        // How much of the width is to the left of the current position
        let dark_width = (timeline_origin_x - start_x).max(0.0);

        // Add hitbox
        let hitbox = Rect::new(start_x, 0.0, start_x + width, height);
        self.interaction
            .track_hitboxes
            .push((track.id.clone(), hitbox, track_render.hitbox_range));
        // If dragging, set the drag target to this track, and the position within the track
        if self.interaction.dragging && track_render.is_current {
            let position_within_track = (start_x + dark_width - track_render.hitbox_range.0)
                / (track_render.hitbox_range.1 - track_render.hitbox_range.0);
            self.interaction.drag_track = Some((track.id.clone(), position_within_track));
        }

        let (Some(album_image), Some(track_data)) = (
            IMAGES_CACHE.get(&track.image_url),
            TRACK_DATA_CACHE.get(&track.id),
        ) else {
            return;
        };

        let rounding = 14.0 * self.scale_factor;
        let buffer_px = 20.0;
        let crop_left = start_x - track_render.hitbox_range.0;
        let crop_right = track_render.hitbox_range.1 - (start_x + width);
        let full_width = track_render.hitbox_range.1 - track_render.hitbox_range.0;
        let left_rounding = rounding * lerpf64((crop_left / buffer_px).clamp(0.0, 1.0), 1.0, 0.3);
        let right_rounding = rounding * lerpf64((crop_right / buffer_px).clamp(0.0, 1.0), 1.0, 0.3);
        let radii =
            RoundedRectRadii::new(left_rounding, right_rounding, right_rounding, left_rounding);
        // --- BACKGROUND ---
        if !track_render.art_only && width > height {
            // Don't need to render all the way to the edge since the album art is at the right edge
            let background_width = width - height * 0.25;
            self.scene.push_clip_layer(
                start_translation,
                &RoundedRect::new(0.0, 0.0, background_width, height, radii),
            );
            let image_width = f64::from(track_data.palette_image.image.width);
            let background_aspect_ratio = background_width / height;
            let extra_fade_alpha = ((width - height) / 30.0).min(1.0) as f32;
            self.scene.fill(
                Fill::EvenOdd,
                start_translation * Affine::scale(full_width / image_width),
                ImageBrush {
                    image: &track_data.palette_image.image,
                    sampler: track_data
                        .palette_image
                        .sampler
                        .with_alpha(fade_alpha * extra_fade_alpha),
                },
                None,
                &Rect::new(0.0, 0.0, image_width, image_width * background_aspect_ratio),
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
                Affine::translate((start_x + point.x, point.y)),
                Color::from_rgb8(255, 224, 210).with_alpha(1.0 - (anim_lerp + 0.4).min(1.0) as f32),
                None,
                &Circle::new(Point::default(), 500.0 * anim_lerp),
            );
        }

        // --- ALBUM ART SQUARE ---
        if fade_alpha > 0.0 {
            self.scene.fill(
                Fill::EvenOdd,
                Affine::translate((start_x + width - height, 0.0)),
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
        }

        self.scene.pop_layer();

        // --- TEXT ---
        if !track_render.art_only && fade_alpha >= 1.0 {
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
            let text_brush = Color::from_rgb8(240, 240, 240);

            // Render the songs title (strip anything beyond a - or ( in the song title)
            let song_name = track.title[..track
                .title
                .find(" (")
                .or_else(|| track.title.find(" -"))
                .unwrap_or(track.title.len())]
                .trim();
            let font_size = 12.0;
            let text_height = (height * 0.2).floor();
            let (layout, layout_width) = self.layout_text_cached(song_name, font_size);
            let width_ratio = available_width / layout_width;
            if width_ratio <= 1.0 {
                let (layout, _) =
                    self.layout_text_cached(song_name, font_size * width_ratio.max(0.8));
                self.draw_text(
                    &layout,
                    text_start_left,
                    text_height,
                    Align::Start,
                    // Fade out when it gets too small, 0.6-0.4
                    text_brush.with_alpha(((width_ratio - 0.4) / 0.2) as f32),
                );
            } else {
                self.draw_text(
                    &layout,
                    text_start_right,
                    text_height,
                    Align::End,
                    text_brush,
                );
            }

            // Get text layouts for bottom row of text
            let font_size = 10.5;
            let text_height = (height * 0.52).floor();

            let artist_text = &track.artist_name;
            let (artist_layout, artist_layout_width) =
                self.layout_text_cached(artist_text, font_size);
            let dot_text = "\u{2004}•\u{2004}"; // Use thin spaces on either side of the bullet point
            let (_, dot_layout_width) = self.layout_text_cached(dot_text, font_size);
            let time_text = if track_render.seconds_until_start >= 60.0 {
                format!(
                    "{}m{}s",
                    (track_render.seconds_until_start / 60.0).floor(),
                    (track_render.seconds_until_start % 60.0).floor()
                )
            } else {
                format!("{}s", track_render.seconds_until_start.round())
            };
            let (time_layout, time_layout_width) = self.layout_text_cached(&time_text, font_size);

            let width_ratio =
                available_width / (artist_layout_width + dot_layout_width + time_layout_width);
            if width_ratio <= 1.0 || !track_render.is_current {
                let (layout, _) = self.layout_text_cached(
                    &format!("{time_text}{dot_text}{artist_text}"),
                    font_size * width_ratio.clamp(0.8, 1.0),
                );
                self.draw_text(
                    &layout,
                    if width_ratio >= 1.0 {
                        text_start_right
                    } else {
                        text_start_left
                    },
                    text_height,
                    if width_ratio >= 1.0 {
                        Align::End
                    } else {
                        Align::Start
                    },
                    // Fade out when it gets too small, 0.6-0.4
                    text_brush.with_alpha(((width_ratio - 0.4) / 0.2) as f32),
                );
            } else {
                self.draw_text(
                    &time_layout,
                    start_x + 12.0,
                    text_height,
                    Align::Start,
                    text_brush,
                );
                self.draw_text(
                    &artist_layout,
                    text_start_right,
                    text_height,
                    Align::End,
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
    fn layout_text(&self, text: &str, font_size: f64) -> TextLayout {
        let font_weight = 700.0f32;
        let mut face = self.font.base_face.clone();
        if let Some(index) = self.font.weight_axis_index {
            let axis = &self.font.axes[index];
            face.set_variation(axis.tag, font_weight.clamp(axis.min_value, axis.max_value));
        }

        let font_size_px = (font_size * self.scale_factor) as f32;
        let scale = font_size_px / f32::from(face.units_per_em());
        let baseline = f32::from(face.ascender()) * scale;
        let space_advance = face
            .glyph_index(' ')
            .and_then(|gid| face.glyph_hor_advance(gid))
            .map_or_else(|| f32::from(face.units_per_em()) * 0.5, f32::from);
        let line_height_units = f32::from(face.ascender() + face.descender() + face.line_gap());

        let mut pen_x = 0.0f32;
        let mut glyphs = Vec::with_capacity(text.len());
        let mut previous: Option<GlyphId> = None;

        for ch in text.chars() {
            let glyph_id = face.glyph_index(ch);
            if let (Some(left), Some(right)) = (previous, glyph_id) {
                pen_x += {
                    face.tables().kern.map_or(0.0, |kern_table| {
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
                    })
                } * scale;
            }
            let advance_units = glyph_id
                .and_then(|gid| face.glyph_hor_advance(gid))
                .map_or(space_advance, f32::from);
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
            width: f64::from(pen_x),
            height: f64::from(line_height_units * scale),
            font_size: font_size_px,
            coords: self
                .font
                .axes
                .iter()
                .map(|axis| {
                    let weight = font_weight.clamp(axis.min_value, axis.max_value);
                    let delta = weight - axis.def_value;
                    if delta.abs() < f32::EPSILON {
                        return 0;
                    }
                    let range = if delta < 0.0 {
                        axis.def_value - axis.min_value
                    } else {
                        axis.max_value - axis.def_value
                    };
                    if range.abs() < f32::EPSILON {
                        return 0;
                    }
                    NormalizedCoordinate::from(delta / range).get()
                })
                .collect(),
        }
    }

    fn layout_text_cached(&mut self, text: &str, font_size: f64) -> (TextLayoutKey, f64) {
        // All numbers replaced with zero since numbers are same width
        let text_cleaned: String = text
            .chars()
            .map(|c| if c.is_ascii_digit() { '0' } else { c })
            .collect();
        let key = TextLayoutKey::new(&text_cleaned, font_size);
        if let Some(layout) = self.render_state.layout_cache.get(&key) {
            return (key, layout.width);
        }
        let layout = self.layout_text(text, font_size);
        let width = layout.width;
        self.render_state.layout_cache.insert(key.clone(), layout);
        (key, width)
    }

    /// Draw the text layout onto the scene.
    fn draw_text(
        &mut self,
        layout_key: &TextLayoutKey,
        pos_x: f64,
        pos_y: f64,
        horizontal_align: Align,
        brush: Color,
    ) {
        let layout = &self.render_state.layout_cache[layout_key];
        self.scene
            .draw_glyphs(&self.font.font_data)
            .font_size(layout.font_size)
            .normalized_coords(&layout.coords)
            .transform(Affine::translate((
                pos_x
                    - match horizontal_align {
                        Align::Start => 0.0,
                        Align::End => layout.width,
                    },
                pos_y - layout.height * 0.5,
            )))
            .hint(true)
            .brush(brush)
            .draw(Fill::EvenOdd, layout.glyphs.iter().copied());
    }

    fn render_playing_particles(
        &mut self,
        dt: f64,
        track: &Track,
        x: f64,
        height: f64,
        track_move_speed: f64,
        volume: Option<u8>,
    ) {
        let lightness_boost = 50.0;
        let Some(track_data) = TRACK_DATA_CACHE.get(&track.id) else {
            return;
        };
        let mut primary_colors: Vec<_> = track_data
            .primary_colors
            .iter()
            .map(|[r, g, b, _]| {
                [
                    (lerpf64(0.3, f64::from(*r) + lightness_boost, 255.0)).min(255.0) as u8,
                    (lerpf64(0.3, f64::from(*g) + lightness_boost, 210.0)).min(255.0) as u8,
                    (lerpf64(0.3, f64::from(*b) + lightness_boost, 160.0)).min(255.0) as u8,
                ]
            })
            .collect();
        if primary_colors.is_empty() {
            primary_colors.extend_from_slice(&[[100, 100, 100], [150, 150, 150], [200, 200, 200]]);
        }

        // Emit new particles while playing
        let mut emit_count = if track_move_speed.abs() > f64::EPSILON {
            self.particles.spawn_accumulator += dt * SPARK_EMISSION;
            let emit_count = self.particles.spawn_accumulator.floor() as u8;
            self.particles.spawn_accumulator -= f64::from(emit_count);
            emit_count
        } else {
            self.particles.spawn_accumulator = 0.0;
            0
        };

        // Spawn new particles, kill dead particles, update positions of others, then render them
        let spawn_offset = track_move_speed.signum() * 2.0;
        let horizontal_bias =
            (track_move_speed.abs().powf(0.2) * spawn_offset * 0.5).clamp(-3.0, 3.0);
        for particle in &mut self.particles.particles {
            particle.life -= dt;

            // Emit a new particle
            if emit_count > 0 && particle.life <= 0.0 {
                particle.position = [
                    x + spawn_offset,
                    height * lerpf64(fastrand::f64(), 0.05, 0.95),
                ];
                particle.velocity = [
                    fastrand::usize(SPARK_VELOCITY_X) as f64 * self.scale_factor * horizontal_bias,
                    fastrand::usize(SPARK_VELOCITY_Y) as f64 * -self.scale_factor,
                ];
                particle.color = fastrand::usize(0..primary_colors.len());
                particle.life = lerpf64(fastrand::f64(), SPARK_LIFETIME.start, SPARK_LIFETIME.end);
                emit_count -= 1;
            }
            if particle.life <= 0.0 {
                continue;
            }

            particle.velocity[1] += SPARK_GRAVITY * self.scale_factor * dt;
            particle.position[0] += particle.velocity[0] * dt;
            particle.position[1] += particle.velocity[1] * dt;

            let fade = (particle.life / SPARK_LIFETIME.end).clamp(0.0, 1.0);
            let length =
                lerpf64(fade, SPARK_LENGTH_RANGE.start, SPARK_LENGTH_RANGE.end) * self.scale_factor;
            let thickness = lerpf64(fade, SPARK_THICKNESS_RANGE.start, SPARK_THICKNESS_RANGE.end)
                * self.scale_factor;
            let rgb = primary_colors
                .get(particle.color)
                .unwrap_or(&[255, 210, 160]);
            self.scene.fill(
                Fill::EvenOdd,
                Affine::translate((particle.position[0], particle.position[1]))
                    * Affine::rotate(particle.velocity[1].atan2(particle.velocity[0]))
                    * Affine::translate((-length * 0.5, 0.0)),
                Color::from_rgba8(
                    rgb[0],
                    rgb[1],
                    rgb[2],
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

        // Line at the now playing position to denote the cutoff
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
                Affine::translate((line_x, 0.0)),
                line_color,
                None,
                &RoundedRect::new(0.0, 0.0, line_width, line_height, 100.0),
            );
            self.scene.fill(
                Fill::EvenOdd,
                Affine::translate((line_x, height - line_height)),
                line_color,
                None,
                &RoundedRect::new(0.0, 0.0, line_width, line_height, 100.0),
            );

            let icon_height = height * 0.333;
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
                    Affine::translate((line_x - icon_scale * 0.3, height * 0.5 - icon_scale * 0.5))
                        * Affine::scale(icon_scale / play_icon_width),
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
                    Affine::translate((line_x, 0.0)),
                    Color::from_rgb8(150, 150, 150),
                    None,
                    &RoundedRect::new(0.0, 0.0, line_width, height, 100.0),
                );
            }
            self.scene.fill(
                Fill::EvenOdd,
                Affine::translate((line_x, height * (1.0 - volume))),
                line_color,
                None,
                &RoundedRect::new(0.0, 0.0, line_width, height * volume, 100.0),
            );
        }
    }
}
