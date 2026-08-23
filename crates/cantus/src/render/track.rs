use crate::render::{
    Globals, PANEL_START,
    sdf::{PillSample, VISIBLE_ALPHA, avalanche, pill_margin, pill_sheen, sample_pill, sd_capsule_box, sd_rounded_box, sd_star, simplex_noise},
};
use core::f32::consts::{FRAC_PI_2, TAU};
use isthmus::{
    Fragment, Image, Quad, TextFragment, Unorm8x4,
    float::FloatExt,
    glam::{UVec2, Vec2, Vec3, Vec4, uvec2, vec2, vec3},
    spirv_std::arch::{Derivative, kill},
    text::{self, TextStyle},
};

/// Number of colors extracted from album artwork.
pub const PALETTE_COLORS: usize = 4;
/// Visual width, in pixels, of rating and playlist icons before hover growth.
const ICON_WIDTH: f32 = 21.6;
/// Center-to-center icon spacing for rating stars and playlist artwork.
const ICON_SPACING: f32 = 18.0;
const ICON_REACTION_RADIUS: f32 = ICON_WIDTH * 2.5;
/// Stars in the rating row; each holds a half-star either side of its centre.
const STAR_RATINGS: usize = 5;
const MAX_RENDER_INSTANCES: usize = 32;
const TITLE_STYLE: TextStyle = TextStyle::new(16.0, 700.0);
const DETAILS_STYLE: TextStyle = TextStyle::new(14.0, 700.0);
const PLAYLIST_EXPANSION_DURATION: f32 = 1.0 / 6.0;

/// Per-track character, every field normalised to 0..1 on arrival.
#[repr(C)]
#[derive(Clone, Copy, Default, isthmus::ShaderData)]
#[cfg_attr(not(target_arch = "spirv"), derive(serde::Deserialize))]
pub struct AudioFeatures {
    energy: f32,
    danceability: f32,
    acousticness: f32,
    tempo: f32,
    valence: f32,
    instrumentalness: f32,
    loudness: f32,
}

impl AudioFeatures {
    /// Overall agitation of the plasma field.
    fn turbulence(self) -> f32 {
        self.energy * 0.55 + self.danceability * 0.25 + self.loudness * 0.2
    }

    /// Beat pulse, sharpened so it reads as a kick rather than a sine.
    fn beat(self, time: f32) -> f32 {
        let pulse = (time * self.tempo * 5.0 * TAU).sin() * 0.5 + 0.5;
        pulse * pulse * self.danceability * (0.025 + self.energy * 0.055)
    }

    /// Time scaled so energetic, fast tracks flow faster, offset per track by `seed`.
    fn flow_time(self, time: f32, seed: f32) -> f32 {
        let pace = ((self.tempo - 0.2) * 2.5).saturate();
        time * (0.12 + self.energy * 0.25 + pace * 0.12) + seed
    }

    /// Clamps freshly fetched features into the ranges the shader assumes.
    #[must_use]
    pub fn normalized(self) -> Self {
        Self {
            energy: self.energy.saturate(),
            danceability: self.danceability.saturate(),
            acousticness: self.acousticness.saturate(),
            tempo: (self.tempo / 300.0).saturate(),
            valence: self.valence.saturate(),
            instrumentalness: self.instrumentalness.saturate(),
            loudness: ((self.loudness + 30.0) / 30.0).saturate(),
        }
    }
}

/// Where one queued track sits on the timeline this frame.
#[derive(Clone, Copy)]
struct TrackLayout {
    queue_index: usize,
    start_ms: f32,
    x: f32,
    width: f32,
}

fn icon_center(pill: Quad, panel_height: f32, count: usize, index: usize, expansion: f32, secondary: bool) -> Vec2 {
    let center = vec2(pill.center.x, PANEL_START + panel_height * 0.975 - 1.0 + f32::from(secondary) * ICON_SPACING * expansion);
    center + vec2((index as f32 - (count.saturating_sub(1)) as f32 * 0.5) * ICON_SPACING * expansion, 0.0)
}

fn icon_geometry(center: Vec2, frame: &Globals) -> (Quad, f32) {
    let mouse_distance = center.distance(frame.info.mouse_pos);
    let proximity = mouse_distance.smoothstep(ICON_REACTION_RADIUS, ICON_WIDTH * 0.25) * frame.info.mouse_pressure.clamp(0.0, 1.0);
    let pixel_radius = ICON_WIDTH * 0.5 * (1.05 + 0.63 * proximity);
    let x_push = (center.x - frame.info.mouse_pos.x) * proximity * 0.5;
    let quad = Quad::new(center + vec2(x_push, 0.0), Vec2::splat(pixel_radius * 2.0 + 14.0), Vec2::from_angle(x_push * 0.01));
    (quad, mouse_distance)
}

fn over_icon(base: Vec4, color: Vec3, shape: f32, alpha: f32) -> Vec4 {
    let mask = (0.5 - shape).saturate();
    let shadow = (-shape.max(0.0) * 0.5).exp();
    let bevel = 1.0 - shape.smoothstep(0.0, -5.0);
    let layer = ((color + bevel * bevel * 0.045) * mask * alpha).extend(mask.max(shadow * shadow * 0.2) * alpha);
    base * (1.0 - layer.w) + layer
}

fn plasma_field(uv: Vec2, swatch: Vec4, x: f32, y: f32, phase: f32) -> Vec4 {
    let wave = (uv.dot(vec2(x, y)) + phase).sin() * 0.5 + 0.5;
    let weight = (0.12 + wave * wave) * (0.25 + swatch.w * 3.0);
    (swatch.truncate() * weight).extend(weight)
}

fn hash(point: Vec2, seed: f32) -> f32 {
    let cell = uvec2(point.x as i32 as u32, point.y as i32 as u32);
    let value = avalanche(cell ^ UVec2::splat(seed.to_bits() * 2_654_435_761));
    value.x as f32 * 2.328_306_4e-10
}

fn stable_seed(value: &str) -> f32 {
    let key = value
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3));
    key as u32 as f32 * 2.328_306_4e-10
}

/// Twinkling points for acoustic tracks.
fn speckle(pixel: Vec2, time: f32, seed: f32, effects: AudioFeatures) -> f32 {
    let drift = vec2(0.16 + seed.fract() * 0.08, 0.055 + (seed * 0.7).sin() * 0.025);
    let uv = pixel / (8.0 - effects.acousticness) + time * (0.35 + effects.energy * 0.55) * drift;
    let cell = uv.floor();
    let phase = hash(vec2(cell.y, cell.x), seed + 2.71);
    let center = vec2(phase, (phase * 7.13).fract()) * 0.56 - 0.28;
    let twinkle = time * (0.7 + phase * 0.9 + effects.energy * 0.8) + phase * TAU;
    hash(cell, seed).smoothstep(0.985 - effects.acousticness * 0.09, 1.0)
        * (1.0 - (uv - cell - 0.5 - center).length().smoothstep(0.06, 0.28))
        * (twinkle.sin() * 0.5 + 0.5)
        * (0.12 + effects.acousticness * 0.48)
}

/// Caustic web for instrumental tracks.
fn caustics(p: Vec2, time: f32, seed: f32, effects: AudioFeatures) -> f32 {
    if effects.instrumentalness <= 1.0 / 256.0 {
        return 0.0;
    }
    let flow = time * (0.5 + effects.energy * 0.35);
    let warp = vec2((p.y * 1.9 + flow).sin(), (p.x * 1.5 - flow * 0.8).sin()) * 0.35;
    let drift = vec2(flow * 0.05, flow * -0.04) + seed;
    let ridge = |q: Vec2| {
        let line = (1.0 - simplex_noise(q).abs() * 2.0).max(0.0);
        line * line * line
    };
    let web = ridge(p * 0.7 + warp + drift).max(ridge(p * 1.1 - warp - drift * 0.8));
    web * effects.instrumentalness * 0.06
}

#[isthmus::paint]
mod host {
    use super::*;
    use crate::{
        app::music::{CondensedPlaylist, PlaybackState, TRACK_SPACING_MS, Timeline, Track},
        render::{Frame, GAP, particles::Particles},
    };
    use arrayvec::ArrayVec;
    use isthmus::{Rect, Response};

    fn shape_text(frame: &mut Frame, track: &Track, start_ms: f32) -> (text::ShapedLine, text::ShapedLine) {
        let without_suffix = track.name.split_once(" -").map_or(track.name.as_str(), |(name, _)| name);
        let title = without_suffix.split_once('(').map_or(without_suffix, |(name, _)| name).trim();
        let title = if title.is_empty() { track.name.trim() } else { title };
        let seconds = (start_ms / 1000.0).abs();
        let time = if seconds >= 60.0 {
            let whole_seconds = seconds as u32;
            format!("{}m{}s", whole_seconds / 60, whole_seconds % 60)
        } else {
            format!("{}s", seconds.round())
        };
        let details = format!("{time}\u{2004}•\u{2004}{}", track.artist);
        let mut text = frame.text();
        (text.line(title, TITLE_STYLE).shape(), text.line(&details, DETAILS_STYLE).shape())
    }

    fn paint_text(frame: &mut Frame, line: text::Line, pill: Quad, visibility: f32) {
        if line.is_empty() {
            return;
        }
        // GPU: Track label.
        frame.paint_text(
            line.expanded(20.0).translated(pill.center - pill.size * 0.5),
            |text: TextFragment, frame: Globals, pill: Quad, visibility: f32| {
                let surface = sample_pill(pill, text.pixel, &frame);
                let image_center = vec2(surface.size.x - surface.size.y, 0.0) + Vec2::splat(surface.size.y * 0.5);
                let alpha = text.alpha_at(pill.center - pill.size * 0.5 + surface.refracted)
                    * sd_capsule_box(surface.refracted - image_center, 0.0, surface.size.y * 0.5).smoothstep(2.0, 18.0)
                    * surface.mask
                    * visibility;
                text.color(alpha)
            },
        );
    }

    fn show_track(track: &mut Track, mut layout: TrackLayout, playlists: &mut [CondensedPlaylist], timeline: &Timeline, particles: &mut Particles, frame: &mut Frame) -> Response {
        let height = frame.services.config.height;
        let playlist_expansion = track.runtime.playlist_expansion.smoothstep(0.0, 1.0);
        let track_text = (layout.width > height + 26.0 || playlist_expansion > 0.0).then(|| shape_text(frame, track, layout.start_ms));
        if playlist_expansion > 0.0
            && let Some((title, details)) = &track_text
        {
            // Grow toward the width the labels would need at full size.
            let target = title.width.max(details.width) + height + 20.0;
            let extra_width = (target - layout.width).max(0.0) * playlist_expansion;
            layout.x -= extra_width * 0.5;
            layout.width += extra_width;
        }
        let show_details = layout.width > frame.services.config.height;

        let (mut primary_playlists, mut secondary_playlists) = (ArrayVec::<_, MAX_RENDER_INSTANCES>::new(), ArrayVec::<_, MAX_RENDER_INSTANCES>::new());
        let mut rating = None;
        if show_details && let Some(track_id) = track.id {
            for (index, playlist) in playlists.iter().enumerate().filter(|(_, playlist)| playlist.rating_index.is_none()) {
                let group = if playlist.tracks.contains(&track_id) {
                    &mut primary_playlists
                } else if playlist_expansion > 0.0 {
                    &mut secondary_playlists
                } else {
                    continue;
                };
                let _ = group.try_push(index);
            }
            if frame.services.config.ratings_enabled {
                rating = Some(
                    playlists
                        .iter()
                        .find_map(|playlist| playlist.rating_index.filter(|_| playlist.tracks.contains(&track_id)))
                        .map_or(0, |rating| i32::from(rating) + 1),
                );
            }
        }

        let (primary_count, secondary_count) = (primary_playlists.len(), secondary_playlists.len());
        let stars = rating.map_or(0, |_| STAR_RATINGS);
        let primary_icons = stars as f32 + primary_count as f32;
        let primary_alpha = if primary_icons > 0.0 {
            let threshold = ICON_SPACING * 1.05 * primary_icons;
            layout.width.smoothstep(threshold, threshold + ICON_SPACING)
        } else {
            0.0
        };
        let pill_rect = Rect::new(layout.x, PANEL_START, layout.x + layout.width.max(height), PANEL_START + height);
        let pill: Quad = pill_rect.into();
        let colors = track.runtime.art.palette();
        let art = track.runtime.art.ready();
        let visibility = layout.width.smoothstep(height, height + 26.0).max(f32::from(layout.start_ms <= 0.0));
        let seed = stable_seed(&track.uri);
        let effects = track.runtime.audio_features.ready().copied().unwrap_or_default();

        let body = frame.drag(pill_rect);
        let mut hovered = body.hovered();
        if body.clicked() && track.id.is_some() {
            let natural_start = frame.globals.playhead_x + layout.start_ms * frame.globals.px_per_ms;
            let fraction = if frame.pointer().x < frame.services.config.history_width + 40.0 {
                0.0
            } else {
                (frame.pointer().x - natural_start) / (track.duration_ms as f32 * frame.globals.px_per_ms)
            };
            frame.services.music.seek(timeline, layout.queue_index, track.duration_ms, fraction);
        }
        // GPU: Track pill.
        let margin = pill_margin(frame.globals);
        let render_quad = Quad::from_min_max(
            pill.center - pill.size * 0.5 - Vec2::splat(margin),
            pill.center + pill.size * 0.5 + vec2(margin, margin + primary_alpha * 7.0 + ICON_SPACING * playlist_expansion),
        );
        let secondary_icons = secondary_count as f32;
        frame.paint_quad(
            render_quad,
            |fragment: Fragment,
             frame: Globals,
             pill: Quad,
             primary_icons: f32,
             secondary_icons: f32,
             primary_alpha: f32,
             playlist_expansion: f32,
             colors: [Unorm8x4; PALETTE_COLORS],
             visibility: f32,
             seed: f32,
             effects: AudioFeatures| {
                let bottom = pill.center.y + pill.size.y * 0.5;
                let primary_width = ((primary_icons - 1.0).max(0.0) * ICON_SPACING + ICON_WIDTH * 0.7).max(ICON_WIDTH);
                let primary_depth = primary_alpha * 7.0;
                let primary_half_size = vec2(primary_width * 0.5, primary_depth * 0.5 + 2.0);
                let primary_center = vec2(pill.center.x, bottom + primary_depth * 0.5 - 2.0);
                let primary_support = sd_rounded_box(fragment.pixel - primary_center, primary_half_size, primary_half_size.y);
                let primary_mouse_support = sd_rounded_box(frame.info.mouse_pos - primary_center, primary_half_size, primary_half_size.y);
                let primary_surface = sample_pill(pill, fragment.pixel, &frame).union(primary_support, primary_mouse_support, 9.0, primary_alpha);

                let secondary_expansion = if secondary_icons > 0.0 { playlist_expansion } else { 0.0 };
                let secondary_width = ((secondary_icons - 1.0).max(0.0) * ICON_SPACING + ICON_WIDTH * 0.7).max(ICON_WIDTH);
                let secondary_depth = secondary_expansion * ICON_SPACING;
                let secondary_half_size = vec2(secondary_width * 0.5, secondary_depth * 0.5 + 2.0);
                let secondary_center = vec2(pill.center.x, bottom + primary_depth + secondary_depth * 0.5 - 2.0);
                let secondary_support = sd_rounded_box(fragment.pixel - secondary_center, secondary_half_size, secondary_half_size.y);
                let secondary_mouse_support = sd_rounded_box(frame.info.mouse_pos - secondary_center, secondary_half_size, secondary_half_size.y);
                let surface = primary_surface.union(secondary_support, secondary_mouse_support, 9.0, secondary_expansion);
                shade_track(fragment.pixel, &frame, colors, visibility, seed, effects, surface)
            },
        );
        if let Some(art) = art {
            let image = &art.image;
            let center = pill.center + vec2((pill.size.x - pill.size.y) * 0.5, 0.0);
            let quad = Quad::new(center, Vec2::splat(pill.size.y), Vec2::X).expanded(pill_margin(frame.globals));
            // GPU: Track album artwork.
            frame.paint_quad(quad, |fragment: Fragment, frame: Globals, pill: Quad, image: Image, visibility: f32| {
                let surface = sample_pill(pill, fragment.pixel, &frame);
                let image_center = vec2(surface.size.x - surface.size.y, 0.0) + Vec2::splat(surface.size.y * 0.5);
                let offset = surface.local - image_center;
                let radius = surface.size.y * 0.5 + surface.bulge() * 0.5;
                let texture = image.sample(offset / (radius * 2.0) + 0.5);
                let alpha = texture.w * (1.0 - (offset.length() - radius).smoothstep(-4.0, 0.0)) * (1.0 - surface.distance.smoothstep(-0.5, 0.5)) * visibility;
                (texture.truncate() * alpha).extend(alpha)
            });
        }
        let (left, right) = (18.0, layout.width - height - 8.0);
        if right > left
            && let Some((title, details)) = track_text
        {
            let title = frame.text().fit(&title, (height * 0.26).floor(), left..right);
            paint_text(frame, title, pill, visibility);
            let details = frame.text().fit(&details, (height * 0.57).floor(), left..right);
            paint_text(frame, details, pill, visibility);
        }

        let alpha = primary_alpha.max(playlist_expansion * f32::from(primary_icons > 0.0));
        if let Some(mut rating) = rating
            && alpha > 0.0
        {
            for index in 0..STAR_RATINGS as u32 {
                let center = icon_center(pill, height, stars + primary_count, index as usize, 1.0, false);
                frame.input_region(Rect::from_center(center, Vec2::splat(ICON_REACTION_RADIUS)));
                let response = frame.interact(Rect::from_center(center, Vec2::splat(ICON_WIDTH * 0.5)));
                hovered |= response.hovered();
                let right_half = frame.pointer().x >= center.x;
                if response.hovered() {
                    rating = index as i32 * 2 + 1 + i32::from(right_half);
                }
                if response.clicked()
                    && let Some(track_id) = track.id
                {
                    frame.services.music.rate_track(playlists, track_id, index as u8 * 2 + u8::from(right_half));
                    particles.burst(frame.pointer(), frame.globals.info.time);
                }
                let (quad, _) = icon_geometry(center, frame.globals);
                let fill = ((rating as f32 - index as f32 * 2.0) * 0.5).saturate();
                // GPU: Track rating star.
                frame.paint_quad(quad, |fragment: Fragment, _frame: Globals, fill: f32, alpha: f32| {
                    let split = fragment.uv.x - fill;
                    let unselected = (split / split.fwidth() + 0.5).saturate();
                    let point = (fragment.uv * 2.0 - 1.0) * 18.0;
                    let star = sd_star(point, 5.6, 3.58) - 1.12;
                    over_icon(Vec4::ZERO, vec3(1.0, 0.85, 0.2).lerp(Vec3::splat(0.33), unselected), star, alpha)
                });
            }
        }
        let mut toggled_playlist = None;
        if let Some(track_id) = track.id {
            let icons = primary_playlists.into_iter().chain(secondary_playlists).map(|index| &playlists[index]);
            for (index, playlist) in icons.enumerate() {
                let primary = index < primary_count;
                let (icon, count, alpha, expansion) = if primary {
                    (index + stars, stars + primary_count, alpha, 1.0)
                } else {
                    (index - primary_count, secondary_count, playlist_expansion, playlist_expansion)
                };
                if alpha > 0.0 {
                    let center = icon_center(pill, height, count, icon, expansion, !primary);
                    frame.input_region(Rect::from_center(center, Vec2::splat(ICON_REACTION_RADIUS)));
                    let rect = Rect::from_center(center, Vec2::splat(ICON_WIDTH * 0.5));
                    let response = frame.interact(rect);
                    hovered |= response.hovered();
                    if response.clicked() {
                        toggled_playlist = Some(playlist.id);
                    }
                    let (quad, mouse_distance) = icon_geometry(center, frame.globals);
                    if let Some(art) = playlist.art.ready() {
                        let image = &art.image;
                        let desaturation = if primary || (frame.globals.info.mouse_pressure > 0.0 && mouse_distance <= ICON_WIDTH * 0.5) {
                            0.0
                        } else {
                            0.2
                        };
                        // GPU: Track playlist icon.
                        frame.paint_quad(quad, |fragment: Fragment, _frame: Globals, image: Image, alpha: f32, desaturation: f32| {
                            let texture = image.sample(fragment.uv);
                            over_icon(
                                Vec4::ZERO,
                                texture.truncate().lerp(Vec3::splat(0.24), desaturation),
                                sd_capsule_box((fragment.uv * 2.0 - 1.0) * 18.0, 0.0, 6.7),
                                alpha,
                            )
                        });
                    }
                }
            }
            if let Some(playlist_id) = toggled_playlist {
                frame.services.music.toggle_playlist(playlists, track_id, playlist_id);
                particles.burst(frame.pointer(), frame.globals.info.time);
            }
        }
        track.runtime.playlist_expansion.approach(
            f32::from(hovered && show_details && visibility >= 1.0),
            frame.delta_time.min(0.1) / PLAYLIST_EXPANSION_DURATION,
        );
        body
    }

    pub fn show(frame: &mut Frame, playback: &mut PlaybackState, particles: &mut Particles) {
        if playback.queue.is_empty() {
            return;
        }
        let current_ms = playback.timeline.queue_start_ms;
        let current_index = playback.timeline.track_at_playhead(&playback.queue).map(|(index, _)| index);

        let mut current_track = None;
        let (px_per_ms, playhead_x) = (frame.globals.px_per_ms, frame.globals.playhead_x);
        let end_ms = (frame.services.config.timeline_future_minutes - frame.services.config.timeline_past_minutes) * 60_000.0;
        let gap = TRACK_SPACING_MS * px_per_ms;
        let width_trim = (GAP - gap).max(0.0);
        let end_x = frame.services.config.history_width + frame.services.config.timeline_future_minutes * 60_000.0 * px_per_ms + width_trim;
        let (history_width, panel_height) = (frame.services.config.history_width, frame.services.config.height);
        // Walked newest first, so compact history pills stack leftward from the playhead.
        let mut compact_slot = 0;
        let mut transition = 0.0;
        let mut queue_end_ms = current_ms + playback.queue.iter().map(Track::queue_span_ms).sum::<f32>();
        let mut layouts = ArrayVec::<_, MAX_RENDER_INSTANCES>::new();
        for (queue_index, track) in playback.queue.iter().enumerate().rev() {
            queue_end_ms -= track.queue_span_ms();
            let natural_start = playhead_x + queue_end_ms * px_per_ms;
            let natural_end = natural_start + track.duration_ms as f32 * px_per_ms;
            let mut layout = TrackLayout {
                queue_index,
                start_ms: queue_end_ms,
                x: 0.0,
                width: panel_height,
            };
            if layout.start_ms > end_ms {
                layout.width = 0.0;
            } else if natural_end >= history_width + panel_height {
                layout.x = natural_start.max(history_width);
                layout.width = (natural_end.min(end_x) - layout.x - width_trim).max(0.0);
            } else if natural_end >= history_width {
                transition = (history_width + panel_height - natural_end) / panel_height;
                layout.x = natural_end - panel_height;
            } else {
                let right = history_width - gap - (compact_slot as f32 + transition) * panel_height * 0.55;
                compact_slot += 1;
                layout.x = right - panel_height;
            }
            if !layouts.is_full() && layout.width > 0.0 && layout.x + layout.width > 0.0 {
                layouts.push(layout);
            }
            if current_index == Some(queue_index) {
                current_track = Some(layout);
            }
        }
        layouts.reverse();
        let mut current_response = None;
        for layout in layouts {
            let track = &mut playback.queue[layout.queue_index];
            let response = show_track(track, layout, &mut playback.playlists, &playback.timeline, particles, frame);
            if current_index == Some(layout.queue_index) {
                current_response = Some(response);
            }
        }
        let dragging = current_response.is_some_and(|response| response.dragging());
        let drag_offset_ms = current_response
            .filter(Response::dragging)
            .map_or(0.0, |response| (frame.globals.info.mouse_pos.x - response.drag_origin.x) / frame.globals.px_per_ms);
        playback.update_timeline(drag_offset_ms, dragging, frame.delta_time);
        if current_response.is_some_and(|response| response.released())
            && dragging
            && let Some(layout) = current_track
            && playback.queue[layout.queue_index].id.is_some()
        {
            let duration_ms = playback.queue[layout.queue_index].duration_ms;
            let natural_start = frame.globals.playhead_x + layout.start_ms * frame.globals.px_per_ms;
            let fraction = (frame.globals.playhead_x.max(layout.x) - natural_start) / (duration_ms as f32 * frame.globals.px_per_ms);
            frame.services.music.seek(&playback.timeline, layout.queue_index, duration_ms, fraction);
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
pub use host::show;

fn shade_track(pixel: Vec2, frame: &Globals, colors: [Unorm8x4; PALETTE_COLORS], visibility: f32, seed: f32, effects: AudioFeatures, surface: PillSample) -> Vec4 {
    let local_uv = surface.uv();
    let local_centered = local_uv - 0.5;

    if surface.alpha * visibility <= VISIBLE_ALPHA {
        kill();
    }

    // Weight overlapping wave fields by each palette colour's prevalence.
    let flow_time = effects.flow_time(frame.info.time, seed);
    let beat = effects.beat(frame.info.time);
    let turbulence = effects.turbulence();
    let lens_warp = (1.0 + surface.distance.min(0.0) / 120.0).saturate();
    let deformation = local_centered * lens_warp * lens_warp * 0.6 + surface.ripple;
    let frequency = (surface.size.x / surface.size.y * (0.5 + seed.fract() * 0.12 + turbulence * 0.18)).max(1.7);
    let field_uv = (local_uv.clamp(Vec2::ZERO, Vec2::ONE) - deformation * 0.08) * vec2(frequency, 1.6);
    let warped_uv = field_uv
        + vec2(
            (field_uv.y * 2.7 + flow_time).sin() + (field_uv.x * 1.3 - flow_time * 0.7).cos(),
            (field_uv.x * 2.3 - flow_time * 0.8).cos() + (field_uv.y * 1.7 + flow_time * 0.6).sin(),
        ) * (0.14 + turbulence * 0.2 + beat);
    let phase = seed + FRAC_PI_2;
    let swatch = |index: usize| colors[index].to_vec4();
    let weighted = plasma_field(warped_uv, swatch(0), 2.1, 0.7, flow_time)
        + plasma_field(warped_uv, swatch(1), 0.6, -2.4, phase - flow_time * 0.8)
        + plasma_field(warped_uv, swatch(2), -1.5, 1.9, flow_time * 0.65 + 2.0)
        + plasma_field(warped_uv, swatch(3), 2.4, 1.6, phase - flow_time * 0.55);
    let mut color = weighted.truncate() / weighted.w;

    let luma = color.dot(vec3(0.2126, 0.7152, 0.0722));
    let played = pixel.x.smoothstep(frame.playhead_x + 3.0, frame.playhead_x - 3.0);
    color = Vec3::splat(luma).lerp(color, 1.55 + effects.valence * 0.4).clamp(Vec3::splat(0.035), Vec3::splat(0.92))
        * (0.52 / luma.max(0.001)).min(1.0)
        * (0.96 + effects.valence * 0.06 + beat * 0.5)
        * (0.84 + surface.refracted_uv().y.smoothstep(0.45, 1.0) * 0.1)
        * (1.0 - 0.4 * played);

    color += colors[3].to_vec3().lerp(Vec3::ONE, 0.25) * speckle(surface.local, frame.info.time, seed, effects);
    color += color.lerp(Vec3::ONE, 0.75) * caustics(surface.local / surface.size.y, frame.info.time, seed, effects);

    color += color.lerp(Vec3::ONE, 0.32) * pill_sheen(surface.distance);
    color = color.lerp(color * 1.5 + 0.1, surface.ripple_flash);
    let output = surface.color(color) * visibility;
    if output.w <= 0.0 {
        kill();
    }
    output
}
