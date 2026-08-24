use crate::render::{
    Fragment, PANEL_START, TextFragment,
    sdf::{VISIBLE_ALPHA, hash, pill_margin, pill_sheen, ripple_light, sample_pill, sd_capsule_box, sd_rounded_box, sd_rounded_triangle, sd_star, simplex_noise},
};
use core::f32::consts::{FRAC_PI_2, TAU};
use isthmus::{
    FloatExt, Image, Quad, ShaderData, Unorm8x4,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    rgba,
    spirv_std::arch::{Derivative, kill},
    text::TextStyle,
};

#[cfg(not(target_arch = "spirv"))]
use crate::{
    app::music::{LoudnessTimeline, Music, TRACK_SPACING_MS, Track},
    interaction::Rect,
    render::{BarLayout, GAP, RenderContext},
};

/// Number of colors extracted from album artwork.
pub const PALETTE_COLORS: usize = 4;
/// Visual width, in pixels, of rating and playlist icons before hover growth.
const ICON_WIDTH: f32 = 21.6;
/// Center-to-center icon spacing for rating stars and playlist artwork.
const ICON_SPACING: f32 = 18.0;
const ICON_REACTION_RADIUS: f32 = ICON_WIDTH * 2.5;
const PRIMARY_SUPPORT_DEPTH: f32 = 7.0;
const SECONDARY_SUPPORT_DEPTH: f32 = 18.0;

/// Track-level audio character values normalized for the renderer.
#[repr(C)]
#[derive(Clone, Copy, Default, ShaderData)]
#[cfg_attr(not(target_arch = "spirv"), derive(serde::Deserialize))]
pub struct AudioFeatures {
    /// Energy (intensity) controls background flow speed and turbulence.
    pub energy: f32,
    /// Danceability (dance suitability) controls the background pulse and turbulence.
    pub danceability: f32,
    /// Acousticness (acoustic character) controls the density and motion of background speckles.
    pub acousticness: f32,
    /// Tempo (BPM) controls the background animation pace.
    pub tempo: f32,
    /// Valence (positive mood) controls background color vividness and brightness.
    pub valence: f32,
    /// Instrumentalness (lack of vocals) controls the strength of the caustic light pattern.
    pub instrumentalness: f32,
}

#[derive(Default)]
#[cfg(not(target_arch = "spirv"))]
pub struct MusicView {
    particles: [Particle; 32],
    particles_debt: f32,
    playlist_icons: Vec<usize>,
    bar_split: f32,
    icon_presence: f32,
    icon_morph: f32,
}

#[derive(Clone, Copy, Default)]
struct Particle {
    origin: Vec2,
    velocity: Vec2,
    spawned_at: f32,
    expires_at: f32,
    color: Vec3,
    strength: f32,
}

fn particle_color(color: Vec3) -> Vec3 {
    Vec3::splat(color.dot(vec3(0.299, 0.587, 0.114))).lerp(color, 2.0).lerp(Vec3::ONE, 0.2) * 2.0
}

#[cfg(not(target_arch = "spirv"))]
fn loudness_at(timeline: &LoudnessTimeline, position_ms: f32) -> f32 {
    let index = (position_ms.max(0.0) / timeline.period_ms.max(1) as f32) as usize;
    timeline.samples.get(index).copied().unwrap_or_default().clamp(0, 127) as f32 / 127.0
}

fn icon_color(color: Vec3, shape: f32, alpha: f32) -> Vec4 {
    let mask = (0.5 - shape).saturate();
    let shadow = (-shape.max(0.0) * 0.5).exp();
    let bevel = 1.0 - shape.smoothstep(0.0, -5.0);
    ((color + bevel * bevel * 0.045) * mask * alpha).extend(mask.max(shadow * shadow * 0.2) * alpha)
}

/// Twinkling points for acoustic tracks.
fn speckle(pixel: Vec2, time: f32, seed: f32, audio: AudioFeatures) -> f32 {
    let drift = vec2(0.16 + seed.fract() * 0.08, 0.055 + (seed * 0.7).sin() * 0.025);
    let uv = pixel / (8.0 - audio.acousticness) + time * (0.35 + audio.energy * 0.55) * drift;
    let cell = uv.floor();
    let phase = hash(vec2(cell.y, cell.x) + seed * 4096.0 + 2.71).x;
    let center = vec2(phase, (phase * 7.13).fract()) * 0.56 - 0.28;
    let twinkle = time * (0.7 + phase * 0.9 + audio.energy * 0.8) + phase * TAU;
    hash(cell + seed * 4096.0).x.smoothstep(0.985 - audio.acousticness * 0.09, 1.0)
        * (1.0 - (uv - cell - 0.5 - center).length().smoothstep(0.06, 0.28))
        * (twinkle.sin() * 0.5 + 0.5)
        * (0.12 + audio.acousticness * 0.48)
}

/// Broad, domain-warped light variation for instrumental tracks.
fn caustics(p: Vec2, time: f32, seed: f32, audio: AudioFeatures) -> f32 {
    let domain = p * 1.35 + vec2(seed * 17.0, time * (0.35 + audio.energy * 0.2));
    let warp = vec2(simplex_noise(domain), simplex_noise(domain + vec2(19.1, -7.3)));
    let light = simplex_noise(domain - warp * 0.6) * 0.5 + 0.5;
    light * light * audio.instrumentalness * 0.3
}

#[isthmus::paint]
impl MusicView {
    pub fn show(&mut self, frame: &mut RenderContext, music: &mut Music, bar: BarLayout) {
        if music.queue.is_empty() {
            return;
        }
        // Track layout
        let gap = TRACK_SPACING_MS * bar.px_per_ms;
        let width_trim = (GAP - gap).max(0.0);
        let (history_width, panel_height) = (frame.config.history_width, frame.config.height);
        let future_start_ms = (frame.config.timeline_future_minutes - frame.config.timeline_past_minutes) * 60_000.0;
        let future_end = history_width + frame.config.timeline_future_minutes * 60_000.0 * bar.px_per_ms + width_trim;
        // Count compact history from newest to oldest; rendering consumes it in reverse.
        let mut compact_tracks = 0;
        let mut transition = 0.0;
        let mut start_ms = music.timeline.queue_start_ms + music.queue.iter().map(Track::queue_span_ms).sum::<f32>();
        for track in music.queue.iter().rev() {
            start_ms -= track.queue_span_ms();
            let natural_end = bar.playhead_x + (start_ms + track.duration_ms as f32) * bar.px_per_ms;
            if (history_width..history_width + panel_height).contains(&natural_end) {
                transition = (history_width + panel_height - natural_end) / panel_height;
            } else if natural_end < history_width {
                compact_tracks += 1;
            }
        }

        // Track render and interaction
        let mut drag_response = None;
        let backend = music.backend.clone();
        let timeline = &music.timeline;
        let playlists = &mut music.playlists;
        let queue = &mut music.queue;
        let globals = frame.globals();
        let mut paints = frame.paint.group();
        start_ms = timeline.queue_start_ms;
        for (queue_index, track) in queue.iter_mut().enumerate() {
            let track_start_ms = start_ms;
            start_ms += track.queue_span_ms();
            let natural_start = bar.playhead_x + track_start_ms * bar.px_per_ms;
            let natural_end = natural_start + track.duration_ms as f32 * bar.px_per_ms;
            if track_start_ms > future_start_ms {
                break;
            }
            let (mut x, mut width) = if natural_end >= history_width + panel_height {
                let x = natural_start.max(history_width);
                (x, (natural_end.min(future_end) - x - width_trim).max(0.0))
            } else if natural_end >= history_width {
                (natural_end - panel_height, panel_height)
            } else {
                compact_tracks -= 1;
                let right = history_width - gap - (compact_tracks as f32 + transition) * panel_height * 0.55;
                (right - panel_height, panel_height)
            };
            if width <= 0.0 || x + width <= 0.0 {
                continue;
            }
            // Track content
            let playlist_expansion = track.runtime.playlist_expansion.smoothstep(0.0, 1.0);
            let track_text = (width > panel_height + 26.0 || playlist_expansion > 0.0).then(|| {
                let without_suffix = track.name.split_once(" -").map_or(track.name.as_str(), |(name, _)| name);
                let title = without_suffix.split('(').next().filter(|title| !title.trim().is_empty()).unwrap_or(&track.name).trim();
                let seconds = (track_start_ms / 1000.0).abs();
                let details = if seconds >= 60.0 {
                    let whole_seconds = seconds as u32;
                    format!("{}m{}s\u{2004}•\u{2004}{}", whole_seconds / 60, whole_seconds % 60, track.artist)
                } else {
                    format!("{}s\u{2004}•\u{2004}{}", seconds.round(), track.artist)
                };
                let mut text = paints.text();
                (
                    text.line(title, TextStyle::new(16.0, 700.0)).shape(),
                    text.line(&details, TextStyle::new(14.0, 700.0)).shape(),
                )
            });
            if playlist_expansion > 0.0
                && let Some((title, details)) = &track_text
            {
                // Grow toward the width the labels would need at full size.
                let target = title.width.max(details.width) + panel_height + 20.0;
                let extra_width = (target - width).max(0.0) * playlist_expansion;
                x -= extra_width * 0.5;
                width += extra_width;
            }
            let show_details = width > panel_height;

            self.playlist_icons.clear();
            let mut primary_count = 0;
            let mut rating = None;
            if show_details && let Some(track_id) = track.id {
                if frame.config.ratings_enabled {
                    rating = Some(0);
                }
                for (index, playlist) in playlists.iter().enumerate() {
                    let contains_track = playlist.tracks.contains(&track_id);
                    if let Some(value) = playlist.rating_index {
                        if contains_track && rating.is_some() {
                            rating = Some(i32::from(value) + 1);
                        }
                    } else if contains_track {
                        self.playlist_icons.insert(primary_count, index);
                        primary_count += 1;
                    } else if playlist_expansion > 0.0 {
                        self.playlist_icons.push(index);
                    }
                }
            }

            let secondary_count = self.playlist_icons.len() - primary_count;
            let stars = rating.map_or(0, |_| 5);
            let primary_icons = (stars + primary_count) as f32;
            let icon_threshold = ICON_SPACING * 1.05 * primary_icons;
            let icon_alphas = vec2(
                primary_icons.min(1.0) * width.smoothstep(icon_threshold, icon_threshold + ICON_SPACING).max(playlist_expansion),
                playlist_expansion * secondary_count.min(1) as f32,
            );
            let row_width = |icons: f32| ((icons - 1.0).max(0.0) * ICON_SPACING + ICON_WIDTH * 0.7).max(ICON_WIDTH);
            let icon_supports = [
                vec2(row_width(primary_icons) * icon_alphas.x, icon_alphas.x * PRIMARY_SUPPORT_DEPTH),
                vec2(row_width(secondary_count as f32) * icon_alphas.y, icon_alphas.y * SECONDARY_SUPPORT_DEPTH),
            ];
            let pill_rect = Rect::new(x, PANEL_START, x + width.max(panel_height), PANEL_START + panel_height);
            let colors = track.runtime.art.palette();
            let art = track.runtime.art.ready();
            let alpha = width.smoothstep(panel_height, panel_height + 26.0).max(f32::from(track_start_ms <= 0.0));
            let seed = track
                .uri
                .bytes()
                .fold(0xcbf2_9ce4_8422_2325, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)) as u32 as f32
                * 2.328_306_4e-10;
            let audio = track.runtime.audio_features.ready().copied().unwrap_or_default();
            // Pill interaction
            let body = frame.interaction.drag(track.interaction_id, pill_rect);
            let mut hovered = body.hovered();
            let mut track_layer = paints.front(hovered);
            if body.clicked() && track.id.is_some() {
                let pointer_x = frame.interaction.position().x;
                let near_visible_start = pointer_x <= pill_rect.min.x.lerp(pill_rect.max.x, 0.05);
                let fraction = if queue_index == timeline.index && near_visible_start {
                    0.0
                } else {
                    ((pointer_x - natural_start) / (track.duration_ms as f32 * bar.px_per_ms)).clamp(0.0, 1.0)
                };
                backend.seek(timeline, queue_index, track.duration_ms, fraction);
            }
            // Pill render
            let pill: Quad = pill_rect.into();
            let margin = pill_margin(globals, track_layer.time);
            let render_quad = Quad::from_min_max(
                pill.center - pill.size * 0.5 - Vec2::splat(margin),
                pill.center + pill.size * 0.5 + vec2(margin, margin + (icon_supports[1].y + 1.0).max(icon_supports[0].y - 6.0).max(0.0)),
            );
            track_layer.paint_quad(
                render_quad,
                |fragment: Fragment, pill: Quad, alpha: f32, seed: f32, icon_supports: [Vec2; 2], colors: [Unorm8x4; PALETTE_COLORS], audio: AudioFeatures| {
                    let bottom = pill.center.y + pill.size.y * 0.5;
                    let mut surface = sample_pill(pill, fragment.pixel, fragment.globals, fragment.time);
                    let mut index = 0;
                    while index < 2 {
                        let support = icon_supports[index];
                        let half_size = support * 0.5;
                        let center = vec2(pill.center.x, bottom - 6.0 + index as f32 * 7.0 + half_size.y);
                        surface = surface.union(
                            sd_rounded_box(fragment.pixel - center, half_size, half_size.y),
                            sd_rounded_box(fragment.globals.pointer - center + vec2(0.0, 10.0), half_size, half_size.y),
                            9.0,
                            (support.y / (PRIMARY_SUPPORT_DEPTH + index as f32 * (SECONDARY_SUPPORT_DEPTH - PRIMARY_SUPPORT_DEPTH))).saturate(),
                        );
                        index += 1;
                    }

                    if surface.alpha * alpha <= VISIBLE_ALPHA {
                        kill();
                    }

                    let uv = surface.uv();
                    let pace = ((audio.tempo - 0.2) * 2.5).saturate();
                    let flow_time = fragment.time * (0.12 + audio.energy * 0.25 + pace * 0.12) + seed;
                    let pulse = (fragment.time * audio.tempo * 5.0 * TAU).sin() * 0.5 + 0.5;
                    let beat = pulse * pulse * audio.danceability * (0.025 + audio.energy * 0.055);
                    let turbulence = audio.energy * 0.65 + audio.danceability * 0.35;

                    // Distort pill-local coordinates into a slow, overlapping plasma field.
                    let lens = (1.0 + surface.distance.min(0.0) / 120.0).saturate();
                    let deformation = (uv - 0.5) * lens * lens * 0.6 + surface.ripple;
                    let frequency = (surface.size.x / surface.size.y * (0.5 + seed.fract() * 0.12 + turbulence * 0.18)).max(1.7);
                    let field = (uv.clamp(Vec2::ZERO, Vec2::ONE) - deformation * 0.08) * vec2(frequency, 1.6);
                    let warped = field
                        + vec2(
                            (field.y * 2.7 + flow_time).sin() + (field.x * 1.3 - flow_time * 0.7).cos(),
                            (field.x * 2.3 - flow_time * 0.8).cos() + (field.y * 1.7 + flow_time * 0.6).sin(),
                        ) * (0.14 + turbulence * 0.2 + beat);

                    // Weight each wave by its palette colour's prevalence.
                    let directions = [vec2(2.1, 0.7), vec2(0.6, -2.4), vec2(-1.5, 1.9), vec2(2.4, 1.6)];
                    let phases = [flow_time, seed + FRAC_PI_2 - flow_time * 0.8, flow_time * 0.65 + 2.0, seed + FRAC_PI_2 - flow_time * 0.55];
                    let mut plasma = Vec4::ZERO;
                    for index in 0..PALETTE_COLORS {
                        let swatch = colors[index].to_vec4();
                        let wave = (warped.dot(directions[index]) + phases[index]).sin() * 0.5 + 0.5;
                        let weight = (0.12 + wave * wave) * (0.25 + swatch.w * 3.0);
                        plasma += (swatch.truncate() * weight).extend(weight);
                    }
                    let mut color = plasma.truncate() / plasma.w.max(0.001);

                    // Normalize brightness before applying track character and playback state.
                    let luma = color.dot(vec3(0.2126, 0.7152, 0.0722));
                    color = Vec3::splat(luma).lerp(color, 1.55 + audio.valence * 0.4).clamp(Vec3::splat(0.035), Vec3::splat(0.92))
                        * (0.52 / luma.max(0.001)).min(1.0)
                        * (0.96 + audio.valence * 0.06 + beat * 0.5)
                        * (0.84 + surface.refracted_uv().y.smoothstep(0.45, 1.0) * 0.1);

                    // Material details sit above the plasma but below refraction/ripple response.
                    color += colors[3].to_vec3().lerp(Vec3::ONE, 0.25) * speckle(surface.local, fragment.time, seed, audio);
                    color *= 1.0 + caustics(surface.local / surface.size.y, fragment.time, seed, audio);
                    color += color.lerp(Vec3::ONE, 0.32) * pill_sheen(surface.distance);
                    color = ripple_light(color, surface.ripple_flash);

                    surface.color(color) * alpha
                },
            );

            // Artwork render
            if let Some(art) = art {
                let image = &art.image;
                let center = pill.center + vec2((pill.size.x - pill.size.y) * 0.5, 0.0);
                let quad = Quad::new(center, Vec2::splat(pill.size.y), Vec2::X).expanded(pill_margin(globals, track_layer.time));
                track_layer.paint_quad(quad, |fragment: Fragment, pill: Quad, image: Image, alpha: f32| {
                    let surface = sample_pill(pill, fragment.pixel, fragment.globals, fragment.time);
                    let image_center = vec2(surface.size.x - surface.size.y, 0.0) + Vec2::splat(surface.size.y * 0.5);
                    let offset = surface.local - image_center;
                    let radius = surface.size.y * 0.5 + surface.bulge() * 0.5;
                    let texture = image.sample(offset / (radius * 2.0) + 0.5);
                    let alpha = texture.w * (1.0 - (offset.length() - radius).smoothstep(-4.0, 0.0)) * (1.0 - surface.distance.smoothstep(-0.5, 0.5)) * alpha;
                    rgba(texture.truncate(), alpha)
                });
            }
            // Text render
            let (left, right) = (18.0, width - panel_height - 8.0);
            if right > left
                && let Some((title, details)) = track_text
            {
                for (line, y) in [(title, 0.26), (details, 0.57)] {
                    let line = track_layer.text().fit(&line, (panel_height * y).floor(), left..right);
                    if line.is_empty() {
                        continue;
                    }
                    track_layer.paint_text(
                        line.expanded(20.0).translated(pill.center - pill.size * 0.5),
                        |text: TextFragment, pill: Quad, alpha: f32| {
                            let surface = sample_pill(pill, text.pixel, text.globals, text.time);
                            let image_center = vec2(surface.size.x - surface.size.y, 0.0) + Vec2::splat(surface.size.y * 0.5);
                            let alpha = text.alpha_at(pill.center - pill.size * 0.5 + surface.refracted)
                                * sd_capsule_box(surface.refracted - image_center, 0.0, surface.size.y * 0.5).smoothstep(2.0, 18.0)
                                * surface.mask
                                * alpha;
                            text.color(alpha)
                        },
                    );
                }
            }
            // Icon interaction and render
            let mut toggled_playlist = None;
            let mut burst = false;
            if let Some(track_id) = track.id {
                for slot in 0..stars + primary_count + secondary_count {
                    let playlist_slot = slot.saturating_sub(stars);
                    let is_star = slot < stars;
                    let (icon, count, alpha, expansion, secondary) = if is_star || playlist_slot < primary_count {
                        (slot, stars + primary_count, icon_alphas.x, 1.0, false)
                    } else {
                        (playlist_slot - primary_count, secondary_count, playlist_expansion, playlist_expansion, true)
                    };
                    if alpha <= 0.0 {
                        continue;
                    }
                    let center = vec2(
                        pill.center.x + (icon as f32 - (count.saturating_sub(1)) as f32 * 0.5) * ICON_SPACING * expansion,
                        PANEL_START + panel_height * 0.975 - 1.0 + f32::from(secondary) * ICON_SPACING * expansion,
                    );
                    frame.interaction.input_region(Rect::from_center(center, Vec2::splat(ICON_REACTION_RADIUS)));
                    let response = frame.interaction.interact(Rect::from_center(center, Vec2::splat(ICON_WIDTH * 0.5)));
                    hovered |= response.hovered();
                    let mouse_distance = center.distance(globals.pointer);
                    let proximity = mouse_distance.smoothstep(ICON_REACTION_RADIUS, ICON_WIDTH * 0.25) * globals.pressure.clamp(0.0, 1.0);
                    let x_push = (center.x - globals.pointer.x) * proximity * 0.5;
                    let radius = ICON_WIDTH * 0.5 * (1.05 + 0.63 * proximity);
                    let quad = Quad::new(center + vec2(x_push, 0.0), Vec2::splat(radius * 2.0 + 14.0), Vec2::from_angle(x_push * 0.01));

                    if !is_star {
                        let playlist_index = self.playlist_icons[playlist_slot];
                        let playlist = &playlists[playlist_index];
                        if response.clicked() {
                            toggled_playlist = Some(playlist.id);
                        }
                        if let Some(art) = playlist.art.ready() {
                            let image = &art.image;
                            let desaturation = 0.2 * f32::from(secondary && (globals.pressure <= 0.0 || mouse_distance > ICON_WIDTH * 0.5));
                            track_layer.paint_quad(quad, |fragment: Fragment, image: Image, alpha: f32, desaturation: f32| {
                                let texture = image.sample(fragment.uv);
                                icon_color(
                                    texture.truncate().lerp(Vec3::splat(0.24), desaturation),
                                    sd_capsule_box((fragment.uv * 2.0 - 1.0) * 18.0, 0.0, 6.7),
                                    alpha,
                                )
                            });
                        }
                    } else if let Some(rating) = rating.as_mut() {
                        let right_half = frame.interaction.position().x >= center.x;
                        if response.hovered() {
                            *rating = slot as i32 * 2 + 1 + i32::from(right_half);
                        }
                        if response.clicked() {
                            backend.rate_track(playlists, track_id, slot as u8 * 2 + u8::from(right_half));
                            burst = true;
                        }
                        let fill = ((*rating as f32 - slot as f32 * 2.0) * 0.5).saturate();
                        track_layer.paint_quad(quad, |fragment: Fragment, fill: f32, alpha: f32| {
                            let split = fragment.uv.x - fill;
                            let unselected = (split / split.fwidth() + 0.5).saturate();
                            let star = sd_star((fragment.uv * 2.0 - 1.0) * 18.0, 5.6, 3.58) - 1.12;
                            icon_color(vec3(1.0, 0.85, 0.2).lerp(Vec3::splat(0.33), unselected), star, alpha)
                        });
                    }
                }
                if let Some(playlist_id) = toggled_playlist {
                    backend.toggle_playlist(playlists, track_id, playlist_id);
                    burst = true;
                }
            }
            if burst {
                for particle in self.particles.iter_mut().filter(|particle| particle.expires_at <= track_layer.time).take(20) {
                    *particle = Particle {
                        origin: frame.interaction.position(),
                        velocity: Vec2::from_angle(fastrand::f32() * TAU) * (30.0 + fastrand::f32() * 20.0),
                        spawned_at: track_layer.time,
                        expires_at: track_layer.time + 0.5 + fastrand::f32(),
                        color: particle_color(vec3(1.0, 0.843, 0.196)),
                        strength: 1.0,
                    };
                }
            }
            track.runtime.playlist_expansion = track
                .runtime
                .playlist_expansion
                .move_towards(f32::from(hovered && show_details && alpha >= 1.0), track_layer.delta_time.min(0.1) / 0.16);
            if body.dragging() {
                drag_response = Some(body);
            }
        }
        paints.finish();

        // Track dragging
        let drag_offset_ms = drag_response.map_or(0.0, |response| (frame.interaction.position().x - response.drag_origin.x) / bar.px_per_ms);
        music.update_timeline(drag_offset_ms, drag_response.is_some(), frame.paint.delta_time);
        let playhead_track = music.timeline.track_at_playhead(&music.queue);
        if drag_response.is_some_and(|response| response.released())
            && let Some((index, position_ms)) = playhead_track
        {
            let track = &music.queue[index];
            if track.id.is_some() {
                backend.seek(&music.timeline, index, track.duration_ms, position_ms / track.duration_ms as f32);
            }
        }

        // Particle emission
        let time = frame.paint.time;
        if let Some((index, _)) = playhead_track {
            let movement = music.timeline.movement;
            let loudness = music.queue[index]
                .runtime
                .loudness
                .ready()
                .map_or(0.5, |timeline| loudness_at(timeline, music.timeline.position_now()));
            let rate = 24.0.lerp(240.0, loudness.powf(0.7));
            self.particles_debt = (self.particles_debt + frame.paint.delta_time * rate) * f32::from(movement.abs() > 0.00001);
            let emit_count = self.particles_debt.floor() as usize;
            self.particles_debt -= emit_count as f32;
            let horizontal_bias = (movement.abs().powf(0.2) * movement.signum()).clamp(-3.0, 3.0);
            let palette = music.queue[index].runtime.art.palette();
            for particle in self.particles.iter_mut().filter(|particle| particle.expires_at <= time).take(emit_count) {
                let y = fastrand::f32();
                *particle = Particle {
                    origin: vec2(bar.playhead_x, PANEL_START + panel_height * (0.1 + y * 0.85)),
                    velocity: vec2(fastrand::usize(40..60) as f32 * horizontal_bias, (y - 0.5) * 10.0),
                    spawned_at: time,
                    expires_at: time + 1.2 + fastrand::f32() * 0.3,
                    color: particle_color(palette[fastrand::usize(0..palette.len())].to_vec3()),
                    strength: loudness,
                };
            }
        } else {
            self.particles_debt = 0.0;
        }

        // Particle render
        for particle in self.particles.iter().filter(|particle| particle.expires_at > time) {
            let elapsed = time - particle.spawned_at;
            let age = elapsed / (particle.expires_at - particle.spawned_at);
            let opacity = (1.0 - age) * elapsed.smoothstep(0.0, 0.15) * (0.12 + particle.strength * 0.38);
            let color = particle.color;
            frame.paint.paint_quad(
                Quad::oriented(
                    particle.origin + particle.velocity * elapsed,
                    vec2(10.0, 5.0) * (age + 0.5) * (0.7 + particle.strength * 0.6),
                    particle.velocity,
                ),
                |fragment: Fragment, color: Vec3, opacity: f32| {
                    let centered = fragment.uv * 2.0 - 1.0;
                    let shape = (centered * vec2(0.8, 1.0)).length().smoothstep(1.0, 0.2);
                    rgba(color, opacity * shape)
                },
            );
        }
        // Playhead interaction and render
        let half_width = panel_height * 0.4;
        let response = frame
            .interaction
            .interact(Rect::from_center(vec2(bar.playhead_x, PANEL_START + panel_height * 0.5), Vec2::splat(half_width)));
        let speed = frame.paint.delta_time * 5.5;
        let last_toggle = music.last_toggle.elapsed().as_secs_f32() / 0.7;
        if !response.hovered() && music.playing && last_toggle < 1.0 {
            self.bar_split = 1.0 - last_toggle;
            self.icon_presence = 1.0 - last_toggle;
            self.icon_morph = self.icon_morph.move_towards(1.0, speed * 1.5);
        } else {
            let show_icon = f32::from(response.hovered() || !music.playing);
            self.bar_split = self.bar_split.move_towards(show_icon, speed);
            self.icon_presence = self.icon_presence.max(show_icon);
            self.icon_presence = self.icon_presence.move_towards(show_icon, speed);
            self.icon_morph = self.icon_morph.move_towards(f32::from(response.hovered() && !music.playing), speed);
        }
        if response.clicked() {
            music.toggle_playing();
        }
        frame.paint.paint_quad(
            Quad::from_min_max(
                vec2(bar.playhead_x - half_width, PANEL_START - 5.0),
                vec2(bar.playhead_x + half_width, PANEL_START + panel_height + 5.0),
            ),
            |fragment: Fragment, bar_split: f32, icon_presence: f32, icon_morph: f32| {
                let mirrored = fragment.local.abs();
                let bar_len = fragment.globals.bar_height * (0.5 - 0.375 * bar_split);
                let bar_distance = sd_capsule_box(vec2(mirrored.y - (fragment.globals.bar_height - bar_len) * 0.5, mirrored.x), bar_len * 0.5, 4.5);
                let pause_distance = vec2((mirrored.x - 4.0 * bar_split).abs(), (mirrored.y - fragment.globals.bar_height * 0.1).max(0.0)).length() - 3.5;
                let play_scale = fragment.globals.bar_height * 0.18 * (1.0 + icon_morph * (1.0 - icon_presence));
                let play_distance = sd_rounded_triangle(fragment.local.perp(), play_scale, play_scale * 0.5);
                let icon_distance = pause_distance.lerp(play_distance, icon_morph);
                let bar_mask = 1.0 - bar_distance.smoothstep(-0.8, 0.2);
                let icon_mask = (1.0 - icon_distance.smoothstep(-0.8, 0.2)) * icon_presence;
                let alpha = icon_mask.max(bar_mask);
                if alpha <= 0.0 {
                    kill();
                }
                let color = vec3(1.0, 0.878, 0.824).lerp(Vec3::splat(0.15), bar_distance.min(icon_distance).smoothstep(-2.5, -1.0));
                rgba(color, alpha)
            },
        );
    }
}
