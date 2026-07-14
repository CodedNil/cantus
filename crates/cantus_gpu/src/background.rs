use crate::{
    direction_and_length, pixel_to_ndc, quad_coord, sd_capsule_box, sd_squircle, sd_star,
    smooth_union, unpack3x8unorm,
};
use cantus_shared::{
    BACKPLATE_RADIUS, BackgroundPill, GlobalUniforms, ICON_WIDTH, MAX_PILL_PLAYLIST_ICONS,
    PillIconRow, smoothstep,
};
use core::f32::consts::{FRAC_PI_2, TAU};
use spirv_std::{
    Sampler,
    arch::{Derivative, kill},
    glam::{UVec3, Vec2, Vec3, Vec4, vec2, vec3},
    image::Image2dArray,
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

fn hash(point: Vec2, seed: f32) -> f32 {
    let bits = UVec3::new(point.x.to_bits(), point.y.to_bits(), seed.to_bits());
    let mut value = bits.dot(UVec3::new(0x9e37_79b9, 0x85eb_ca6b, 0xc2b2_ae35));
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^= value >> 16;
    (value >> 8) as f32 * (1.0 / 16_777_216.0)
}

fn icon_local(pixel_pos: Vec2, center: Vec2, global: &GlobalUniforms) -> (Vec2, Vec2, f32, f32) {
    let pressure = global.mouse_pressure.clamp(0.001, 1.0);
    let mouse_distance = center.distance(global.mouse_pos);
    let proximity = smoothstep(30.0, 8.0, mouse_distance / pressure);
    let pixel_radius = ICON_WIDTH * 0.5 * (1.05 + 0.63 * proximity);
    let x_push = (center.x - global.mouse_pos.x) * proximity * 0.5;
    let local = pixel_pos - (center + vec2(x_push, 0.0));
    let angle = -x_push * 0.01;
    let sin = angle.sin();
    let cos = angle.cos();
    let local = vec2(local.x * cos - local.y * sin, local.x * sin + local.y * cos);
    (
        local / (pixel_radius * 2.0) + 0.5,
        local,
        pixel_radius,
        mouse_distance,
    )
}

fn near(pixel_pos: Vec2, center: Vec2, half_size: Vec2) -> bool {
    (pixel_pos - center).abs().cmplt(half_size).all()
}

fn near_icon(pixel_pos: Vec2, center: Vec2) -> bool {
    near(pixel_pos, center, Vec2::splat(ICON_WIDTH * 1.8))
}

fn near_row(pixel_pos: Vec2, row: PillIconRow) -> bool {
    near(
        pixel_pos,
        row.center,
        vec2(row.padded_half_span() + ICON_WIDTH * 1.8, ICON_WIDTH * 1.8),
    )
}

fn icon_overlay(color: Vec3, dist: f32, alpha: f32) -> Vec4 {
    let mask = (0.5 - dist).clamp(0.0, 1.0);
    let shadow = 1.0 - smoothstep(0.0, 6.0, dist);
    let shadow = shadow * shadow * 0.2;
    let bevel = 1.0 - smoothstep(0.0, -5.0, dist);
    let bevel = bevel * bevel * 0.045;
    ((color + bevel) * mask * alpha).extend(mask.max(shadow) * alpha)
}

fn over_premul(base: Vec4, overlay: Vec4, alpha: f32) -> Vec4 {
    let overlay = (overlay.truncate() * alpha).extend(overlay.w * alpha);
    base * (1.0 - overlay.w) + overlay
}

#[spirv(vertex)]
pub fn vs_background(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(instance_index)] i_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] pills: &[BackgroundPill],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_pixel_pos: &mut Vec2,
    #[spirv(location = 1, flat)] out_pill_idx: &mut u32,
) {
    let pill = pills[i_idx as usize];
    let margin = 16.0;
    let unit_coord = quad_coord(v_idx);
    let pill_size = vec2(pill.width, global.bar_height.y);
    let pill_origin = vec2(pill.x, global.bar_height.x);

    let (primary_row, secondary_row) = pill.icon_rows(global.bar_height.x, global.bar_height.y);
    let icon_row_radius = BACKPLATE_RADIUS + ICON_WIDTH / 3.0 + ICON_WIDTH * 0.125;
    let icon_half_size = (primary_row.half_size(icon_row_radius) * pill.primary_alpha)
        .max(secondary_row.half_size(icon_row_radius) * pill.secondary_expansion);
    let render_min = vec2(
        (pill_origin.x - margin).min(primary_row.center.x - icon_half_size.x),
        pill_origin.y - margin,
    );
    let render_max = vec2(
        (pill_origin.x + pill_size.x + margin).max(primary_row.center.x + icon_half_size.x),
        (pill_origin.y + pill_size.y + margin)
            .max(secondary_row.backplate_center().y + icon_half_size.y),
    );

    let pixel_pos = render_min + unit_coord * (render_max - render_min);
    *out_pos = pixel_to_ndc(pixel_pos, global.screen_size);
    *out_pixel_pos = pixel_pos;
    *out_pill_idx = i_idx;
}

#[spirv(fragment)]
pub fn fs_background(
    #[spirv(location = 0)] pixel_pos: Vec2,
    #[spirv(location = 1, flat)] pill_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] pills: &[BackgroundPill],
    #[spirv(descriptor_set = 0, binding = 2)] images: &Image2dArray,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = pills[pill_idx as usize];
    let pill_size = vec2(pill.width, global.bar_height.y);
    let local_pixel = pixel_pos - vec2(pill.x, global.bar_height.x);
    let local_uv = local_pixel / pill_size;
    let local_centered = local_uv - 0.5;

    let anim_t = (global.time - global.expansion_time) * 1.2;
    let (ripple_dir, ripple_strength, ripple_flash) = if (-0.02..1.02).contains(&anim_t) {
        let ripple_t = anim_t.clamp(0.0, 1.0);
        let (ripple_dir, ripple_distance) = direction_and_length(pixel_pos - global.expansion_xy);
        let ripple_active = smoothstep(-0.02, 0.0, anim_t) * (1.0 - smoothstep(1.0, 1.02, anim_t));
        let ripple_decay = 1.0 - ripple_t;
        let ripple_wave =
            smoothstep(80.0, 0.0, (ripple_distance - ripple_t * 600.0).abs()) * ripple_active;
        (
            ripple_dir,
            ripple_decay * ripple_decay * ripple_wave * 0.5,
            ripple_decay * ripple_wave * 0.5,
        )
    } else {
        (Vec2::ZERO, 0.0, 0.0)
    };

    let (mouse_dir, mouse_d, mouse_inf) = if global.mouse_pressure > 0.0 {
        let (mouse_dir, mouse_distance) = direction_and_length(pixel_pos - global.mouse_pos);
        let influence = smoothstep(120.0, 0.0, mouse_distance);
        (
            mouse_dir,
            mouse_distance,
            influence * influence * global.mouse_pressure,
        )
    } else {
        (Vec2::ZERO, 0.0, 0.0)
    };
    let bulge = ripple_strength * 22.0 + mouse_inf * 8.0;
    let stretched_uv_y = local_centered.y * (pill_size.y / (pill_size.y + bulge)) + 0.5;

    let pill_corner_radius = BACKPLATE_RADIUS + ICON_WIDTH * 0.5;
    let (primary_row, secondary_row) = pill.icon_rows(global.bar_height.x, global.bar_height.y);

    // Overall frame SDF: pill body plus icon-row backplates.
    let backplate_radius = BACKPLATE_RADIUS + mouse_inf * ICON_WIDTH / 3.0;
    let mut dist = sd_squircle(
        local_centered * pill_size,
        (pill_size + vec2(0.0, bulge)) * 0.5,
        pill_corner_radius,
    );
    let primary_dist = sd_capsule_box(
        pixel_pos - primary_row.backplate_center(),
        primary_row.padded_half_span(),
        backplate_radius,
    );
    dist = smooth_union(dist, primary_dist, 30.0, pill.primary_alpha);
    let secondary_dist = sd_capsule_box(
        pixel_pos - secondary_row.backplate_center(),
        secondary_row.padded_half_span(),
        backplate_radius,
    );
    dist = smooth_union(
        dist,
        secondary_dist,
        ICON_WIDTH * 0.5,
        pill.secondary_expansion,
    );
    let mask = (0.5 - dist).clamp(0.0, 1.0);
    let shadow = (1.0 - smoothstep(0.0, 14.0, dist)) * 0.16;

    // Domain-warped plasma distributes all four colours without fixed regions.
    // ReccoBeats features shape its pace, rhythmic pulse, and turbulence per track.
    let seed = (pill.colors[0] % 1000) as f32 * 0.013;
    let audio = pill.audio_features.decode();
    let turbulence = audio.turbulence();
    let beat_wave = (global.time * audio.tempo_hz() * TAU).sin() * 0.5 + 0.5;
    let beat = beat_wave * beat_wave * audio.danceability * (0.025 + audio.energy * 0.055);
    let lens_warp = (1.0 + dist.min(0.0) / 120.0).clamp(0.0, 1.0);
    let lens_warp = lens_warp * lens_warp * 0.6;
    let deformation =
        local_centered * lens_warp + ripple_dir * ripple_strength + mouse_dir * mouse_inf * 0.03;
    let flow_speed = 0.12 + audio.energy * 0.25 + audio.tempo_normalized() * 0.12
        - audio.instrumentalness * 0.035
        - audio.acousticness * 0.025;
    let flow_time = global.time * flow_speed + seed;
    let body_uv = local_uv.clamp(Vec2::ZERO, Vec2::ONE);
    let frequency =
        (pill_size.x / pill_size.y * (0.5 + seed.fract() * 0.12 + turbulence * 0.18)).max(1.7);
    let field_uv = (body_uv - deformation * 0.08) * vec2(frequency, 1.6);
    let warp_amount = 0.14 + turbulence * 0.2 + beat;
    let warped_uv = field_uv
        + vec2(
            (field_uv.y * 2.7 + flow_time).sin() + (field_uv.x * 1.3 - flow_time * 0.7).cos(),
            (field_uv.x * 2.3 - flow_time * 0.8).cos() + (field_uv.y * 1.7 + flow_time * 0.6).sin(),
        ) * warp_amount;
    let directions = [
        vec2(2.1, 0.7),
        vec2(0.6, -2.4),
        vec2(-1.5, 1.9),
        vec2(2.4, 1.6),
    ];
    let speeds = [1.0, -0.8, 0.65, -0.55];
    let offsets = [0.0, seed + FRAC_PI_2, 2.0, seed + FRAC_PI_2];
    let mut color = Vec3::ZERO;
    let mut weight_sum = 0.0;
    for index in 0..4 {
        let packed = pill.colors[index];
        let wave = (warped_uv.dot(directions[index]) + flow_time * speeds[index] + offsets[index])
            .sin()
            * 0.5
            + 0.5;
        let weight = (0.12 + wave * wave) * (0.25 + (packed >> 24) as f32 / 255.0 * 3.0);
        color += unpack3x8unorm(packed) * weight;
        weight_sum += weight;
    }
    color /= weight_sum;

    // Preserve rich colour while keeping white text legible.
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

    // Independently pulsing motes drift through the palette field.
    let speckle_uv = local_pixel / (8.0 - audio.acousticness * 0.8)
        + global.time
            * (0.35 + audio.acousticness * 0.55)
            * vec2(
                0.16 + seed.fract() * 0.08,
                0.055 + (seed * 0.7).sin() * 0.025,
            );
    let cell = speckle_uv.floor();
    let random = hash(cell, seed);
    let phase = hash(vec2(cell.y, cell.x), seed + 2.71);
    let offset = vec2(phase, (phase * 7.13).fract()) * 0.56 - 0.28;
    let twinkle =
        (global.time * (0.7 + phase * 0.9 + audio.acousticness * 0.8) + phase * TAU).sin() * 0.5
            + 0.5;
    let speckle_threshold = 0.985 - audio.acousticness * 0.09;
    let speck = smoothstep(speckle_threshold, 1.0, random)
        * (1.0 - smoothstep(0.06, 0.28, (speckle_uv.fract() - 0.5 - offset).length()))
        * twinkle;
    color = color.lerp(
        unpack3x8unorm(pill.colors[3]),
        speck * (0.12 + audio.acousticness * 0.48),
    );

    // Sharp cover art at the trailing edge.
    let image_left = pill_size.x - pill_size.y;
    if pill.image_index >= 0 && local_pixel.x >= image_left {
        let uv_img = vec2((local_pixel.x - image_left) / pill_size.y, stretched_uv_y);
        let tex = images.sample(*sampler, uv_img.extend(pill.image_index as f32));
        let img_mask = 1.0
            - smoothstep(
                -0.5,
                0.5,
                sd_squircle(
                    (uv_img - 0.5) * pill_size.y,
                    Vec2::splat(pill_size.y * 0.5),
                    pill_corner_radius,
                ),
            );
        color = color.lerp(tex.truncate(), img_mask * tex.w);
    }

    color = {
        let top_sheen = smoothstep(0.12, 0.0, stretched_uv_y) * mask * 0.13;
        let rim = smoothstep(5.0, -3.0, dist) * 0.08;
        let mouse_glint = smoothstep(30.0, 0.0, (mouse_d - 15.0).abs()) * mouse_inf * 0.18;
        color + color.lerp(Vec3::ONE, 0.32) * (top_sheen + rim + mouse_glint)
    };
    color = color.lerp(color * 1.5 + 0.1, ripple_flash);

    let alpha = mask.max(shadow) * pill.alpha;
    // Keep shadows black while premultiplying the visible pill color.
    let mut output = (color * mask * pill.alpha).extend(alpha);

    if pill.rating >= 0 && pill.primary_alpha > 0.0 && near_row(pixel_pos, primary_row) {
        let rating = pill.rating as f32;
        for index in 0..5 {
            let fill = ((rating - index as f32 * 2.0) * 0.5).clamp(0.0, 1.0);
            let center = primary_row.icon_center(index as f32);
            if !near_icon(pixel_pos, center) {
                continue;
            }
            let (local_uv, local_pixel, pixel_radius, _) = icon_local(pixel_pos, center, global);
            let dist =
                sd_star(local_pixel, pixel_radius * 0.5, pixel_radius * 0.32) - pixel_radius * 0.1;
            let split_line = local_uv.x - fill;
            let selection_mask = (split_line / split_line.fwidth() + 0.5).clamp(0.0, 1.0);
            let color = vec3(1.0, 0.85, 0.2).lerp(vec3(0.33, 0.33, 0.33), selection_mask);
            let overlay = icon_overlay(color, dist, pill.primary_alpha);
            output = over_premul(output, overlay, pill.alpha);
        }
    }

    // Playlist artwork icons.
    if near_row(pixel_pos, primary_row) || near_row(pixel_pos, secondary_row) {
        let stars = pill.star_count();
        let primary_playlists = pill.primary_playlist_count as usize;
        let playlist_count = (primary_playlists + pill.secondary_playlist_count as usize)
            .min(MAX_PILL_PLAYLIST_ICONS);
        for index in 0..playlist_count {
            let image_index = pill.playlist_images[index];
            if image_index < 0 {
                continue;
            }

            let primary_icon = index < primary_playlists;
            let (row, icon_index, alpha) = if primary_icon {
                (primary_row, index as f32 + stars, pill.primary_alpha)
            } else {
                (
                    secondary_row,
                    (index - primary_playlists) as f32,
                    pill.secondary_expansion,
                )
            };
            if alpha <= 0.0 {
                continue;
            }
            let center = row.icon_center(icon_index);
            if !near_icon(pixel_pos, center) {
                continue;
            }
            let (local_uv, local_pixel, pixel_radius, mouse_distance) =
                icon_local(pixel_pos, center, global);
            let desaturation = if primary_icon
                || (global.mouse_pressure > 0.0 && mouse_distance <= ICON_WIDTH * 0.5)
            {
                0.0
            } else {
                0.2
            };
            let dist = sd_squircle(local_pixel, Vec2::splat(pixel_radius * 0.6), 6.0);
            let tex = images.sample(*sampler, local_uv.extend(image_index as f32));
            let overlay = icon_overlay(
                tex.truncate().lerp(Vec3::splat(0.24), desaturation),
                dist,
                alpha,
            );
            output = over_premul(output, overlay, pill.alpha);
        }
    }

    if output.w <= 0.0 {
        kill();
    }
    *out_color = output;
}
