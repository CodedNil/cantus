use crate::common::{
    pixel_to_ndc, quad_coord, sd_capsule_box, sd_squircle, sd_star, smooth_union, unpack3x8unorm,
};
use cantus_shared::{
    BACKPLATE_RADIUS, BackgroundPill, GlobalUniforms, ICON_WIDTH, MAX_PILL_PLAYLIST_ICONS,
    smoothstep,
};
use spirv_std::{
    Sampler,
    arch::{Derivative, kill},
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    image::Image2dArray,
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

fn repeat_art_uv(uv: Vec2) -> Vec2 {
    vec2(
        1.0 - ((uv.x * 0.5).fract() * 2.0 - 1.0).abs(),
        uv.y.clamp(0.0, 1.0),
    )
}

/// Returns direction and length without `glam::normalize_or_zero`, whose `is_finite` check emits an infinity literal that Naga rejects for this SPIR-V.
fn direction_and_length(vec: Vec2) -> (Vec2, f32) {
    let length = vec.length();
    let direction = if length > 0.001 {
        vec / length
    } else {
        Vec2::ZERO
    };
    (direction, length)
}

fn icon_local(pixel_pos: Vec2, center: Vec2, global: &GlobalUniforms) -> (Vec2, Vec2, f32) {
    let pressure = global.mouse_pressure.clamp(0.001, 1.0);
    let proximity = smoothstep(30.0, 8.0, center.distance(global.mouse_pos) / pressure);
    let pixel_radius = ICON_WIDTH * 0.5 * (1.05 + 0.63 * proximity);
    let x_push = (center.x - global.mouse_pos.x) * proximity * 0.5;
    let local = pixel_pos - (center + vec2(x_push, 0.0));
    let angle = -x_push * 0.01;
    let sin = angle.sin();
    let cos = angle.cos();
    let local = vec2(local.x * cos - local.y * sin, local.x * sin + local.y * cos);
    (local / (pixel_radius * 2.0) + 0.5, local, pixel_radius)
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

    let (ripple_dir, ripple_strength, ripple_flash) = {
        let anim_t = (global.time - global.expansion_time) * 1.2;
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
    };

    let (mouse_dir, mouse_d, mouse_inf) = {
        let (mouse_dir, mouse_distance) = direction_and_length(pixel_pos - global.mouse_pos);
        let influence = smoothstep(120.0, 0.0, mouse_distance);
        (
            mouse_dir,
            mouse_distance,
            influence * influence * global.mouse_pressure,
        )
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

    // Procedural colour and shared warp.
    let seed = (pill.colors[0] % 1000) as f32 * 29.537;
    let lens_warp = (1.0 + dist.min(0.0) / 120.0).clamp(0.0, 1.0);
    let lens_warp = lens_warp * lens_warp * 0.6;
    let deformation =
        local_centered * lens_warp + ripple_dir * ripple_strength + mouse_dir * mouse_inf * 0.03;
    let mut color = {
        let palette_uv = (local_pixel / global.screen_size.y - deformation)
            * 0.2
            * vec2(1.0, global.screen_size.y / global.screen_size.x)
            + seed;
        let flow_seed = global.time * 0.15 + seed;
        let s1 =
            (palette_uv.x * 6.0 + flow_seed + (palette_uv.y * 4.0 + flow_seed * 0.5).sin()).sin();
        let s2 =
            (palette_uv.y * 5.0 - flow_seed + (palette_uv.x * 3.0 + flow_seed * 0.8).sin()).sin();
        let c0 = unpack3x8unorm(pill.colors[0]);
        let c1 = unpack3x8unorm(pill.colors[1]);
        let c2 = unpack3x8unorm(pill.colors[2]);
        let c3 = unpack3x8unorm(pill.colors[3]);

        c0.lerp(c1, s1 * 0.5 + 0.5)
            .lerp(c2.lerp(c3, s2 * 0.5 + 0.5), 0.5)
            .lerp((c0 + c1 + c2 + c3) * 0.25, 0.1)
    };

    // Blurred album-art wash behind the procedural colour.
    if pill.image_index >= 0 {
        let art_uv = vec2(local_uv.x, stretched_uv_y);
        let art_aspect = vec2(pill_size.y / pill_size.x, 1.0);
        color = color.lerp(
            images
                .sample(
                    *sampler,
                    repeat_art_uv(art_uv - deformation * art_aspect)
                        .extend(pill.image_index as f32 + 1.0),
                )
                .truncate(),
            0.3,
        );
    }

    // Apply color saturation and darkening.
    color = {
        let luma = color.dot(vec3(0.2126, 0.7152, 0.0722));
        let saturation = 3.2 - 1.6 * smoothstep(0.1, 0.4, luma);
        let color = Vec3::splat(luma)
            .lerp(color, saturation)
            .clamp(Vec3::splat(0.06), Vec3::splat(0.85))
            * 1.0f32.min(0.52 / luma.max(0.001));
        color.lerp(
            color * 0.6,
            smoothstep(
                global.playhead_x + 3.0,
                global.playhead_x - 3.0,
                pixel_pos.x,
            ),
        )
    };

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

    if pill.rating >= 0 {
        let mut rating = pill.rating as f32;
        let (index, right_half) = primary_row.hit(global.mouse_pos);
        if global.mouse_pressure > 0.0 && (0..5).contains(&index) {
            rating = index as f32 * 2.0 + 1.0 + u32::from(right_half) as f32;
        }
        for index in 0..5 {
            let fill = ((rating - index as f32 * 2.0) * 0.5).clamp(0.0, 1.0);
            let center = primary_row.icon_center(index as f32);
            let (local_uv, local_pixel, pixel_radius) = icon_local(pixel_pos, center, global);
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
    let stars = pill.star_count();
    let primary_playlists = pill.primary_playlist_count as usize;
    let playlist_count =
        (primary_playlists + pill.secondary_playlist_count as usize).min(MAX_PILL_PLAYLIST_ICONS);
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
        let center = row.icon_center(icon_index);
        let desaturation = if primary_icon
            || (global.mouse_pressure > 0.0
                && center.distance(global.mouse_pos) <= ICON_WIDTH * 0.5)
        {
            0.0
        } else {
            0.2
        };

        let (local_uv, local_pixel, pixel_radius) = icon_local(pixel_pos, center, global);
        let dist = sd_squircle(local_pixel, Vec2::splat(pixel_radius * 0.6), 6.0);
        let tex = images.sample(*sampler, local_uv.extend(image_index as f32));
        let overlay = icon_overlay(
            tex.truncate().lerp(Vec3::splat(0.24), desaturation),
            dist,
            alpha,
        );
        output = over_premul(output, overlay, pill.alpha);
    }

    if output.w <= 0.0 {
        kill();
    }
    *out_color = output;
}
