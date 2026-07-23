use crate::{
    pill_fragment, pill_sheen, pixel_to_ndc, quad_coord, sd_capsule_box, sd_star, smooth_union,
    unpack3x8unorm,
};
use cantus_shared::{
    AudioFeatures, GlobalUniforms, ICON_WIDTH, MAX_PILL_PLAYLIST_ICONS, PillIconRow, TrackPill,
    smoothstep,
};
use core::f32::consts::{FRAC_PI_2, TAU};
use spirv_std::{
    Sampler,
    arch::{Derivative, kill},
    glam::{FloatExt, Vec2, Vec3, Vec4, vec2, vec3},
    image::Image2dArray,
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

fn plasma_field(uv: Vec2, packed: u32, x: f32, y: f32, phase: f32) -> Vec4 {
    let wave = (uv.dot(vec2(x, y)) + phase).sin() * 0.5 + 0.5;
    let weight = (0.12 + wave * wave) * (0.25 + (packed >> 24) as f32 / 255.0 * 3.0);
    (unpack3x8unorm(packed) * weight).extend(weight)
}

fn hash(point: Vec2, seed: f32) -> f32 {
    let value = (point.dot(vec2(127.1, 311.7)) + seed * 74.7).sin() * 43_758.547;
    value - value.floor()
}

fn speckle(pixel: Vec2, time: f32, seed: f32, audio: AudioFeatures) -> f32 {
    let amount = audio.acousticness * 0.7 + audio.instrumentalness * 0.3;
    let drift = vec2(
        0.16 + seed.fract() * 0.08,
        0.055 + (seed * 0.7).sin() * 0.025,
    );
    let uv = pixel / (8.0 - amount) + time * (0.35 + audio.instrumentalness * 0.55) * drift;
    let cell = uv.floor();
    let phase = hash(vec2(cell.y, cell.x), seed + 2.71);
    let center = vec2(phase, (phase * 7.13).fract()) * 0.56 - 0.28;
    let twinkle = time * (0.7 + phase * 0.9 + audio.instrumentalness * 0.8) + phase * TAU;
    smoothstep(0.985 - amount * 0.09, 1.0, hash(cell, seed))
        * (1.0 - smoothstep(0.06, 0.28, (uv - cell - 0.5 - center).length()))
        * (twinkle.sin() * 0.5 + 0.5)
        * (0.12 + amount * 0.48)
}

fn icon_local(pixel: Vec2, center: Vec2, global: &GlobalUniforms) -> (Vec2, Vec2, f32, f32) {
    let mouse_distance = center.distance(global.mouse_pos);
    let proximity = smoothstep(
        30.0,
        8.0,
        mouse_distance / global.mouse_pressure.clamp(0.001, 1.0),
    );
    let pixel_radius = ICON_WIDTH * 0.5 * (1.05 + 0.63 * proximity);
    let x_push = (center.x - global.mouse_pos.x) * proximity * 0.5;
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
    if value > 0.0 { 1.0 } else { 0.0 }
}

fn over_icon(base: Vec4, color: Vec3, shape: f32, alpha: f32) -> Vec4 {
    let mask = (0.5 - shape).saturate();
    let shadow = (-shape.max(0.0) * 0.5).exp();
    let bevel = 1.0 - smoothstep(0.0, -5.0, shape);
    let layer = ((color + bevel * bevel * 0.045) * mask * alpha)
        .extend(mask.max(shadow * shadow * 0.2) * alpha);
    base * (1.0 - layer.w) + layer
}

#[spirv(vertex)]
pub fn vs_track(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(instance_index)] i_idx: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] pills: &[TrackPill],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_pixel_pos: &mut Vec2,
    #[spirv(location = 1, flat)] out_pill_idx: &mut u32,
) {
    let pill = pills[i_idx as usize];
    let margin = 48.0;
    let pill_size = vec2(pill.width, global.bar_height.y);
    let pill_origin = vec2(pill.x, global.bar_height.x);

    let (primary_row, secondary_row) = pill.icon_rows(global.bar_height.x, global.bar_height.y);

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

    let pixel_pos = render_min + quad_coord(v_idx) * (render_max - render_min);
    *out_pos = pixel_to_ndc(pixel_pos, global.screen_size);
    *out_pixel_pos = pixel_pos;
    *out_pill_idx = i_idx;
}

#[spirv(fragment)]
pub fn fs_track(
    #[spirv(location = 0)] pixel_pos: Vec2,
    #[spirv(location = 1, flat)] pill_idx: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] pills: &[TrackPill],
    #[spirv(descriptor_set = 0, binding = 2)] images: &Image2dArray,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = pills[pill_idx as usize];
    let (interaction, local_pixel, pill_size, body_dist) =
        pill_fragment(pixel_pos, global, pill.x, pill.width);
    let local_uv = local_pixel / pill_size;
    let local_centered = local_uv - 0.5;
    let stretched_uv_y =
        local_centered.y * (pill_size.y / (pill_size.y + interaction.bulge())) + 0.5;

    let (primary_row, secondary_row) = pill.icon_rows(global.bar_height.x, global.bar_height.y);

    let mut dist = smooth_union(
        body_dist,
        sd_capsule_box(
            pixel_pos - (primary_row.backplate_center() - vec2(0.0, 2.0)),
            primary_row.half_span(),
            9.0,
        ),
        10.0,
        pill.primary_alpha,
    );
    dist = smooth_union(
        dist,
        sd_capsule_box(
            pixel_pos - secondary_row.backplate_center(),
            secondary_row.half_span(),
            10.5 * pill.secondary_expansion,
        ),
        ICON_WIDTH * 0.5,
        presence(pill.secondary_expansion),
    );
    let (dist, mask, alpha) = interaction.surface(dist);

    // Weight overlapping wave fields by each palette colour's prevalence.
    let seed = (pill.colors[0] % 1000) as f32 * 0.013;
    let audio = pill.audio_features;
    let turbulence =
        audio.energy * 0.55 + audio.danceability * 0.25 + (audio.loudness + 60.0) / 60.0 * 0.2;
    let beat = (global.time * audio.tempo * (TAU / 60.0)).sin() * 0.5 + 0.5;
    let beat = beat * beat * audio.danceability * (0.025 + audio.energy * 0.055);
    let lens_warp = (1.0 + dist.min(0.0) / 120.0).saturate();
    let deformation = local_centered * lens_warp * lens_warp * 0.6
        + interaction.ripple
        + interaction.mouse * 0.03;
    let flow_time = global.time
        * (0.12 + audio.energy * 0.25 + ((audio.tempo - 60.0) / 120.0).saturate() * 0.12)
        + seed;
    let frequency =
        (pill_size.x / pill_size.y * (0.5 + seed.fract() * 0.12 + turbulence * 0.18)).max(1.7);
    let field_uv =
        (local_uv.clamp(Vec2::ZERO, Vec2::ONE) - deformation * 0.08) * vec2(frequency, 1.6);
    let warped_uv = field_uv
        + vec2(
            (field_uv.y * 2.7 + flow_time).sin() + (field_uv.x * 1.3 - flow_time * 0.7).cos(),
            (field_uv.x * 2.3 - flow_time * 0.8).cos() + (field_uv.y * 1.7 + flow_time * 0.6).sin(),
        ) * (0.14 + turbulence * 0.2 + beat);
    let phase = seed + FRAC_PI_2;
    let weighted = plasma_field(warped_uv, pill.colors[0], 2.1, 0.7, flow_time)
        + plasma_field(
            warped_uv,
            pill.colors[1],
            0.6,
            -2.4,
            phase - flow_time * 0.8,
        )
        + plasma_field(warped_uv, pill.colors[2], -1.5, 1.9, flow_time * 0.65 + 2.0)
        + plasma_field(
            warped_uv,
            pill.colors[3],
            2.4,
            1.6,
            phase - flow_time * 0.55,
        );
    let mut color = weighted.truncate() / weighted.w;

    let luma = color.dot(vec3(0.2126, 0.7152, 0.0722));
    let played = smoothstep(
        global.playhead_x + 3.0,
        global.playhead_x - 3.0,
        pixel_pos.x,
    );
    color = Vec3::splat(luma)
        .lerp(color, 1.55 + audio.valence * 0.4)
        .clamp(Vec3::splat(0.035), Vec3::splat(0.92))
        * (0.52 / luma.max(0.001)).min(1.0)
        * (0.96 + audio.valence * 0.06 + beat * 0.5)
        * (0.84 + smoothstep(0.45, 1.0, stretched_uv_y) * 0.1)
        * (1.0 - 0.4 * played);

    color += unpack3x8unorm(pill.colors[3]).lerp(Vec3::ONE, 0.25)
        * speckle(local_pixel, global.time, seed, audio);

    let image_left = pill_size.x - pill_size.y;
    if pill.image_index >= 0 && local_pixel.x >= image_left {
        let uv_img = vec2((local_pixel.x - image_left) / pill_size.y, stretched_uv_y);
        let tex = images.sample(*sampler, uv_img.extend(pill.image_index as f32));
        let image_dist = sd_capsule_box((uv_img - 0.5) * pill_size.y, 0.0, pill_size.y * 0.5);
        let img_mask = (1.0 - smoothstep(-4.0, 0.0, image_dist))
            * (1.0 - smoothstep(-0.5, 0.5, interaction.expand(body_dist)));
        color = color.lerp(tex.truncate(), img_mask * tex.w);
    }

    color += color.lerp(Vec3::ONE, 0.32) * pill_sheen(stretched_uv_y, dist, interaction);
    color = color.lerp(color * 1.5 + 0.1, interaction.ripple_flash);

    let mut output = (color * mask).extend(alpha);

    if pill.rating >= 0 && pill.primary_alpha > 0.0 {
        for star in 0..5 {
            let star = star as f32;
            let center = primary_row.icon_center(star);
            if !near_icon(pixel_pos, center) {
                continue;
            }
            let fill = ((pill.rating as f32 - star * 2.0) * 0.5).saturate();
            let (local_uv, local_pixel, pixel_radius, _) = icon_local(pixel_pos, center, global);
            let dist =
                sd_star(local_pixel, pixel_radius * 0.5, pixel_radius * 0.32) - pixel_radius * 0.1;
            let split_line = local_uv.x - fill;
            let selection_mask = (split_line / split_line.fwidth() + 0.5).saturate();
            let color = vec3(1.0, 0.85, 0.2).lerp(vec3(0.33, 0.33, 0.33), selection_mask);
            output = over_icon(output, color, dist, pill.primary_alpha);
        }
    }

    let primary_playlists = pill.primary_playlist_count as usize;
    let playlist_count =
        (primary_playlists + pill.secondary_playlist_count as usize).min(MAX_PILL_PLAYLIST_ICONS);
    for index in 0..playlist_count {
        let image_index = pill.playlist_images[index];
        if image_index < 0 {
            continue;
        }
        let primary = index < primary_playlists;
        let (row, icon, alpha) = if primary {
            (
                primary_row,
                index as f32 + pill.star_count(),
                pill.primary_alpha,
            )
        } else {
            (
                secondary_row,
                (index - primary_playlists) as f32,
                pill.secondary_expansion,
            )
        };
        let center = row.icon_center(icon);
        if alpha <= 0.0 || !near_icon(pixel_pos, center) {
            continue;
        }
        let (local_uv, local_pixel, pixel_radius, mouse_distance) =
            icon_local(pixel_pos, center, global);
        let desaturation =
            if primary || (global.mouse_pressure > 0.0 && mouse_distance <= ICON_WIDTH * 0.5) {
                0.0
            } else {
                0.2
            };
        let dist = sd_capsule_box(local_pixel, 0.0, pixel_radius * 0.6);
        if dist > 7.0 {
            continue;
        }
        let tex = images.sample(*sampler, local_uv.extend(image_index as f32));
        output = over_icon(
            output,
            tex.truncate().lerp(Vec3::splat(0.24), desaturation),
            dist,
            alpha,
        );
    }
    output *= pill.visibility;
    if output.w <= 0.0 {
        kill();
    }
    *out_color = output;
}
