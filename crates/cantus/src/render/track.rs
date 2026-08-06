use crate::render::{
    shader::{
        SdfSurface, avalanche, hover_mask, pill_fragment, pill_sheen, pixel_to_ndc, quad_coord,
        sd_capsule_box, sd_star,
    },
    shared::{FrameData, smoothstep},
    text,
};
use core::f32::consts::{FRAC_PI_2, TAU};
use isthmus::glam::{FloatExt, UVec2, Vec2, Vec3, Vec4, uvec2, vec2, vec3};
use spirv_std::{
    Sampler,
    arch::{Derivative, kill},
    image::Image2dArray,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

#[cfg(feature = "cpu")]
use {
    crate::{
        MAX_RENDER_INSTANCES, PANEL_START, TRACK_SPACING_MS,
        config::Config,
        interaction::{InteractionState, Rect},
        render::{
            Passes, approach,
            art::{AlbumArt, ArtState},
            frame::Frame,
            shared::GAP,
            text::TextStyle,
        },
        spotify::{CondensedPlaylist, PlaybackState, Track, playlist_icons},
    },
    isthmus::{
        FilterableFloatFormat, FilteringSampler, Pass, SampledTexture, Texture2DArray, wgpu::Extent3d,
    },
    std::{
        sync::{Arc, Weak},
        time::Instant,
    },
};

/// Maximum number of playlist artwork icons carried by one pill instance.
pub const MAX_PILL_PLAYLIST_ICONS: usize = 8;
/// Number of colors extracted from album artwork.
pub const PALETTE_COLORS: usize = 4;
/// Visual width, in pixels, of rating and playlist icons before hover growth.
const ICON_WIDTH: f32 = 21.6;
/// Center-to-center icon spacing for rating stars and playlist artwork.
const ICON_SPACING: f32 = 18.0;
const TEXT_COLOR: Vec3 = Vec3::splat(0.94);
#[cfg(feature = "cpu")]
const TITLE_STYLE: TextStyle = TextStyle::new(16.0, 700.0);
#[cfg(feature = "cpu")]
const DETAILS_STYLE: TextStyle = TextStyle::new(14.0, 700.0);

#[isthmus::data]
#[derive(Default)]
pub struct PaletteColor {
    pub rgb: isthmus::Unorm8x4,
    pub weight: f32,
}

#[isthmus::data]
#[derive(Default)]
pub struct TrackPill {
    pub x: f32,
    pub width: f32,
    pub colors: [PaletteColor; PALETTE_COLORS],
    pub image_index: i32,
    pub rating: i32,
    pub primary_playlist_count: u32,
    pub secondary_playlist_count: u32,
    pub visibility: f32,
    pub primary_alpha: f32,
    pub secondary_expansion: f32,
    pub effects: TrackEffects,
    pub playlist_images: [i32; MAX_PILL_PLAYLIST_ICONS],
    pub text: text::Text<2, { text::MAX_LINE_GLYPHS * 2 }>,
}

impl TrackPill {
    const fn star_count(&self) -> f32 {
        if self.rating >= 0 { 5.0 } else { 0.0 }
    }

    fn icon_rows(&self, panel_top: f32, panel_height: f32) -> (PillIconRow, PillIconRow) {
        let center = Vec2::new(self.x + self.width * 0.5, panel_top + panel_height * 0.975 - 3.0);
        let secondary_expansion = self.secondary_expansion;
        (
            PillIconRow {
                center,
                count: self.star_count() + self.primary_playlist_count as f32,
                expansion: 1.0,
            },
            PillIconRow {
                center: center + Vec2::new(0.0, ICON_SPACING * secondary_expansion),
                count: self.secondary_playlist_count as f32,
                expansion: secondary_expansion,
            },
        )
    }
}

#[cfg(feature = "cpu")]
#[derive(Clone, Copy, serde::Deserialize)]
pub struct AudioFeatures {
    pub energy: f32,
    pub danceability: f32,
    pub acousticness: f32,
    pub tempo: f32,
    pub valence: f32,
    pub instrumentalness: f32,
    pub loudness: f32,
}

#[isthmus::data]
#[derive(Default)]
pub struct TrackEffects {
    pub acousticness: f32,
    pub valence: f32,
    pub instrumentalness: f32,
    pub turbulence: f32,
    pub seed: f32,
    pub beat: f32,
    pub flow_time: f32,
}

#[cfg(feature = "cpu")]
const MAX_TEMPO_BPM: f32 = 300.0;

#[derive(Copy, Clone)]
struct PillIconRow {
    center: Vec2,
    count: f32,
    expansion: f32,
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
}

#[isthmus::outline]
fn plasma_field(uv: Vec2, color: PaletteColor, x: f32, y: f32, phase: f32) -> Vec4 {
    let wave = (uv.dot(vec2(x, y)) + phase).sin() * 0.5 + 0.5;
    let weight = (0.12 + wave * wave) * (0.25 + color.weight * 3.0);
    (color.rgb.rgb() * weight).extend(weight)
}

fn hash(point: Vec2, seed: f32) -> f32 {
    let cell = uvec2(point.x as i32 as u32, point.y as i32 as u32);
    let value = avalanche(cell ^ UVec2::splat(seed.to_bits() * 2_654_435_761));
    value.x as f32 * 2.328_306_4e-10
}

fn speckle(pixel: Vec2, time: f32, effects: TrackEffects) -> f32 {
    let acousticness = effects.acousticness;
    let instrumentalness = effects.instrumentalness;
    let amount = acousticness * 0.7 + instrumentalness * 0.3;
    let drift = vec2(
        0.16 + effects.seed.fract() * 0.08,
        0.055 + (effects.seed * 0.7).sin() * 0.025,
    );
    let uv = pixel / (8.0 - amount) + time * (0.35 + instrumentalness * 0.55) * drift;
    let cell = uv.floor();
    let phase = hash(vec2(cell.y, cell.x), effects.seed + 2.71);
    let center = vec2(phase, (phase * 7.13).fract()) * 0.56 - 0.28;
    let twinkle = time * (0.7 + phase * 0.9 + instrumentalness * 0.8) + phase * TAU;
    smoothstep(0.985 - amount * 0.09, 1.0, hash(cell, effects.seed))
        * (1.0 - smoothstep(0.06, 0.28, (uv - cell - 0.5 - center).length()))
        * (twinkle.sin() * 0.5 + 0.5)
        * (0.12 + amount * 0.48)
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

fn presence(value: f32) -> f32 {
    // Not `f32::from(bool)`: it lowers through a `u8` cast, which needs `OpCapability Int8`.
    if value > 0.0 { 1.0 } else { 0.0 }
}

fn over_icon(base: Vec4, color: Vec3, shape: f32, alpha: f32) -> Vec4 {
    let mask = (0.5 - shape).saturate();
    let shadow = (-shape.max(0.0) * 0.5).exp();
    let bevel = 1.0 - smoothstep(0.0, -5.0, shape);
    let layer =
        ((color + bevel * bevel * 0.045) * mask * alpha).extend(mask.max(shadow * shadow * 0.2) * alpha);
    base * (1.0 - layer.w) + layer
}

#[derive(isthmus::Varyings)]
pub struct Varyings {
    pub pixel_pos: Vec2,
    #[gpu(flat)]
    pub pill_idx: u32,
}

#[isthmus::pass]
pub struct TrackPass {
    pass: Pass<Self>,
    images: ImageAtlas,
    pub offset: f32,
    pub movement_speed: f32,
    pub current_track_palette: Option<[PaletteColor; PALETTE_COLORS]>,
}

#[isthmus::pass]
impl TrackPass {
    #[gpu]
    pub fn vertex(
        #[gpu(vertex_index)] vertex: u32,
        #[gpu(instance_index)] instance: u32,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] pill: TrackPill,
    ) -> isthmus::Vertex<Varyings> {
        let margin = 48.0;
        let pill_size = vec2(pill.width, frame.panel_height);
        let pill_origin = vec2(pill.x, frame.panel_top);

        let (primary_row, secondary_row) = pill.icon_rows(frame.panel_top, frame.panel_height);
        let primary_alpha = pill.primary_alpha;
        let secondary_expansion = pill.secondary_expansion;

        let icon_row_radius = ICON_WIDTH * 1.5;
        let row_bounds =
            |row: PillIconRow, alpha| row.half_size(icon_row_radius) * presence(row.count * alpha);
        let icon_half_size =
            row_bounds(primary_row, primary_alpha).max(row_bounds(secondary_row, secondary_expansion));
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
        isthmus::Vertex {
            position: pixel_to_ndc(pixel_pos, frame.screen_size),
            varyings: Varyings {
                pixel_pos,
                pill_idx: instance,
            },
        }
    }

    #[gpu]
    pub fn fragment(
        Varyings {
            pixel_pos,
            pill_idx: _,
        }: Varyings,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance = pill_idx as usize)] pill: TrackPill,
        #[gpu(resource)] glyphs: &[text::Glyph],
        #[gpu(resource)] edges: &[text::Edge],
        #[gpu(resource)] images: &Image2dArray,
        #[gpu(resource)] sampler: &Sampler,
    ) -> Vec4 {
        let (interaction, local_pixel, pill_size, body_surface) =
            pill_fragment(pixel_pos, frame, pill.x, pill.width);
        let local_uv = local_pixel / pill_size;
        let local_centered = local_uv - 0.5;

        let (primary_row, secondary_row) = pill.icon_rows(frame.panel_top, frame.panel_height);
        let primary_alpha = pill.primary_alpha;
        let secondary_expansion = pill.secondary_expansion;

        let primary_center = primary_row.backplate_center() - vec2(0.0, 2.0);
        let primary_surface = SdfSurface::new(
            sd_capsule_box(pixel_pos - primary_center, primary_row.half_span(), 9.0),
            sd_capsule_box(frame.mouse_pos - primary_center, primary_row.half_span(), 9.0),
        );
        let secondary_center = secondary_row.backplate_center();
        let secondary_surface = SdfSurface::new(
            sd_capsule_box(
                pixel_pos - secondary_center,
                secondary_row.half_span(),
                10.5 * secondary_expansion,
            ),
            sd_capsule_box(
                frame.mouse_pos - secondary_center,
                secondary_row.half_span(),
                10.5 * secondary_expansion,
            ),
        );
        let surface = body_surface
            .smooth_union(primary_surface, 10.0, primary_alpha)
            .smooth_union(secondary_surface, ICON_WIDTH * 0.5, presence(secondary_expansion));
        let (dist, mask, alpha) = interaction.surface(surface);
        if alpha * pill.visibility <= 1.0 / 1024.0 {
            kill();
        }

        let refracted = interaction.refract(local_pixel, pill_size, dist) * pill_size;

        // Weight overlapping wave fields by each palette colour's prevalence.
        let effects = pill.effects;
        let valence = effects.valence;
        let turbulence = effects.turbulence;
        let beat = effects.beat;
        let lens_warp = (1.0 + dist.min(0.0) / 120.0).saturate();
        let deformation = local_centered * lens_warp * lens_warp * 0.6 + interaction.ripple;
        let flow_time = effects.flow_time;
        let frequency = (pill_size.x / pill_size.y
            * (0.5 + effects.seed.fract() * 0.12 + turbulence * 0.18))
            .max(1.7);
        let field_uv =
            (local_uv.clamp(Vec2::ZERO, Vec2::ONE) - deformation * 0.08) * vec2(frequency, 1.6);
        let warped_uv = field_uv
            + vec2(
                (field_uv.y * 2.7 + flow_time).sin() + (field_uv.x * 1.3 - flow_time * 0.7).cos(),
                (field_uv.x * 2.3 - flow_time * 0.8).cos() + (field_uv.y * 1.7 + flow_time * 0.6).sin(),
            ) * (0.14 + turbulence * 0.2 + beat);
        let phase = effects.seed + FRAC_PI_2;
        let weighted = plasma_field(warped_uv, pill.colors[0], 2.1, 0.7, flow_time)
            + plasma_field(warped_uv, pill.colors[1], 0.6, -2.4, phase - flow_time * 0.8)
            + plasma_field(warped_uv, pill.colors[2], -1.5, 1.9, flow_time * 0.65 + 2.0)
            + plasma_field(warped_uv, pill.colors[3], 2.4, 1.6, phase - flow_time * 0.55);
        let mut color = weighted.truncate() / weighted.w;

        let luma = color.dot(vec3(0.2126, 0.7152, 0.0722));
        let played = smoothstep(frame.playhead_x + 3.0, frame.playhead_x - 3.0, pixel_pos.x);
        color = Vec3::splat(luma)
            .lerp(color, 1.55 + valence * 0.4)
            .clamp(Vec3::splat(0.035), Vec3::splat(0.92))
            * (0.52 / luma.max(0.001)).min(1.0)
            * (0.96 + valence * 0.06 + beat * 0.5)
            * (0.84 + smoothstep(0.45, 1.0, refracted.y / pill_size.y) * 0.1)
            * (1.0 - 0.4 * played);

        color +=
            pill.colors[3].rgb.rgb().lerp(Vec3::ONE, 0.25) * speckle(local_pixel, frame.time, effects);

        let image_left = pill_size.x - pill_size.y;
        if pill.image_index >= 0 && local_pixel.x >= image_left {
            let uv_img = (refracted - vec2(image_left, 0.0)) / pill_size.y;
            let tex = images.sample(*sampler, uv_img.extend(pill.image_index as f32));
            let image_dist = sd_capsule_box((uv_img - 0.5) * pill_size.y, 0.0, pill_size.y * 0.5);
            let img_mask = (1.0 - smoothstep(-4.0, 0.0, image_dist))
                * (1.0 - smoothstep(-0.5, 0.5, interaction.expand(body_surface)));
            color = color.lerp(tex.truncate(), img_mask * tex.w);
        }

        color += color.lerp(Vec3::ONE, 0.32) * pill_sheen(refracted.y / pill_size.y, dist);
        color = color.lerp(color * 1.5 + 0.1, interaction.ripple_flash);
        let mut output = (color * mask).extend(alpha);

        if pill.rating >= 0 && primary_alpha > 0.0 {
            let mut star_index = 0;
            while star_index < 5 {
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
                    output = over_icon(output, color, dist, primary_alpha);
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
                    (primary_row, index as f32 + pill.star_count(), primary_alpha)
                } else {
                    (
                        secondary_row,
                        (index - primary_playlists) as f32,
                        secondary_expansion,
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

        let distance = text::pair_distance(&pill.text, [0, 1], glyphs, edges, refracted);
        let image_center = vec2(image_left + pill_size.y * 0.5, pill_size.y * 0.5);
        let image_fade = smoothstep(
            0.0,
            8.0,
            sd_capsule_box(refracted - image_center, 0.0, pill_size.y * 0.5),
        );
        let text_alpha = text::alpha(distance) * image_fade * mask;
        output = output * (1.0 - text_alpha) + (TEXT_COLOR * text_alpha).extend(text_alpha);

        output *= pill.visibility;
        if output.w <= 0.0 {
            kill();
        }
        output
    }
}

#[cfg(feature = "cpu")]
impl Default for AudioFeatures {
    fn default() -> Self {
        Self {
            energy: 0.5,
            danceability: 0.5,
            acousticness: 0.3,
            tempo: 120.0,
            valence: 0.5,
            instrumentalness: 0.1,
            loudness: -10.0,
        }
    }
}

#[cfg(feature = "cpu")]
impl TrackEffects {
    fn new(mut audio: AudioFeatures, time: f32, seed: f32) -> Self {
        audio.energy = audio.energy.saturate();
        audio.danceability = audio.danceability.saturate();
        audio.acousticness = audio.acousticness.saturate();
        audio.valence = audio.valence.saturate();
        audio.instrumentalness = audio.instrumentalness.saturate();
        audio.loudness = ((audio.loudness + 60.0) / 60.0).saturate();
        let tempo = audio.tempo.clamp(0.0, MAX_TEMPO_BPM);
        let beat = (time * tempo * (TAU / 60.0)).sin() * 0.5 + 0.5;
        Self {
            acousticness: audio.acousticness,
            valence: audio.valence,
            instrumentalness: audio.instrumentalness,
            turbulence: audio.energy * 0.55 + audio.danceability * 0.25 + audio.loudness * 0.2,
            seed,
            beat: beat * beat * audio.danceability * (0.025 + audio.energy * 0.055),
            flow_time: time * (0.12 + audio.energy * 0.25 + ((tempo - 60.0) / 120.0).saturate() * 0.12)
                + seed,
        }
    }
}
#[cfg(feature = "cpu")]
const DETAIL_FADE_DURATION: f32 = 0.2;
#[cfg(feature = "cpu")]
const PLAYLIST_EXPANSION_DURATION: f32 = 1.0 / 6.0;
#[cfg(feature = "cpu")]
const MAX_TEXTURE_IMAGES: u32 = 32;
#[cfg(feature = "cpu")]
pub const IMAGE_SIZE: u32 = 64;

/// The album-art texture atlas this pass's shader samples from.
#[cfg(feature = "cpu")]
struct ImageAtlas {
    texture: SampledTexture<Texture2DArray>,
    slots: [Weak<AlbumArt>; MAX_TEXTURE_IMAGES as usize],
    used: u32,
}

#[cfg(feature = "cpu")]
impl ImageAtlas {
    /// Clears the per-frame usage mask; call once before re-registering this frame's images.
    const fn begin_frame(&mut self) {
        self.used = 0;
    }

    fn slot_for(&mut self, art: &Arc<AlbumArt>) -> i32 {
        if let Some(index) = self
            .slots
            .iter()
            .position(|slot| slot.as_ptr() == Arc::as_ptr(art))
        {
            self.used |= 1 << index;
            return index as i32;
        }

        let index = (!self.used).trailing_zeros();
        if index >= MAX_TEXTURE_IMAGES {
            return -1;
        }
        if self
            .texture
            .write([0, 0, index], [IMAGE_SIZE; 2], &art.pixels)
            .is_err()
        {
            return -1;
        }
        self.used |= 1 << index;
        self.slots[index as usize] = Arc::downgrade(art);
        index as i32
    }
}

/// The atlas slot index `art` is (or becomes) resident in, or -1 if it isn't ready or there's no free slot.
#[cfg(feature = "cpu")]
fn image_index(images: &mut ImageAtlas, art: &ArtState) -> i32 {
    let Some(art) = art.ready() else {
        return -1;
    };
    images.slot_for(art)
}

#[cfg(feature = "cpu")]
#[derive(Clone, Copy)]
struct TrackLayout {
    start_ms: f32,
    x: f32,
    width: f32,
}

#[cfg(feature = "cpu")]
impl TrackLayout {
    fn natural_x_range(self, track: &Track, px_per_ms: f32, playhead_x: f32) -> (f32, f32) {
        let start = playhead_x + self.start_ms * px_per_ms;
        (start, start + track.duration_ms as f32 * px_per_ms)
    }
}

/// Every queued track's on-screen position for the current scroll offset.
#[cfg(feature = "cpu")]
fn layouts<'a>(
    queue: &'a mut [Track],
    config: &Config,
    px_per_ms: f32,
    playhead_x: f32,
    current_ms: f32,
) -> impl Iterator<Item = (usize, &'a mut Track, TrackLayout)> + 'a {
    let end_ms = (config.timeline_future_minutes - config.timeline_past_minutes) * 60_000.0;
    let gap = TRACK_SPACING_MS * px_per_ms;
    let width_trim = (GAP - gap).max(0.0);
    let end_x =
        config.history_width + config.timeline_future_minutes * 60_000.0 * px_per_ms + width_trim;
    let mut compact_slot = 0;
    let mut transition = 0.0;
    let mut queue_end_ms = current_ms + queue.iter().map(Track::queue_span_ms).sum::<f32>();
    let (history_width, panel_height) = (config.history_width, config.height);
    queue.iter_mut().enumerate().rev().map(move |(index, track)| {
        queue_end_ms -= track.queue_span_ms();
        let mut layout = TrackLayout {
            start_ms: queue_end_ms,
            x: 0.0,
            width: panel_height,
        };
        let natural_start = playhead_x + layout.start_ms * px_per_ms;
        let natural_end = natural_start + track.duration_ms as f32 * px_per_ms;
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
        (index, track, layout)
    })
}

#[cfg(feature = "cpu")]
fn song_name(track: &Track) -> &str {
    let name = track
        .name
        .split_once(" -")
        .map_or(track.name.as_str(), |(name, _)| name);
    let name = name.split_once('(').map_or(name, |(name, _)| name).trim();
    if name.is_empty() { track.name.trim() } else { name }
}

#[cfg(feature = "cpu")]
fn track_details(track: &Track, start_ms: f32) -> String {
    let seconds = (start_ms / 1000.0).abs();
    let time = if seconds >= 60.0 {
        let seconds = seconds as u32;
        format!("{}m{}s", seconds / 60, seconds % 60)
    } else {
        format!("{}s", seconds.round())
    };
    let artist = track.artists.first().map_or("", |artist| &artist.name);
    format!("{time}\u{2004}•\u{2004}{artist}")
}

/// The on-screen width `track`'s title/details labels would need at full size.
#[cfg(feature = "cpu")]
fn track_width(font: &text::Font, track: &Track, start_ms: f32, panel_height: f32) -> f32 {
    let text_width = font
        .width(song_name(track), TITLE_STYLE)
        .max(font.width(&track_details(track, start_ms), DETAILS_STYLE));
    text_width + panel_height + 20.0
}

/// Shapes and positions `track`'s title and details lines into `pill`, local to the pill's own origin.
#[cfg(feature = "cpu")]
fn render_labels(
    font: &text::Font,
    pill: &mut TrackPill,
    track: &Track,
    layout: TrackLayout,
    panel_height: f32,
) {
    let left = 12.0;
    let right = layout.width - panel_height - 8.0;
    let top = (panel_height * 0.26).floor();
    let bottom = (panel_height * 0.57).floor();
    let (title, details) = if right > left {
        (song_name(track), track_details(track, layout.start_ms))
    } else {
        ("", String::new())
    };
    pill.text.fit(font, title, TITLE_STYLE, top, left, right);
    pill.text.fit(font, &details, DETAILS_STYLE, bottom, left, right);
}

#[cfg(feature = "cpu")]
struct PlaybackControls<'a> {
    playlists: &'a mut [CondensedPlaylist],
    queue_index: &'a mut usize,
    progress: &'a mut u32,
    last_progress_update: &'a mut Instant,
    last_interaction: &'a mut Instant,
}

#[cfg(feature = "cpu")]
impl PlaybackControls<'_> {
    fn seek(
        &mut self,
        interaction: &InteractionState,
        queue_index: usize,
        duration_ms: u32,
        position: f32,
    ) {
        interaction.seek(
            self.queue_index,
            self.progress,
            self.last_progress_update,
            self.last_interaction,
            queue_index,
            duration_ms,
            position,
        );
    }
}

#[cfg(feature = "cpu")]
impl TrackPass {
    pub fn new(passes: &Passes<'_>, sampler: &FilteringSampler, font: &text::Font) -> Self {
        let texture = passes.sampled_texture::<Texture2DArray>(
            "Images",
            Extent3d {
                width: IMAGE_SIZE,
                height: IMAGE_SIZE,
                depth_or_array_layers: MAX_TEXTURE_IMAGES,
            },
            FilterableFloatFormat::Rgba8Unorm,
        );
        let pass = passes.instances(
            MAX_RENDER_INSTANCES,
            Resources {
                images: texture.view(),
                glyphs: &font.glyphs,
                edges: &font.edges,
                sampler,
            },
        );
        let images = ImageAtlas {
            texture,
            slots: [const { Weak::new() }; MAX_TEXTURE_IMAGES as usize],
            used: 0,
        };
        Self {
            pass,
            images,
            offset: 0.0,
            movement_speed: 0.0,
            current_track_palette: None,
        }
    }

    fn draw_pill(
        &mut self,
        font: &text::Font,
        track: &mut Track,
        layout: &mut TrackLayout,
        playback: &mut PlaybackControls<'_>,
        frame: &mut Frame,
        pill_queue_index: usize,
    ) -> (TrackPill, bool) {
        let playlist_expansion = track.runtime.playlist_expansion_curve();
        if playlist_expansion > 0.0 {
            let target_width = track_width(font, track, layout.start_ms, frame.config.height);
            let extra_width = (target_width - layout.width).max(0.0) * playlist_expansion;
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
        let colors = track.runtime.art.palette();
        let base = colors[0].rgb.rgb();
        let seed = avalanche(uvec2(base.x.to_bits(), base.y.to_bits() ^ base.z.to_bits())).x as f32
            * 2.328_306_4e-10;
        let mut pill = TrackPill {
            x: layout.x,
            width: layout.width.max(frame.config.height),
            colors,
            visibility: detail_alpha.max(f32::from(layout.start_ms <= 0.0)),
            image_index: image_index(&mut self.images, &track.runtime.art),
            rating: -1,
            effects: TrackEffects::new(
                track.audio_features.unwrap_or_default(),
                frame.shared.time,
                seed,
            ),
            playlist_images: [-1; MAX_PILL_PLAYLIST_ICONS],
            ..Default::default()
        };

        render_labels(font, &mut pill, track, *layout, frame.config.height);
        if show_details && let Some(track_id) = track.id {
            let icons = playlist_icons(track_id, playback.playlists, true).chain(
                playlist_icons(track_id, playback.playlists, false)
                    .filter(|_| track.runtime.playlist_expansion > 0.0),
            );
            for (slot, playlist) in pill.playlist_images.iter_mut().zip(icons) {
                *slot = image_index(&mut self.images, &playlist.art);
                let primary = playlist.tracks.contains(&track_id);
                pill.primary_playlist_count += u32::from(primary);
                pill.secondary_playlist_count += u32::from(!primary);
            }
            pill.rating = if frame.config.ratings_enabled {
                playback
                    .playlists
                    .iter()
                    .find_map(|playlist| {
                        playlist
                            .rating_index
                            .filter(|_| playlist.tracks.contains(&track_id))
                    })
                    .map_or(0, |rating| i32::from(rating) + 1)
            } else {
                -1
            };
        }
        let primary_icons = pill.star_count() + pill.primary_playlist_count as f32;
        approach(
            &mut track.runtime.primary_icon_alpha,
            f32::from(primary_icons > 0.0 && layout.width >= ICON_SPACING * 1.05 * primary_icons),
            frame.delta_time / DETAIL_FADE_DURATION,
        );
        pill.primary_alpha = track
            .runtime
            .primary_icon_alpha
            .max(playlist_expansion * f32::from(primary_icons > 0.0));
        pill.secondary_expansion = playlist_expansion;

        let mut hovered = false;
        if let Some(track_id) = track.id {
            let (primary, secondary) = pill.icon_rows(PANEL_START, frame.config.height);
            let stars = pill.star_count() as usize;
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
                    if response.clicked {
                        if primary_row && index < stars {
                            frame.interaction.rate_track(
                                playback.playlists,
                                track_id,
                                index as u8 * 2 + u8::from(right_half),
                            );
                        } else {
                            let playlist_id = playlist_icons(track_id, playback.playlists, primary_row)
                                .nth(index - stars * usize::from(primary_row))
                                .map(|playlist| playlist.id);
                            if let Some(playlist_id) = playlist_id {
                                frame.interaction.toggle_playlist(
                                    playback.playlists,
                                    track_id,
                                    playlist_id,
                                );
                            }
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
            let (start, end) =
                layout.natural_x_range(track, frame.shared.px_per_ms, frame.shared.playhead_x);
            let position = if frame.interaction.pointer.x < frame.config.history_width + 40.0 {
                0.0
            } else {
                (frame.interaction.pointer.x - start) / (end - start)
            };
            playback.seek(frame.interaction, pill_queue_index, track.duration_ms, position);
        }
        approach(
            &mut track.runtime.playlist_expansion,
            f32::from(hovered && show_details && detail_alpha >= 1.0),
            frame.delta_time.min(0.1) / PLAYLIST_EXPANSION_DURATION,
        );
        (pill, hovered)
    }

    pub fn update(&mut self, font: &text::Font, playback: &mut PlaybackState, frame: &mut Frame) {
        self.images.begin_frame();
        if playback.queue.is_empty() {
            self.current_track_palette = None;
            self.pass.instances.clear();
            return;
        }
        let cur_idx = playback.queue_index.min(playback.queue.len() - 1);
        let drag_offset_ms = if frame.interaction.dragging {
            (frame.shared.mouse_pos.x - frame.interaction.press_origin.x) / frame.shared.px_per_ms
        } else {
            0.0
        };
        let current_ms = -playback.estimated_progress()
            - playback.queue[..cur_idx]
                .iter()
                .map(Track::queue_span_ms)
                .sum::<f32>()
            + drag_offset_ms;

        let diff = current_ms - self.offset;
        let current_ms = if !frame.interaction.dragging && diff.abs() > 200.0 {
            self.offset + diff * 3.5 * frame.delta_time
        } else {
            current_ms
        };
        self.movement_speed = self.movement_speed.lerp(
            (current_ms - self.offset) * frame.delta_time,
            (frame.delta_time * 10.0).min(1.0),
        );
        self.offset = current_ms;

        self.pass.instances.clear();
        let mut foreground = None;
        let mut current_track = None;
        if frame.interaction.dragging {
            frame.interaction.claim_hover();
        }
        let mut controls = PlaybackControls {
            playlists: &mut playback.playlists,
            queue_index: &mut playback.queue_index,
            progress: &mut playback.progress,
            last_progress_update: &mut playback.last_progress_update,
            last_interaction: &mut playback.last_interaction,
        };
        for (pill_queue_index, track, mut layout) in layouts(
            &mut playback.queue,
            frame.config,
            frame.shared.px_per_ms,
            frame.shared.playhead_x,
            current_ms,
        ) {
            let is_current = layout.start_ms <= 0.0 && layout.start_ms + track.duration_ms as f32 >= 0.0;
            let can_render =
                self.pass.instances.len() + usize::from(foreground.is_some()) < MAX_RENDER_INSTANCES;
            if can_render && layout.width > 0.0 && layout.x + layout.width > 0.0 {
                let (pill, hovered) =
                    self.draw_pill(font, track, &mut layout, &mut controls, frame, pill_queue_index);
                if hovered {
                    foreground = Some(pill);
                } else {
                    self.pass.instances.push(pill);
                }
            }
            if is_current {
                current_track = Some((pill_queue_index, layout));
            }
        }
        self.pass.instances.reverse();
        if let Some(track) = foreground {
            self.pass.instances.push(track);
        }
        if frame.interaction.released() {
            if frame.interaction.dragging
                && let Some((index, layout)) = current_track
                && playback.queue[index].id.is_some()
            {
                let (start, end) = layout.natural_x_range(
                    &playback.queue[index],
                    frame.shared.px_per_ms,
                    frame.shared.playhead_x,
                );
                let position = (frame.shared.playhead_x.max(layout.x) - start) / (end - start);
                controls.seek(
                    frame.interaction,
                    index,
                    playback.queue[index].duration_ms,
                    position,
                );
            }
            frame.interaction.cancel_drag();
        }
        self.current_track_palette =
            current_track.map(|(index, _)| playback.queue[index].runtime.art.palette());
    }
}
