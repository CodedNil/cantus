use crate::common::{
    pixel_to_ndc, quad_coord, sd_capsule_box, sd_squircle, sd_star, smooth_union, smoothstep,
    unpack3x8unorm,
};
use cantus_shared::{
    BACKPLATE_HOVER_GROWTH, BACKPLATE_RADIUS, BackgroundPill, GlobalUniforms,
    ICON_HITBOX_HALF_SIZE, ICON_SPACING, MAX_PILL_PLAYLIST_ICONS, pill_icon_primary_center_y,
};
use core::f32::consts::PI;
use spirv_std::{
    Sampler,
    arch::{Derivative, kill},
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    image::Image2dArray,
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

const ART_SHARED_WARP: f32 = 0.4;
const ICON_QUAD_RADIUS: f32 = 12.6;
const ICON_HOVER_SCALE: f32 = 0.6;
const PRIMARY_SMOOTHING: f32 = 30.0;

fn repeat_art_uv(uv: Vec2) -> Vec2 {
    vec2(
        1.0 - ((uv.x * 0.5).fract() * 2.0 - 1.0).abs(),
        uv.y.clamp(0.0, 1.0),
    )
}

fn primary_fade(pill: &BackgroundPill) -> f32 {
    let primary_icon_count = pill.primary_icon_count();
    if primary_icon_count <= 0.0 {
        return pill.secondary_expansion;
    }
    let needed_width = ICON_SPACING * primary_icon_count * 0.7;
    let width_fade = ((pill.width - needed_width) / (needed_width * 0.5)).clamp(0.0, 1.0);
    width_fade.max(pill.secondary_expansion)
}

fn icon_local(pixel_pos: Vec2, center: Vec2, global: &GlobalUniforms) -> (Vec2, Vec2, f32) {
    let pressure = global.mouse_pressure.clamp(0.001, 1.0);
    let proximity = smoothstep(30.0, 8.0, center.distance(global.mouse_pos) / pressure);
    let pixel_radius = ICON_QUAD_RADIUS * (1.0 + ICON_HOVER_SCALE * proximity);
    let x_push = (center.x - global.mouse_pos.x) * proximity * 0.5;
    let local = pixel_pos - (center + vec2(x_push, 0.0));
    let local = if proximity > 0.0 {
        let angle = -x_push * 0.01;
        let sin = angle.sin();
        let cos = angle.cos();
        vec2(local.x * cos - local.y * sin, local.x * sin + local.y * cos)
    } else {
        local
    };
    (local / (pixel_radius * 2.0) + 0.5, local, pixel_radius)
}

fn star_overlay(
    pixel_pos: Vec2,
    center: Vec2,
    fill: f32,
    tint: Vec3,
    alpha: f32,
    global: &GlobalUniforms,
) -> Vec4 {
    let (local_uv, local_pixel, pixel_radius) = icon_local(pixel_pos, center, global);
    let dist = sd_star(local_pixel, pixel_radius * 0.5, pixel_radius * 0.32) - pixel_radius * 0.1;
    let split_line = local_uv.x - fill;
    let selection_mask = (split_line / split_line.fwidth() + 0.5).clamp(0.0, 1.0);
    let color = vec3(1.0, 0.85, 0.2)
        .lerp(vec3(0.33, 0.33, 0.33), selection_mask)
        .lerp(tint, 0.16);
    icon_overlay(color, dist, alpha)
}

fn icon_overlay(color: Vec3, dist: f32, alpha: f32) -> Vec4 {
    let mask = (0.5 - dist).clamp(0.0, 1.0);
    let shadow = 1.0 - smoothstep(0.0, 6.0, dist);
    let shadow = shadow * shadow * 0.2;
    if mask <= 0.0 && shadow <= 0.0 {
        return Vec4::ZERO;
    }
    let highlighting = 1.0 - smoothstep(0.0, -5.0, dist);
    let highlighting2 = highlighting * highlighting;
    let highlighting = highlighting2 * highlighting2 * 0.04;
    ((color + highlighting) * mask * alpha).extend(mask.max(shadow) * alpha)
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

    let icon_radius = BACKPLATE_RADIUS + BACKPLATE_HOVER_GROWTH + ICON_HITBOX_HALF_SIZE * 0.25;
    let (primary_row, secondary_row) = pill.icon_rows(pill_icon_primary_center_y(
        global.bar_height.x,
        global.bar_height.y,
    ));
    let icon_half_size = primary_row
        .half_size(icon_radius)
        .max(secondary_row.half_size(icon_radius))
        * primary_fade(&pill);
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
    let pill_origin = vec2(pill.x, global.bar_height.x);
    let local_pixel = pixel_pos - pill_origin;
    let local_uv = local_pixel / pill_size;
    let world_uv = local_pixel / global.screen_size.y;

    let anim_t = (global.time - global.expansion_time) * 1.2;
    let ripple_active = if anim_t * (1.0 - anim_t) >= 0.0 {
        1.0
    } else {
        0.0
    };
    let ripple_vec = pixel_pos - global.expansion_xy;
    let (ripple_dir, wave, ripple_str) = if ripple_active > 0.0 {
        let direction = (ripple_vec + 0.001).normalize();
        let wave_dist = (ripple_vec.length() - anim_t * 600.0).abs();
        let wave = smoothstep(80.0, 0.0, wave_dist);
        let ripple_decay = 1.0 - anim_t;
        (direction, wave, ripple_decay * ripple_decay * wave * 0.5)
    } else {
        (Vec2::ZERO, 0.0, 0.0)
    };

    let mouse_vec = pixel_pos - global.mouse_pos;
    let mouse_d = mouse_vec.length();
    let mouse_inf = smoothstep(120.0, 0.0, mouse_d);
    let mouse_inf = mouse_inf * mouse_inf * global.mouse_pressure;
    let mouse_pull = (mouse_vec + 0.001).normalize() * mouse_inf * 15.0;

    let bulge = ripple_str * 22.0 + mouse_inf * 8.0;
    let stretched_uv_y = (local_uv.y - 0.5) * (pill_size.y / (pill_size.y + bulge)) + 0.5;
    let icon_center_y = pill_icon_primary_center_y(global.bar_height.x, global.bar_height.y);
    let local_icon_growth = mouse_inf * BACKPLATE_HOVER_GROWTH;
    let primary_fade = primary_fade(&pill);
    let (primary_row, secondary_row) = pill.icon_rows(icon_center_y);
    let pill_dist = sd_squircle(
        (local_uv - 0.5) * pill_size,
        (pill_size + vec2(0.0, bulge)) * 0.5,
        BACKPLATE_RADIUS + ICON_HITBOX_HALF_SIZE,
    );
    let mut dist = pill_dist;
    if primary_fade > 0.0 {
        let primary_dist = sd_capsule_box(
            pixel_pos - primary_row.backplate_center(),
            primary_row.padded_half_span(),
            BACKPLATE_RADIUS + local_icon_growth,
        );
        dist = smooth_union(dist, primary_dist, PRIMARY_SMOOTHING, primary_fade);
    }
    if pill.secondary_expansion > 0.0 {
        let secondary_dist = sd_capsule_box(
            pixel_pos - secondary_row.backplate_center(),
            secondary_row.padded_half_span(),
            BACKPLATE_RADIUS + local_icon_growth,
        );
        dist = smooth_union(
            dist,
            secondary_dist,
            ICON_HITBOX_HALF_SIZE,
            pill.secondary_expansion,
        );
    }
    let mask = (0.5 - dist).clamp(0.0, 1.0);
    let shadow = (1.0 - smoothstep(0.0, 14.0, dist)) * 0.16;

    let seed = (pill.colors[0] % 1000) as f32 * 29.537;
    let t = global.time * 0.15 + seed;
    let lens_warp = (1.0 + dist.min(0.0) / 120.0).clamp(0.0, 1.0);
    let lens_warp = lens_warp * lens_warp * 0.6;
    let deformation = (local_uv - 0.5) * lens_warp + ripple_dir * ripple_str + mouse_pull * 0.002;
    let uv = world_uv - deformation;
    let p = uv * 0.2 * vec2(1.0, global.screen_size.y / global.screen_size.x) + seed;
    let s1 = (p.x * 6.0 + t + (p.y * 4.0 + t * 0.5).sin()).sin();
    let s2 = (p.y * 5.0 - t + (p.x * 3.0 + t * 0.8).sin()).sin();
    let c0 = unpack3x8unorm(pill.colors[0]);
    let c1 = unpack3x8unorm(pill.colors[1]);
    let c2 = unpack3x8unorm(pill.colors[2]);
    let c3 = unpack3x8unorm(pill.colors[3]);

    let mut color = c0
        .lerp(c1, s1 * 0.5 + 0.5)
        .lerp(c2.lerp(c3, s2 * 0.5 + 0.5), 0.5);
    color = color.lerp((c0 + c1 + c2 + c3) * 0.25, 0.1);

    // Blend in a softly blurred version of the album art.
    if pill.image_index >= 0 {
        // Stretch the blurred art across the full pill and add a slowly rotating flow field.
        let art_uv = vec2(local_uv.x, stretched_uv_y);
        let art_aspect = vec2(pill_size.y / pill_size.x, 1.0);
        let flow_pos = (art_uv - 0.5) / art_aspect;
        let flow_time = global.time * 0.55 + seed;
        let edge_fade = (stretched_uv_y * PI).sin().max(0.0);
        let flow = vec2(
            (flow_pos.y * 4.0 + flow_time + (flow_pos.x * 0.7 - flow_time * 0.6).sin()).sin()
                * 0.18,
            (flow_pos.x + flow_time + (flow_pos.y * 5.0 + flow_time).cos()).cos()
                * 0.14
                * edge_fade,
        );
        let shared_warp = deformation * art_aspect * -ART_SHARED_WARP;
        let flow_uv = flow * art_aspect;
        let rotated_flow_uv = vec2(-flow.y * art_aspect.x, flow.x);
        let layer = pill.image_index as f32 + 1.0;
        let art_sample = images.sample(
            *sampler,
            repeat_art_uv(art_uv + shared_warp + flow_uv).extend(layer),
        ) * 0.65
            + images.sample(
                *sampler,
                repeat_art_uv(
                    vec2(1.0 - art_uv.x, art_uv.y) + shared_warp + rotated_flow_uv * 0.75,
                )
                .extend(layer),
            ) * 0.35;
        color = color.lerp(art_sample.truncate(), 0.3);
    }

    let luma = color.dot(vec3(0.2126, 0.7152, 0.0722));
    let saturation = 3.2 + (1.6 - 3.2) * smoothstep(0.1, 0.4, luma);
    color = Vec3::splat(luma).lerp(color, saturation);
    color = color.clamp(Vec3::splat(0.06), Vec3::splat(0.85)) * 1.0f32.min(0.52 / luma.max(0.001));
    // Darken anything to the left of the playhead
    color = color.lerp(
        color * 0.6,
        smoothstep(
            global.playhead_x + 3.0,
            global.playhead_x - 3.0,
            pixel_pos.x,
        ),
    );

    let img_x = pill_size.x - pill_size.y;
    let local_x = local_pixel.x;
    let uv_img = vec2((local_x - img_x) / pill_size.y, stretched_uv_y);
    if pill.image_index >= 0 && local_x >= img_x {
        let tex = images.sample(*sampler, uv_img.extend(pill.image_index as f32));
        let img_mask = 1.0
            - smoothstep(
                -0.5,
                0.5,
                sd_squircle(
                    (uv_img - 0.5) * pill_size.y,
                    vec2(pill_size.y * 0.5, pill_size.y * 0.5),
                    BACKPLATE_RADIUS + ICON_HITBOX_HALF_SIZE,
                ),
            );
        color = color.lerp(tex.truncate(), img_mask * tex.w);
    }

    let sheen = smoothstep(0.1, 0.0, stretched_uv_y) * mask * 0.15;
    let rim = (1.0 - smoothstep(0.0, -6.0, dist)) * 0.1;
    let mouse_sheen = smoothstep(30.0, 0.0, (mouse_d - 15.0).abs()) * mouse_inf * 0.2;
    color += color.lerp(Vec3::ONE, 0.3) * (sheen + rim + mouse_sheen);

    color = color.lerp(
        color * 1.5 + 0.1,
        (1.0 - anim_t) * wave * ripple_active * 0.5,
    );

    let alpha = mask.max(shadow) * pill.alpha;
    // Keep shadows black while premultiplying the visible pill color.
    let mut output = (color * mask * pill.alpha).extend(alpha);

    let stars = pill.star_count();
    if pill.rating >= 0 && primary_row.count > 0.0 {
        let mut rating = pill.rating as f32;
        if global.mouse_pressure > 0.0 {
            for index in 0..5 {
                if let Some(center) = primary_row.hit_icon(index as f32, global.mouse_pos) {
                    rating = index as f32 * 2.0
                        + 1.0
                        + if global.mouse_pos.x >= center.x {
                            1.0
                        } else {
                            0.0
                        };
                }
            }
        }
        for index in 0..5 {
            let fill = ((rating - index as f32 * 2.0) * 0.5).clamp(0.0, 1.0);
            let center = primary_row.icon_center(index as f32);
            output = over_premul(
                output,
                star_overlay(pixel_pos, center, fill, color, primary_fade, global),
                pill.alpha,
            );
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

        let (row_start, row, alpha, icon_offset) = if index < primary_playlists {
            (0, primary_row, primary_fade, stars)
        } else {
            (
                primary_playlists,
                secondary_row,
                pill.secondary_expansion,
                0.0,
            )
        };
        let icon_index = (index - row_start) as f32 + icon_offset;
        let center = row.icon_center(icon_index);
        let desaturation = if index < primary_playlists
            || (global.mouse_pressure > 0.0
                && center.distance(global.mouse_pos) <= ICON_HITBOX_HALF_SIZE)
        {
            0.0
        } else {
            0.2
        };

        let (local_uv, local_pixel, pixel_radius) = icon_local(pixel_pos, center, global);
        let dist = sd_squircle(
            local_pixel,
            vec2(pixel_radius * 0.6, pixel_radius * 0.6),
            6.0,
        );
        let tex = images.sample(*sampler, local_uv.extend(image_index as f32));
        let color = tex.truncate().lerp(Vec3::splat(0.24), desaturation);
        output = over_premul(output, icon_overlay(color, dist, alpha), pill.alpha);
    }

    if output.w <= 0.0 {
        kill();
    }
    *out_color = output;
}
