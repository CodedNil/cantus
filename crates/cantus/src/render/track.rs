use crate::render::{
    Globals, PANEL_START,
    sdf::{Pill, Surface, VISIBLE_ALPHA, avalanche, pill_sheen, sd_capsule_box, sd_star, simplex_noise},
    text::{self, TextStyle},
};
use core::f32::consts::{FRAC_PI_2, TAU};
use isthmus::{
    ImageHandle, Quad, Unorm8x4,
    float::FloatExt,
    glam::{UVec2, Vec2, Vec3, Vec4, uvec2, vec2, vec3},
    spirv_std::arch::{Derivative, kill},
};

#[cfg(not(target_arch = "spirv"))]
use {
    crate::{
        app::music::{AlbumArt, CondensedPlaylist, IMAGE_SIZE, PlaybackState, TRACK_SPACING_MS, Timeline, Track, TrackId, playlist_icons},
        render::{Frame, GAP, particles::Particles},
    },
    arrayvec::ArrayVec,
    isthmus::{ImageArray, Rect},
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
const MAX_TEXTURE_IMAGES: usize = 32;
const MAX_RENDER_INSTANCES: usize = 32;
const TITLE_STYLE: TextStyle = TextStyle::new(16.0, 700.0);
const DETAILS_STYLE: TextStyle = TextStyle::new(14.0, 700.0);
const DETAIL_FADE_DURATION: f32 = 0.2;
const PLAYLIST_EXPANSION_DURATION: f32 = 1.0 / 6.0;

#[cfg(not(target_arch = "spirv"))]
#[derive(Default)]
pub struct TrackView {
    images: Option<ImageArray>,
}

/// Per-track character, every field normalised to 0..1 on arrival.
#[isthmus::data]
#[derive(Default)]
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

fn icon_center(pill: Pill, panel_height: f32, count: usize, index: usize, expansion: f32, secondary: bool) -> Vec2 {
    let center = vec2(
        pill.x + pill.width * 0.5,
        PANEL_START + panel_height * 0.975 - 1.0 + f32::from(secondary) * ICON_SPACING * expansion,
    );
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
    let amount = effects.acousticness;
    let drift = vec2(0.16 + seed.fract() * 0.08, 0.055 + (seed * 0.7).sin() * 0.025);
    let uv = pixel / (8.0 - amount) + time * (0.35 + effects.energy * 0.55) * drift;
    let cell = uv.floor();
    let phase = hash(vec2(cell.y, cell.x), seed + 2.71);
    let center = vec2(phase, (phase * 7.13).fract()) * 0.56 - 0.28;
    let twinkle = time * (0.7 + phase * 0.9 + effects.energy * 0.8) + phase * TAU;
    hash(cell, seed).smoothstep(0.985 - amount * 0.09, 1.0)
        * (1.0 - (uv - cell - 0.5 - center).length().smoothstep(0.06, 0.28))
        * (twinkle.sin() * 0.5 + 0.5)
        * (0.12 + amount * 0.48)
}

/// Caustic web for instrumental tracks.
fn caustics(p: Vec2, time: f32, seed: f32, effects: AudioFeatures) -> f32 {
    let amount = effects.instrumentalness;
    if amount <= 1.0 / 256.0 {
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
    web * amount * 0.06
}

#[isthmus::paint]
impl TrackView {
    fn image(&mut self, url: Option<&str>, art: Option<&AlbumArt>) -> ImageHandle {
        let (Some(url), Some(art)) = (url, art) else {
            return ImageHandle::NONE;
        };
        self.images.as_mut().and_then(|images| images.cache(url, [IMAGE_SIZE; 2], &art.pixels)).unwrap_or_default()
    }

    fn playlists(track_id: TrackId, playlists: &[CondensedPlaylist], include_secondary: bool) -> impl Iterator<Item = &CondensedPlaylist> {
        playlist_icons(track_id, playlists, true).chain(playlist_icons(track_id, playlists, false).filter(move |_| include_secondary))
    }

    fn display_title(track: &Track) -> &str {
        let without_suffix = track.name.split_once(" -").map_or(track.name.as_str(), |(name, _)| name);
        let title = without_suffix.split_once('(').map_or(without_suffix, |(name, _)| name).trim();
        if title.is_empty() { track.name.trim() } else { title }
    }

    fn track_details(track: &Track, start_ms: f32) -> String {
        let seconds = (start_ms / 1000.0).abs();
        let time = if seconds >= 60.0 {
            let whole_seconds = seconds as u32;
            format!("{}m{}s", whole_seconds / 60, whole_seconds % 60)
        } else {
            format!("{}s", seconds.round())
        };
        format!("{time}\u{2004}•\u{2004}{}", track.artist)
    }

    fn show_track(&mut self, track: &mut Track, mut layout: TrackLayout, playlists: &mut [CondensedPlaylist], timeline: &Timeline, particles: &mut Particles, frame: &mut Frame) {
        let height = frame.services.config.height;
        let playlist_expansion = track.runtime.playlist_expansion.smoothstep(0.0, 1.0);
        let track_text = (layout.width > height + 26.0 || playlist_expansion > 0.0).then(|| {
            let details = Self::track_details(track, layout.start_ms);
            (frame.text(Self::display_title(track), TITLE_STYLE).shape(), frame.text(&details, DETAILS_STYLE).shape())
        });
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
        track.runtime.detail_alpha.approach(f32::from(show_details), frame.delta_time / DETAIL_FADE_DURATION);
        let detail_alpha = track.runtime.detail_alpha;

        let (mut primary_count, mut secondary_count) = (0, 0);
        let mut rating = -1;
        if show_details && let Some(track_id) = track.id {
            for playlist in Self::playlists(track_id, playlists, playlist_expansion > 0.0) {
                let primary = playlist.tracks.contains(&track_id);
                primary_count += u32::from(primary);
                secondary_count += u32::from(!primary);
            }
            if frame.services.config.ratings_enabled {
                rating = playlists
                    .iter()
                    .find_map(|playlist| playlist.rating_index.filter(|_| playlist.tracks.contains(&track_id)))
                    .map_or(0, |rating| i32::from(rating) + 1);
            }
        }

        let stars = if rating >= 0 { STAR_RATINGS } else { 0 };
        let primary_icons = stars as f32 + primary_count as f32;
        track.runtime.icon_alpha.approach(
            f32::from(primary_icons > 0.0 && layout.width >= ICON_SPACING * 1.05 * primary_icons),
            frame.delta_time / DETAIL_FADE_DURATION,
        );
        let primary_alpha = track.runtime.icon_alpha;
        let pill = Pill::new(layout.x, PANEL_START, layout.width.max(height));
        let colors = track.runtime.art.palette();
        let image = self.image(track.image.as_deref(), track.runtime.art.ready());
        let visibility = detail_alpha.max(f32::from(layout.start_ms <= 0.0));
        let seed = stable_seed(&track.uri);
        let effects = track.runtime.audio_features.ready().copied().unwrap_or_default();

        let body = frame.interact(pill.rect(height));
        let mut hovered = body.hovered();
        if body.pressed() {
            frame.enable_drag();
        }
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
        frame.paint_quad(
            pill.quad(&frame.globals),
            |frame: Globals,
             fragment: isthmus::Fragment,
             pill: Pill,
             colors: [Unorm8x4; PALETTE_COLORS],
             image: ImageHandle,
             visibility: f32,
             seed: f32,
             effects: AudioFeatures| {
                let surface = pill.sample(fragment.pixel, &frame);
                let image_center = vec2(surface.size.x - surface.size.y, 0.0) + Vec2::splat(surface.size.y * 0.5);
                let offset = surface.local - image_center;
                let mut album = Vec4::ZERO;
                if offset.abs().max_element() < surface.size.y {
                    let radius = surface.size.y * 0.5 + surface.bulge() * 0.5;
                    let image_distance = offset.length() - radius;
                    if let Some(texture) = fragment.sample_image(image, offset / (radius * 2.0) + 0.5) {
                        album = texture;
                        album.w *= (1.0 - image_distance.smoothstep(-4.0, 0.0)) * (1.0 - surface.distance.smoothstep(-0.5, 0.5));
                    }
                }
                shade_track(fragment.pixel, &frame, colors, visibility, seed, effects, surface, album)
            },
        );
        let (left, right) = (18.0, layout.width - height - 8.0);
        if right > left
            && let Some((title, details)) = track_text
        {
            let title = frame.fit_text(&title, (height * 0.26).floor(), left..right);
            Self::paint_track_text(frame, title, pill, visibility);
            let details = frame.fit_text(&details, (height * 0.57).floor(), left..right);
            Self::paint_track_text(frame, details, pill, visibility);
        }

        let primary_alpha = primary_alpha.max(playlist_expansion * f32::from(primary_icons > 0.0));
        if rating >= 0 && primary_alpha > 0.0 {
            for index in 0..STAR_RATINGS as u32 {
                let center = icon_center(pill, height, stars + primary_count as usize, index as usize, 1.0, false);
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
                    particles.burst(frame);
                }
                let (quad, _) = icon_geometry(center, &frame.globals);
                // GPU: Track rating star.
                frame.paint_quad(quad, |_frame: Globals, fragment: isthmus::Fragment, rating: i32, index: u32, primary_alpha: f32| {
                    let radius = (fragment.size.x - 14.0) * 0.5;
                    let fill = ((rating as f32 - index as f32 * 2.0) * 0.5).saturate();
                    let split = fragment.uv.x - fill;
                    let unselected = (split / split.fwidth() + 0.5).saturate();
                    let star = sd_star(fragment.local, radius * 0.5, radius * 0.32) - radius * 0.1;
                    over_icon(Vec4::ZERO, vec3(1.0, 0.85, 0.2).lerp(Vec3::splat(0.33), unselected), star, primary_alpha)
                });
            }
        }
        let primary_count = primary_count as usize;
        let mut toggled_playlist = None;
        if let Some(track_id) = track.id {
            for (index, playlist) in Self::playlists(track_id, playlists, playlist_expansion > 0.0).enumerate() {
                let primary = index < primary_count;
                let (icon, count, alpha, expansion) = if primary {
                    (index + stars, stars + primary_count, primary_alpha, 1.0)
                } else {
                    (index - primary_count, secondary_count as usize, playlist_expansion, playlist_expansion)
                };
                if alpha > 0.0 {
                    let image = self.image(playlist.image_url.as_deref(), playlist.art.ready());
                    let center = icon_center(pill, height, count, icon, expansion, !primary);
                    frame.input_region(Rect::from_center(center, Vec2::splat(ICON_REACTION_RADIUS)));
                    let rect = Rect::from_center(center, Vec2::splat(ICON_WIDTH * 0.5));
                    let response = frame.interact(rect);
                    hovered |= response.hovered();
                    if response.clicked() {
                        toggled_playlist = Some(playlist.id);
                    }
                    let (quad, mouse_distance) = icon_geometry(center, &frame.globals);
                    // GPU: Track playlist icon.
                    frame.paint_quad(
                        quad,
                        |frame: Globals, fragment: isthmus::Fragment, image: ImageHandle, alpha: f32, primary: bool, mouse_distance: f32| {
                            let radius = (fragment.size.x - 14.0) * 0.5;
                            let desaturation = if primary || (frame.info.mouse_pressure > 0.0 && mouse_distance <= ICON_WIDTH * 0.5) {
                                0.0
                            } else {
                                0.2
                            };
                            let Some(texture) = fragment.sample_image(image, fragment.uv) else {
                                kill();
                            };
                            over_icon(
                                Vec4::ZERO,
                                texture.truncate().lerp(Vec3::splat(0.24), desaturation),
                                sd_capsule_box(fragment.local, 0.0, radius * 0.6),
                                alpha,
                            )
                        },
                    );
                }
            }
            if let Some(playlist_id) = toggled_playlist {
                frame.services.music.toggle_playlist(playlists, track_id, playlist_id);
                particles.burst(frame);
            }
        }
        track.runtime.playlist_expansion.approach(
            f32::from(hovered && show_details && detail_alpha >= 1.0),
            frame.delta_time.min(0.1) / PLAYLIST_EXPANSION_DURATION,
        );
    }

    fn paint_track_text(frame: &mut Frame, line: text::Line, pill: Pill, visibility: f32) {
        if line.is_empty() {
            return;
        }
        let origin = vec2(pill.x, pill.y);
        // GPU: Track label.
        frame.paint_text(
            line.expanded(20.0).translated(origin),
            |frame: Globals, text: isthmus::TextFragment, pill: Pill, visibility: f32| {
                let surface = pill.sample(text.pixel, &frame);
                let image_center = vec2(surface.size.x - surface.size.y, 0.0) + Vec2::splat(surface.size.y * 0.5);
                let alpha = text.alpha_at(vec2(pill.x, pill.y) + surface.refracted)
                    * sd_capsule_box(surface.refracted - image_center, 0.0, surface.size.y * 0.5).smoothstep(2.0, 18.0)
                    * surface.mask
                    * visibility;
                text.color(alpha)
            },
        );
    }

    pub fn show(&mut self, frame: &mut Frame, playback: &mut PlaybackState, particles: &mut Particles) {
        if self.images.is_none() {
            self.images = Some(frame.image_array([IMAGE_SIZE; 2], MAX_TEXTURE_IMAGES));
        }
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
        for layout in layouts {
            let track = &mut playback.queue[layout.queue_index];
            self.show_track(track, layout, &mut playback.playlists, &playback.timeline, particles, frame);
        }
        if frame.released() {
            if frame.dragging()
                && let Some(layout) = current_track
                && playback.queue[layout.queue_index].id.is_some()
            {
                let duration_ms = playback.queue[layout.queue_index].duration_ms;
                let natural_start = frame.globals.playhead_x + layout.start_ms * frame.globals.px_per_ms;
                let fraction = (frame.globals.playhead_x.max(layout.x) - natural_start) / (duration_ms as f32 * frame.globals.px_per_ms);
                frame.services.music.seek(&playback.timeline, layout.queue_index, duration_ms, fraction);
            }
            frame.cancel_drag();
        }
    }
}

fn shade_track(pixel: Vec2, frame: &Globals, colors: [Unorm8x4; PALETTE_COLORS], visibility: f32, seed: f32, effects: AudioFeatures, surface: Surface, album: Vec4) -> Vec4 {
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

    color = color.lerp(album.truncate(), album.w);

    color += color.lerp(Vec3::ONE, 0.32) * pill_sheen(surface.distance);
    color = color.lerp(color * 1.5 + 0.1, surface.ripple_flash);
    let output = surface.color(color) * visibility;
    if output.w <= 0.0 {
        kill();
    }
    output
}
