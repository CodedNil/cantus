use crate::render::{
    FrameData, PANEL_START,
    shader::{
        SdfSurface, avalanche, hover_mask, pill_fragment, pill_margin, pill_sheen, pixel_to_ndc,
        presence, quad_coord, sd_capsule_box, sd_star, simplex_noise,
    },
    smoothstep, text,
};
use core::f32::consts::{FRAC_PI_2, TAU};
use isthmus::{
    Sampler, Texture2DArray, Unorm8x4, Vertex,
    glam::{FloatExt, UVec2, Vec2, Vec3, Vec4, uvec2, vec2, vec3},
    spirv_std::arch::{Derivative, kill},
};

#[cfg(target_arch = "spirv")]
use isthmus::spirv_std::num_traits::Float;

#[cfg(feature = "cpu")]
use crate::{
    app::{
        interaction::Rect,
        music::{AlbumArt, IMAGE_SIZE, TRACK_SPACING_MS},
        music::{CondensedPlaylist, PlaybackState, Timeline, Track, playlist_icons},
    },
    render::{
        GAP,
        cpu::{Frame, Passes, approach},
        text::TextStyle,
    },
};

#[cfg(feature = "cpu")]
use isthmus::{FilterableFloatFormat, SampledTexture, TextureView, wgpu::Extent3d};
/// Maximum number of playlist artwork icons carried by one pill instance.
pub const MAX_PILL_PLAYLIST_ICONS: usize = 8;
/// Number of colors extracted from album artwork.
pub const PALETTE_COLORS: usize = 4;
/// Visual width, in pixels, of rating and playlist icons before hover growth.
const ICON_WIDTH: f32 = 21.6;
/// Center-to-center icon spacing for rating stars and playlist artwork.
const ICON_SPACING: f32 = 18.0;
/// Stars in the rating row; each holds a half-star either side of its centre.
const STAR_RATINGS: usize = 5;
#[cfg(feature = "cpu")]
mod host {
    use super::TextStyle;

    pub const MAX_TEXTURE_IMAGES: u32 = 32;
    pub const MAX_RENDER_INSTANCES: usize = 32;
    pub const TITLE_STYLE: TextStyle = TextStyle::new(16.0, 700.0);
    pub const DETAILS_STYLE: TextStyle = TextStyle::new(14.0, 700.0);
    pub const DETAIL_FADE_DURATION: f32 = 0.2;
    pub const PLAYLIST_EXPANSION_DURATION: f32 = 1.0 / 6.0;
}

#[cfg(feature = "cpu")]
use host::{
    DETAIL_FADE_DURATION, DETAILS_STYLE, MAX_RENDER_INSTANCES, MAX_TEXTURE_IMAGES,
    PLAYLIST_EXPANSION_DURATION, TITLE_STYLE,
};

#[cfg(feature = "cpu")]
struct ImageAtlas {
    texture: SampledTexture<Texture2DArray>,
    slots: Vec<String>,
    used: u32,
}

#[cfg(feature = "cpu")]
impl ImageAtlas {
    fn new(passes: &Passes<'_>) -> Self {
        Self {
            texture: passes.sampled_texture::<Texture2DArray>(
                "Images",
                Extent3d {
                    width: IMAGE_SIZE,
                    height: IMAGE_SIZE,
                    depth_or_array_layers: MAX_TEXTURE_IMAGES,
                },
                FilterableFloatFormat::Rgba8Unorm,
            ),
            slots: vec![String::new(); MAX_TEXTURE_IMAGES as usize],
            used: 0,
        }
    }

    const fn view(&self) -> &TextureView<Texture2DArray> {
        self.texture.view()
    }

    const fn begin_frame(&mut self) {
        self.used = 0;
    }

    fn index_of(&mut self, url: Option<&str>, art: Option<&AlbumArt>) -> i32 {
        let (Some(url), Some(art)) = (url, art) else {
            return -1;
        };
        if let Some(index) = self.slots.iter().position(|slot| slot == url) {
            self.used |= 1 << index;
            return index as i32;
        }
        let index = (!self.used).trailing_zeros();
        if index >= MAX_TEXTURE_IMAGES
            || self
                .texture
                .write([0, 0, index], [IMAGE_SIZE; 2], &art.pixels)
                .is_err()
        {
            return -1;
        }
        self.used |= 1 << index;
        url.clone_into(&mut self.slots[index as usize]);
        index as i32
    }
}

#[isthmus::pass]
pub struct TrackPass {
    pub(crate) instances: isthmus::Instances<Self>,
    images: ImageAtlas,
    pub current_track_palette: Option<[Unorm8x4; PALETTE_COLORS]>,
}

#[derive(isthmus::Varyings)]
pub struct TrackVaryings {
    pub pixel_pos: Vec2,
    #[gpu(flat)]
    pub pill_idx: u32,
}

#[isthmus::data]
#[derive(Default)]
pub struct TrackPill {
    pub x: f32,
    pub width: f32,
    pub colors: [Unorm8x4; PALETTE_COLORS],
    pub image_index: i32,
    pub rating: i32,
    pub primary_playlist_count: u32,
    pub secondary_playlist_count: u32,
    pub visibility: f32,
    pub primary_alpha: f32,
    pub secondary_expansion: f32,
    pub seed: f32,
    pub effects: AudioFeatures,
    pub playlist_images: [i32; MAX_PILL_PLAYLIST_ICONS],
    pub lines: [text::Line; 2],
}

impl TrackPill {
    const fn star_count(&self) -> f32 {
        if self.rating >= 0 {
            STAR_RATINGS as f32
        } else {
            0.0
        }
    }

    fn icon_rows(&self, panel_height: f32) -> (PillIconRow, PillIconRow) {
        let center = Vec2::new(
            self.x + self.width * 0.5,
            PANEL_START + panel_height * 0.975 - 3.0,
        );
        (
            PillIconRow {
                center,
                count: self.star_count() + self.primary_playlist_count as f32,
                expansion: 1.0,
            },
            PillIconRow {
                center: center + Vec2::new(0.0, ICON_SPACING * self.secondary_expansion),
                count: self.secondary_playlist_count as f32,
                expansion: self.secondary_expansion,
            },
        )
    }
}

/// Per-track character, every field normalised to 0..1 on arrival.
#[isthmus::data]
#[derive(Default)]
#[cfg_attr(feature = "cpu", derive(serde::Deserialize))]
pub struct AudioFeatures {
    pub energy: f32,
    pub danceability: f32,
    pub acousticness: f32,
    pub tempo: f32,
    pub valence: f32,
    pub instrumentalness: f32,
    pub loudness: f32,
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
    #[cfg(feature = "cpu")]
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

#[derive(Copy, Clone)]
struct PillIconRow {
    center: Vec2,
    count: f32,
    expansion: f32,
}

/// Where one queued track sits on the timeline this frame.
#[cfg(feature = "cpu")]
#[derive(Clone, Copy)]
struct TrackLayout {
    start_ms: f32,
    x: f32,
    width: f32,
}

impl PillIconRow {
    #[cfg(feature = "cpu")]
    fn hit(self, point: Vec2) -> Option<(usize, bool)> {
        let index = (point.x - self.center.x) / (ICON_SPACING * self.expansion)
            + (self.count - 1.0).max(0.0) * 0.5
            + 0.5;
        let index =
            (self.expansion > 0.0 && (0.0..self.count).contains(&index)).then_some(index as usize)?;
        let center = self.icon_center(index as f32);
        ((point - center).abs().max_element() <= ICON_WIDTH * 0.5)
            .then_some((index, point.x >= center.x))
    }

    fn half_span(self) -> f32 {
        (self.count - 1.0).max(0.0) * ICON_SPACING * self.expansion * 0.5
    }

    fn half_size(self, radius: f32) -> Vec2 {
        Vec2::new(self.half_span() + radius, radius)
    }

    fn backplate_center(self) -> Vec2 {
        self.center + Vec2::new(0.0, -ICON_WIDTH * 0.25)
    }

    fn icon_center(self, index: f32) -> Vec2 {
        let row_center = (self.count - 1.0).max(0.0) * 0.5;
        Vec2::new(
            self.center.x + (index - row_center) * ICON_SPACING * self.expansion,
            self.center.y + 2.0,
        )
    }

    fn surface(self, offset: Vec2, radius: f32, pixel_pos: Vec2, mouse_pos: Vec2) -> SdfSurface {
        let center = self.backplate_center() + offset;
        SdfSurface::sample(pixel_pos, mouse_pos, |point| {
            sd_capsule_box(point - center, self.half_span(), radius)
        })
    }
}

fn icon_local(pixel: Vec2, center: Vec2, frame: &FrameData) -> (Vec2, Vec2, f32, f32) {
    let mouse_distance = center.distance(frame.mouse_pos);
    let proximity = hover_mask(mouse_distance - ICON_WIDTH * 0.5) * frame.mouse_pressure.clamp(0.0, 1.0);
    let pixel_radius = ICON_WIDTH * 0.5 * (1.05 + 0.63 * proximity);
    let x_push = (center.x - frame.mouse_pos.x) * proximity * 0.5;
    let local = pixel - center - vec2(x_push, 0.0);
    let local = Vec2::from_angle(-x_push * 0.01).rotate(local);
    (
        local / (pixel_radius * 2.0) + 0.5,
        local,
        pixel_radius,
        mouse_distance,
    )
}

fn near_icon(pixel: Vec2, center: Vec2) -> bool {
    (pixel - center).abs().max_element() < ICON_WIDTH * 1.8
}

fn over_icon(base: Vec4, color: Vec3, shape: f32, alpha: f32) -> Vec4 {
    let mask = (0.5 - shape).saturate();
    let shadow = (-shape.max(0.0) * 0.5).exp();
    let bevel = 1.0 - smoothstep(0.0, -5.0, shape);
    let layer =
        ((color + bevel * bevel * 0.045) * mask * alpha).extend(mask.max(shadow * shadow * 0.2) * alpha);
    base * (1.0 - layer.w) + layer
}

#[isthmus::outline]
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

/// Twinkling points for acoustic tracks.
fn speckle(pixel: Vec2, time: f32, seed: f32, effects: AudioFeatures) -> f32 {
    let amount = effects.acousticness;
    let drift = vec2(0.16 + seed.fract() * 0.08, 0.055 + (seed * 0.7).sin() * 0.025);
    let uv = pixel / (8.0 - amount) + time * (0.35 + effects.energy * 0.55) * drift;
    let cell = uv.floor();
    let phase = hash(vec2(cell.y, cell.x), seed + 2.71);
    let center = vec2(phase, (phase * 7.13).fract()) * 0.56 - 0.28;
    let twinkle = time * (0.7 + phase * 0.9 + effects.energy * 0.8) + phase * TAU;
    smoothstep(0.985 - amount * 0.09, 1.0, hash(cell, seed))
        * (1.0 - smoothstep(0.06, 0.28, (uv - cell - 0.5 - center).length()))
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

#[isthmus::pass]
impl TrackPass {
    pub fn new(passes: &Passes<'_>, text: &text::Renderer) -> Self {
        let images = ImageAtlas::new(passes);
        let sampler = passes.filtering_sampler("Linear Sampler");
        let (placed_glyphs, glyphs, edges) = text.resources();
        Self {
            instances: passes.instances((images.view(), &sampler, placed_glyphs, glyphs, edges), []),
            images,
            current_track_palette: None,
        }
    }

    fn track_details(track: &Track, start_ms: f32) -> String {
        let seconds = (start_ms / 1000.0).abs();
        let time = if seconds >= 60.0 {
            let seconds = seconds as u32;
            format!("{}m{}s", seconds / 60, seconds % 60)
        } else {
            format!("{}s", seconds.round())
        };
        let artist = &track.artist;
        format!("{time}\u{2004}•\u{2004}{artist}")
    }

    fn prepare_pill(
        &mut self,
        text: &mut text::Renderer,
        track: &mut Track,
        layout: &mut TrackLayout,
        playlists: &mut [CondensedPlaylist],
        timeline: &Timeline,
        frame: &mut Frame,
        pill_queue_index: usize,
    ) -> (TrackPill, bool) {
        let height = frame.config.height;
        // Bare song name, without remix or feature suffixes.
        let title = track
            .name
            .split_once(" -")
            .map_or(track.name.as_str(), |(name, _)| name);
        let title = title.split_once('(').map_or(title, |(name, _)| name).trim();
        let title = if title.is_empty() {
            track.name.trim()
        } else {
            title
        };
        let playlist_expansion = smoothstep(0.0, 1.0, track.runtime.playlist_expansion);
        let labels = (layout.width > height + 26.0 || playlist_expansion > 0.0).then(|| {
            let details = Self::track_details(track, layout.start_ms);
            (
                text.shape(title, TITLE_STYLE),
                text.shape(&details, DETAILS_STYLE),
            )
        });
        if playlist_expansion > 0.0
            && let Some((title, details)) = &labels
        {
            // Grow toward the width the labels would need at full size.
            let target = title.width.max(details.width) + height + 20.0;
            let extra_width = (target - layout.width).max(0.0) * playlist_expansion;
            layout.x -= extra_width * 0.5;
            layout.width += extra_width;
        }
        let show_details = layout.width > frame.config.height;
        approach(
            &mut track.runtime.detail_alpha,
            f32::from(show_details),
            frame.delta_time / DETAIL_FADE_DURATION,
        );
        let detail_alpha = track.runtime.detail_alpha;

        let (left, right) = (18.0, layout.width - height - 8.0);
        let visible = right > left && labels.is_some();
        let lines = match labels {
            Some((title, details)) if visible => [
                text.fit_shaped(&title, (height * 0.26).floor(), left, right),
                text.fit_shaped(&details, (height * 0.57).floor(), left, right),
            ],
            _ => [text::Line::default(); 2],
        };

        // Icon slots hold the primary playlists first, then the secondary ones.
        let mut playlist_images = [-1; MAX_PILL_PLAYLIST_ICONS];
        let mut playlist_ids = [None; MAX_PILL_PLAYLIST_ICONS];
        let (mut primary_count, mut secondary_count) = (0, 0);
        let mut rating = -1;
        if show_details && let Some(track_id) = track.id {
            let icons = playlist_icons(track_id, playlists, true)
                .map(|playlist| (true, playlist))
                .chain(
                    playlist_icons(track_id, playlists, false)
                        .map(|playlist| (false, playlist))
                        .filter(|_| track.runtime.playlist_expansion > 0.0),
                );
            for (slot, (image, (primary, playlist))) in playlist_images.iter_mut().zip(icons).enumerate()
            {
                *image = self
                    .images
                    .index_of(playlist.image_url.as_deref(), playlist.art.ready());
                playlist_ids[slot] = Some(playlist.id);
                primary_count += u32::from(primary);
                secondary_count += u32::from(!primary);
            }
            if frame.config.ratings_enabled {
                rating = playlists
                    .iter()
                    .find_map(|playlist| {
                        playlist
                            .rating_index
                            .filter(|_| playlist.tracks.contains(&track_id))
                    })
                    .map_or(0, |rating| i32::from(rating) + 1);
            }
        }

        let stars = if rating >= 0 { STAR_RATINGS } else { 0 };
        let primary_icons = stars as f32 + primary_count as f32;
        approach(
            &mut track.runtime.primary_icon_alpha,
            f32::from(primary_icons > 0.0 && layout.width >= ICON_SPACING * 1.05 * primary_icons),
            frame.delta_time / DETAIL_FADE_DURATION,
        );
        // FNV-1a of the track id, so each track has its own stable randomness.
        let seed = track
            .id
            .as_deref()
            .unwrap_or(&track.name)
            .bytes()
            .fold(0xcbf2_9ce4u32, |hash, byte| {
                (hash ^ u32::from(byte)).wrapping_mul(0x0100_0193)
            }) as f32
            * 2.328_306_4e-10;
        let mut pill = TrackPill {
            x: layout.x,
            width: layout.width.max(height),
            colors: track.runtime.art.palette(),
            image_index: self
                .images
                .index_of(track.image.as_deref(), track.runtime.art.ready()),
            rating,
            primary_playlist_count: primary_count,
            secondary_playlist_count: secondary_count,
            visibility: detail_alpha.max(f32::from(layout.start_ms <= 0.0)),
            primary_alpha: track
                .runtime
                .primary_icon_alpha
                .max(playlist_expansion * f32::from(primary_icons > 0.0)),
            secondary_expansion: playlist_expansion,
            seed,
            effects: track.runtime.audio_features.ready().copied().unwrap_or_default(),
            playlist_images,
            lines,
        };

        let mut hovered = false;
        if let Some(track_id) = track.id {
            let (primary, secondary) = pill.icon_rows(height);
            for (row, visible, primary_row) in [
                (primary, pill.primary_alpha > 0.0, true),
                (secondary, secondary.count > 0.0, false),
            ] {
                if !visible {
                    continue;
                }
                let response = frame
                    .interaction
                    .surface(Rect::from_center(row.center, row.half_size(ICON_WIDTH * 0.5)));
                hovered |= response.hovered;
                if let Some((index, right_half)) = row.hit(frame.interaction.pointer) {
                    if primary_row && response.hovered && index < stars {
                        pill.rating = index as i32 * 2 + 1 + i32::from(right_half);
                    }
                    let slot = if primary_row {
                        index.wrapping_sub(stars)
                    } else {
                        pill.primary_playlist_count as usize + index
                    };
                    if response.clicked {
                        if primary_row && index < stars {
                            frame.interaction.rate_track(
                                playlists,
                                track_id,
                                index as u8 * 2 + u8::from(right_half),
                            );
                        } else if let Some(id) = playlist_ids.get(slot).copied().flatten() {
                            frame.interaction.toggle_playlist(playlists, track_id, id);
                        }
                    }
                }
            }
        }

        let body = frame
            .interaction
            .surface(Rect::pill(pill.x, pill.width, frame.config.height));
        hovered |= body.hovered;
        if body.pressed {
            frame.interaction.enable_drag();
        }
        if body.clicked && track.id.is_some() {
            let natural_start = frame.shared.playhead_x + layout.start_ms * frame.shared.px_per_ms;
            let fraction = if frame.interaction.pointer.x < frame.config.history_width + 40.0 {
                0.0
            } else {
                (frame.interaction.pointer.x - natural_start)
                    / (track.duration_ms as f32 * frame.shared.px_per_ms)
            };
            frame
                .interaction
                .seek(timeline, pill_queue_index, track.duration_ms, fraction);
        }
        approach(
            &mut track.runtime.playlist_expansion,
            f32::from(hovered && show_details && detail_alpha >= 1.0),
            frame.delta_time.min(0.1) / PLAYLIST_EXPANSION_DURATION,
        );
        (pill, hovered)
    }

    pub fn update(
        &mut self,
        text: &mut text::Renderer,
        playback: &mut PlaybackState,
        frame: &mut Frame,
    ) {
        self.images.begin_frame();
        if playback.queue.is_empty() {
            self.current_track_palette = None;
            self.instances.clear();
            return;
        }
        let current_ms = playback.timeline.queue_start_ms;
        let current_index = playback
            .timeline
            .track_at_playhead(&playback.queue)
            .map(|(index, _)| index);

        self.instances.clear();
        let mut foreground = None;
        let mut current_track = None;
        if frame.interaction.dragging {
            frame.interaction.claim_hover();
        }
        let (px_per_ms, playhead_x) = (frame.shared.px_per_ms, frame.shared.playhead_x);
        let end_ms =
            (frame.config.timeline_future_minutes - frame.config.timeline_past_minutes) * 60_000.0;
        let gap = TRACK_SPACING_MS * px_per_ms;
        let width_trim = (GAP - gap).max(0.0);
        let end_x = frame.config.history_width
            + frame.config.timeline_future_minutes * 60_000.0 * px_per_ms
            + width_trim;
        let (history_width, panel_height) = (frame.config.history_width, frame.config.height);
        // Walked newest first, so compact history pills stack leftward from the playhead.
        let mut compact_slot = 0;
        let mut transition = 0.0;
        let mut queue_end_ms = current_ms + playback.queue.iter().map(Track::queue_span_ms).sum::<f32>();
        for pill_queue_index in (0..playback.queue.len()).rev() {
            let track = &mut playback.queue[pill_queue_index];
            queue_end_ms -= track.queue_span_ms();
            let natural_start = playhead_x + queue_end_ms * px_per_ms;
            let natural_end = natural_start + track.duration_ms as f32 * px_per_ms;
            let mut layout = TrackLayout {
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
                let right =
                    history_width - gap - (compact_slot as f32 + transition) * panel_height * 0.55;
                compact_slot += 1;
                layout.x = right - panel_height;
            }
            let can_render =
                self.instances.len() + usize::from(foreground.is_some()) < MAX_RENDER_INSTANCES;
            if can_render && layout.width > 0.0 && layout.x + layout.width > 0.0 {
                let (pill, hovered) = self.prepare_pill(
                    text,
                    track,
                    &mut layout,
                    &mut playback.playlists,
                    &playback.timeline,
                    frame,
                    pill_queue_index,
                );
                if hovered {
                    foreground = Some(pill);
                } else {
                    self.instances.push(pill);
                }
            }
            if current_index == Some(pill_queue_index) {
                current_track = Some((pill_queue_index, layout));
            }
        }
        self.instances.reverse();
        if let Some(pill) = foreground {
            self.instances.push(pill);
        }
        if frame.interaction.released() {
            if frame.interaction.dragging
                && let Some((index, layout)) = current_track
                && playback.queue[index].id.is_some()
            {
                let duration_ms = playback.queue[index].duration_ms;
                let natural_start = frame.shared.playhead_x + layout.start_ms * frame.shared.px_per_ms;
                let fraction = (frame.shared.playhead_x.max(layout.x) - natural_start)
                    / (duration_ms as f32 * frame.shared.px_per_ms);
                frame
                    .interaction
                    .seek(&playback.timeline, index, duration_ms, fraction);
            }
            frame.interaction.cancel_drag();
        }
        self.current_track_palette =
            current_track.map(|(index, _)| playback.queue[index].runtime.art.palette());
    }

    #[gpu]
    pub fn vertex(
        #[gpu(vertex_index)] vertex: u32,
        #[gpu(instance_index)] instance: u32,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] pill: TrackPill,
    ) -> Vertex<TrackVaryings> {
        let margin = pill_margin(frame);
        let pill_size = vec2(pill.width, frame.panel_height);
        let pill_origin = vec2(pill.x, PANEL_START);

        let (primary_row, secondary_row) = pill.icon_rows(frame.panel_height);

        let icon_row_radius = ICON_WIDTH * 1.5;
        let row_bounds =
            |row: PillIconRow, alpha| row.half_size(icon_row_radius) * presence(row.count * alpha);
        let icon_half_size = row_bounds(primary_row, pill.primary_alpha)
            .max(row_bounds(secondary_row, pill.secondary_expansion));
        let render_min = vec2(
            (pill_origin.x - margin).min(primary_row.center.x - icon_half_size.x),
            pill_origin.y - margin,
        );
        let render_max = vec2(
            (pill_origin.x + pill_size.x + margin).max(primary_row.center.x + icon_half_size.x),
            (pill_origin.y + pill_size.y + margin)
                .max(secondary_row.backplate_center().y + icon_half_size.y),
        );

        let pixel_pos = render_min + quad_coord(vertex) * (render_max - render_min);
        Vertex {
            position: pixel_to_ndc(pixel_pos, frame.screen_size),
            varyings: TrackVaryings {
                pixel_pos,
                pill_idx: instance,
            },
        }
    }

    #[gpu]
    pub fn fragment(
        TrackVaryings {
            pixel_pos,
            pill_idx: _,
        }: TrackVaryings,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance = pill_idx as usize)] pill: TrackPill,
        #[gpu(resource)] images: &Texture2DArray,
        #[gpu(resource)] sampler: &Sampler,
        #[gpu(resource)] placed_glyphs: &[text::PlacedGlyph],
        #[gpu(resource)] glyphs: &[text::Glyph],
        #[gpu(resource)] edges: &[text::Edge],
    ) -> Vec4 {
        let (interaction, local_pixel, pill_size, body_surface) =
            pill_fragment(pixel_pos, frame, pill.x, PANEL_START, pill.width);
        let local_uv = local_pixel / pill_size;
        let local_centered = local_uv - 0.5;

        let (primary_row, secondary_row) = pill.icon_rows(frame.panel_height);

        let primary_surface = primary_row.surface(vec2(0.0, -2.0), 9.0, pixel_pos, frame.mouse_pos);
        let secondary_surface = secondary_row.surface(
            Vec2::ZERO,
            10.5 * pill.secondary_expansion,
            pixel_pos,
            frame.mouse_pos,
        );
        let surface = body_surface
            .smooth_union(primary_surface, 10.0, pill.primary_alpha)
            .smooth_union(
                secondary_surface,
                ICON_WIDTH * 0.5,
                presence(pill.secondary_expansion),
            );
        let (dist, mask, alpha) = interaction.surface(surface);
        if alpha * pill.visibility <= 1.0 / 1024.0 {
            kill();
        }

        let refracted = interaction.refract(local_pixel, pill_size, dist) * pill_size;

        // Weight overlapping wave fields by each palette colour's prevalence.
        let flow_time = pill.effects.flow_time(frame.time, pill.seed);
        let beat = pill.effects.beat(frame.time);
        let turbulence = pill.effects.turbulence();
        let lens_warp = (1.0 + dist.min(0.0) / 120.0).saturate();
        let deformation = local_centered * lens_warp * lens_warp * 0.6 + interaction.ripple;
        let frequency =
            (pill_size.x / pill_size.y * (0.5 + pill.seed.fract() * 0.12 + turbulence * 0.18)).max(1.7);
        let field_uv =
            (local_uv.clamp(Vec2::ZERO, Vec2::ONE) - deformation * 0.08) * vec2(frequency, 1.6);
        let warped_uv = field_uv
            + vec2(
                (field_uv.y * 2.7 + flow_time).sin() + (field_uv.x * 1.3 - flow_time * 0.7).cos(),
                (field_uv.x * 2.3 - flow_time * 0.8).cos() + (field_uv.y * 1.7 + flow_time * 0.6).sin(),
            ) * (0.14 + turbulence * 0.2 + beat);
        let phase = pill.seed + FRAC_PI_2;
        let swatch = |index: usize| pill.colors[index].to_vec4();
        let weighted = plasma_field(warped_uv, swatch(0), 2.1, 0.7, flow_time)
            + plasma_field(warped_uv, swatch(1), 0.6, -2.4, phase - flow_time * 0.8)
            + plasma_field(warped_uv, swatch(2), -1.5, 1.9, flow_time * 0.65 + 2.0)
            + plasma_field(warped_uv, swatch(3), 2.4, 1.6, phase - flow_time * 0.55);
        let mut color = weighted.truncate() / weighted.w;

        let luma = color.dot(vec3(0.2126, 0.7152, 0.0722));
        let played = smoothstep(frame.playhead_x + 3.0, frame.playhead_x - 3.0, pixel_pos.x);
        color = Vec3::splat(luma)
            .lerp(color, 1.55 + pill.effects.valence * 0.4)
            .clamp(Vec3::splat(0.035), Vec3::splat(0.92))
            * (0.52 / luma.max(0.001)).min(1.0)
            * (0.96 + pill.effects.valence * 0.06 + beat * 0.5)
            * (0.84 + smoothstep(0.45, 1.0, refracted.y / pill_size.y) * 0.1)
            * (1.0 - 0.4 * played);

        color += pill.colors[3].to_vec3().lerp(Vec3::ONE, 0.25)
            * speckle(local_pixel, frame.time, pill.seed, pill.effects);
        color += color.lerp(Vec3::ONE, 0.75)
            * caustics(local_pixel / pill_size.y, frame.time, pill.seed, pill.effects);

        let image_left = pill_size.x - pill_size.y;
        let image_center = vec2(image_left, 0.0) + Vec2::splat(pill_size.y * 0.5);
        if pill.image_index >= 0 && (local_pixel - image_center).abs().max_element() < pill_size.y {
            let offset = local_pixel - image_center;
            let radius = pill_size.y * 0.5 + interaction.bulge(surface) * 0.5;
            let image_dist = offset.length() - radius;
            let uv_img = offset / (radius * 2.0) + 0.5;
            let tex = images.sample(*sampler, uv_img.extend(pill.image_index as f32));
            let img_mask = (1.0 - smoothstep(-4.0, 0.0, image_dist))
                * (1.0 - smoothstep(-0.5, 0.5, interaction.expand(body_surface)));
            color = color.lerp(tex.truncate(), img_mask * tex.w);
        }

        color += color.lerp(Vec3::ONE, 0.32) * pill_sheen(dist);
        color = color.lerp(color * 1.5 + 0.1, interaction.ripple_flash);
        let mut output = (color * mask).extend(alpha);

        if pill.rating >= 0 && pill.primary_alpha > 0.0 {
            let mut star_index = 0;
            while star_index < STAR_RATINGS {
                let star = star_index as f32;
                let center = primary_row.icon_center(star);
                if near_icon(pixel_pos, center) {
                    let fill = ((pill.rating as f32 - star * 2.0) * 0.5).saturate();
                    let (local_uv, local_pixel, pixel_radius, _) = icon_local(pixel_pos, center, frame);
                    let dist = sd_star(local_pixel, pixel_radius * 0.5, pixel_radius * 0.32)
                        - pixel_radius * 0.1;
                    let split_line = local_uv.x - fill;
                    let selection_mask = (split_line / split_line.fwidth() + 0.5).saturate();
                    let color = vec3(1.0, 0.85, 0.2).lerp(vec3(0.33, 0.33, 0.33), selection_mask);
                    output = over_icon(output, color, dist, pill.primary_alpha);
                }
                star_index += 1;
            }
        }

        let primary_playlists = pill.primary_playlist_count as usize;
        let playlist_count =
            (primary_playlists + pill.secondary_playlist_count as usize).min(MAX_PILL_PLAYLIST_ICONS);
        let mut index = 0;
        while index < playlist_count {
            let image_index = pill.playlist_images[index];
            if image_index >= 0 {
                let primary = index < primary_playlists;
                let (row, icon, alpha) = if primary {
                    (primary_row, index as f32 + pill.star_count(), pill.primary_alpha)
                } else {
                    (
                        secondary_row,
                        (index - primary_playlists) as f32,
                        pill.secondary_expansion,
                    )
                };
                let center = row.icon_center(icon);
                if alpha > 0.0 && near_icon(pixel_pos, center) {
                    let (local_uv, local_pixel, pixel_radius, mouse_distance) =
                        icon_local(pixel_pos, center, frame);
                    let desaturation = if primary
                        || (frame.mouse_pressure > 0.0 && mouse_distance <= ICON_WIDTH * 0.5)
                    {
                        0.0
                    } else {
                        0.2
                    };
                    let dist = sd_capsule_box(local_pixel, 0.0, pixel_radius * 0.6);
                    if dist <= 7.0 {
                        let tex = images.sample(*sampler, local_uv.extend(image_index as f32));
                        output = over_icon(
                            output,
                            tex.truncate().lerp(Vec3::splat(0.24), desaturation),
                            dist,
                            alpha,
                        );
                    }
                }
            }
            index += 1;
        }

        let text_alpha = text::line_alpha(pill.lines[0], placed_glyphs, glyphs, edges, refracted).max(
            text::line_alpha(pill.lines[1], placed_glyphs, glyphs, edges, refracted),
        ) * smoothstep(
            2.0,
            18.0,
            sd_capsule_box(refracted - image_center, 0.0, pill_size.y * 0.5),
        ) * mask;
        output = output * (1.0 - text_alpha) + (text::COLOR * text_alpha).extend(text_alpha);

        output *= pill.visibility;
        if output.w <= 0.0 {
            kill();
        }
        output
    }
}
