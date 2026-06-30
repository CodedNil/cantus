use core::f32::consts::PI;

use crate::common::{
    pixel_to_ndc, quad_coord, sd_capsule_box, sd_squircle, smooth_union, smoothstep, unpack4x8unorm,
};
use cantus_shared::{BackgroundPill, GlobalUniforms};
use spirv_std::{
    Sampler,
    arch::kill,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    image::Image2dArray,
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

const ICON_RADIUS: f32 = 7.0;
const ICON_GROWTH: f32 = 8.0;
const ICON_END_PADDING: f32 = 3.0;
const ICON_ROW_GAP: f32 = 20.0;
const PRIMARY_SMOOTHING: f32 = 5.0;
const SECONDARY_SMOOTHING: f32 = 8.0;

#[spirv(vertex)]
pub fn vs_background(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(instance_index)] i_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] pills: &[BackgroundPill],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_local_uv: &mut Vec2,
    #[spirv(location = 1)] out_world_uv: &mut Vec2,
    #[spirv(location = 2, flat)] out_pill_idx: &mut u32,
    #[spirv(location = 3)] out_pixel_pos: &mut Vec2,
) {
    let pill = pills[i_idx as usize];
    let margin = 16.0;
    let unit_coord = quad_coord(v_idx);
    let pill_size = vec2(pill.rect.y, global.bar_height.y);
    let pill_origin = vec2(pill.rect.x, global.bar_height.x);

    let mut render_min = pill_origin - margin;
    let mut render_max = pill_origin + pill_size + margin;
    if pill.icon_span.z > 0.0 || pill.icon_span.w > 0.0 {
        let primary_span = pill.icon_span.x * pill.icon_span.w;
        let secondary_span = pill.icon_span.y * pill.icon_span.z;
        let icon_center_x = pill_origin.x + pill_size.x * 0.5;
        let primary_center_y = global.bar_height.x + global.bar_height.y * 0.975;
        let lowest_center_y = primary_center_y + ICON_ROW_GAP * pill.icon_span.z;
        let max_icon_radius = (ICON_RADIUS + ICON_GROWTH) * global.scale_factor;
        let icon_half_width = primary_span.max(secondary_span)
            + ICON_END_PADDING * global.scale_factor
            + max_icon_radius;
        let union_margin = SECONDARY_SMOOTHING * 0.25 * global.scale_factor;
        render_min.x = render_min
            .x
            .min(icon_center_x - icon_half_width - union_margin);
        render_max.x = render_max
            .x
            .max(icon_center_x + icon_half_width + union_margin);
        render_max.y = render_max
            .y
            .max(lowest_center_y + max_icon_radius + union_margin);
    }

    let pixel_pos = render_min + unit_coord * (render_max - render_min);
    let local_pixel = pixel_pos - pill_origin;
    *out_pos = pixel_to_ndc(pixel_pos, global.screen_size);
    *out_local_uv = local_pixel / pill_size;
    *out_world_uv = local_pixel / global.screen_size.y;
    *out_pill_idx = i_idx;
    *out_pixel_pos = pixel_pos;
}

#[spirv(fragment)]
pub fn fs_background(
    #[spirv(location = 0)] local_uv: Vec2,
    #[spirv(location = 1)] world_uv: Vec2,
    #[spirv(location = 2, flat)] pill_idx: u32,
    #[spirv(location = 3)] pixel_pos: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] pills: &[BackgroundPill],
    #[spirv(descriptor_set = 0, binding = 2)] images: &Image2dArray,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = pills[pill_idx as usize];
    let pill_size = vec2(pill.rect.y, global.bar_height.y);
    let rounding = 22.0 * global.scale_factor;

    let anim_t = (global.time - global.expansion_time) * 1.2;
    let ripple_started = if anim_t < 0.0 { 0.0 } else { 1.0 };
    let ripple_unfinished = if anim_t > 1.0 { 0.0 } else { 1.0 };
    let ripple_active = ripple_started * ripple_unfinished;
    let ripple_vec = pixel_pos - global.expansion_xy;
    let ripple_dir = (ripple_vec + 0.001).normalize();
    let wave_dist = (ripple_vec.length() - anim_t * 600.0).abs();
    let wave_prof = if wave_dist <= 80.0 {
        0.5 + 0.5 * (wave_dist / 80.0 * PI).cos()
    } else {
        0.0
    };
    let ripple_decay = 1.0 - anim_t;
    let ripple_str = ripple_decay * ripple_decay * wave_prof * 0.5 * ripple_active;

    let mouse_vec = pixel_pos - global.mouse_pos;
    let mouse_d = mouse_vec.length();
    let mouse_inf = smoothstep(120.0 * global.scale_factor, 0.0, mouse_d);
    let mouse_inf = mouse_inf * mouse_inf * global.mouse_pressure;
    let mouse_pull = (mouse_vec + 0.001).normalize() * mouse_inf * 15.0 * global.scale_factor;

    let bulge = ripple_str * 22.0 * global.scale_factor + mouse_inf * 8.0;
    let stretched_uv_y = (local_uv.y - 0.5) * (pill_size.y / (pill_size.y + bulge)) + 0.5;
    let pill_dist = sd_squircle(
        (local_uv - 0.5) * pill_size,
        (pill_size + vec2(0.0, bulge)) * 0.5,
        rounding,
    );
    let icon_center_x = pill.rect.x + pill.rect.y * 0.5;
    let primary_center_y = global.bar_height.x + global.bar_height.y * 0.975;
    let smoothing = PRIMARY_SMOOTHING * global.scale_factor;
    let secondary_smoothing = SECONDARY_SMOOTHING * global.scale_factor;
    let primary_center = vec2(icon_center_x, primary_center_y);
    let local_icon_growth = mouse_inf * ICON_GROWTH * global.scale_factor;
    let primary_dist = sd_capsule_box(
        pixel_pos - primary_center,
        pill.icon_span.x + ICON_END_PADDING * global.scale_factor,
        ICON_RADIUS * global.scale_factor + local_icon_growth,
    );
    let dist = smooth_union(pill_dist, primary_dist, smoothing, pill.icon_span.w);
    let secondary_center = vec2(
        icon_center_x,
        primary_center_y + ICON_ROW_GAP * pill.icon_span.z,
    );
    let secondary_dist = sd_capsule_box(
        pixel_pos - secondary_center,
        pill.icon_span.y * pill.icon_span.z + ICON_END_PADDING * global.scale_factor,
        ICON_RADIUS * global.scale_factor + local_icon_growth,
    );
    let dist = smooth_union(dist, secondary_dist, secondary_smoothing, pill.icon_span.z);
    let mask = (0.5 - dist).clamp(0.0, 1.0);
    let main_shadow = (1.0 - smoothstep(0.0, 16.0, pill_dist)) * 0.2;
    let primary_shadow = (1.0 - smoothstep(0.0, 6.0, primary_dist)) * 0.08 * pill.icon_span.w;
    let secondary_shadow = (1.0 - smoothstep(0.0, 6.0, secondary_dist)) * 0.08 * pill.icon_span.z;
    let shadow = main_shadow.max(primary_shadow).max(secondary_shadow);
    if mask <= 0.0 && shadow <= 0.0 {
        kill();
    }

    let seed = (pill.color0 % 1000) as f32 * 29.537;
    let t = global.time * 0.15 + seed;
    let lens_warp = (1.0 + dist.min(0.0) / 120.0).clamp(0.0, 1.0);
    let lens_warp = lens_warp * lens_warp * 0.6;
    let uv = world_uv - (local_uv - 0.5) * lens_warp - ripple_dir * ripple_str - mouse_pull * 0.002;
    let p = uv * 0.2 * vec2(1.0, global.screen_size.y / global.screen_size.x) + seed;
    let s1 = (p.x * 6.0 + t + (p.y * 4.0 + t * 0.5).sin()).sin();
    let s2 = (p.y * 5.0 - t + (p.x * 3.0 + t * 0.8).sin()).sin();
    let mix_val = (s1 * 0.5 + s2 * 0.3 + (p.length() * 4.0 + s1 + t).sin() * 0.2) * 0.5 + 0.5;
    let mix_val = mix_val.clamp(0.0, 1.0);

    let c0 = unpack4x8unorm(pill.color0).truncate();
    let c1 = unpack4x8unorm(pill.color1).truncate();
    let c2 = unpack4x8unorm(pill.color2).truncate();
    let c3 = unpack4x8unorm(pill.color3).truncate();

    let mut color = c0
        .lerp(c1, mix_val)
        .lerp(c3.lerp(c2, s2 * 0.5 + 0.5), mix_val);
    color = color.lerp((c0 + c1 + c2 + c3) * 0.25, 0.1);
    let luma = color.dot(vec3(0.2126, 0.7152, 0.0722));
    let saturation = 3.2 + (1.6 - 3.2) * smoothstep(0.1, 0.4, luma);
    color = Vec3::splat(luma).lerp(color, saturation);
    color = color.clamp(Vec3::splat(0.06), Vec3::splat(0.85)) * 1.0f32.min(0.52 / luma.max(0.001));
    color = color.lerp(
        color * 0.45,
        smoothstep(
            global.playhead_x + 1.2,
            global.playhead_x - 1.2,
            pixel_pos.x,
        ),
    );

    let img_x = pill_size.x - pill_size.y;
    let local_x = local_uv.x * pill_size.x;
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
                    rounding,
                ),
            );
        color = color.lerp(tex.truncate(), img_mask * tex.w);
    }

    let sheen = smoothstep(0.1, 0.0, stretched_uv_y) * mask * 0.15;
    let rim = (1.0 - smoothstep(0.0, -6.0, dist)) * 0.1;
    let mouse_sheen = smoothstep(30.0, 0.0, (mouse_d - 15.0).abs()) * mouse_inf * 0.2;
    color += color.lerp(Vec3::ONE, 0.3) * (sheen + rim + mouse_sheen);

    let stretched_local_uv = vec2(local_uv.x, stretched_uv_y);
    let glint = smoothstep(60.0, 0.0, (stretched_local_uv * pill_size - 20.0).length()) * 0.1
        + smoothstep(
            60.0,
            0.0,
            (stretched_local_uv * pill_size - (pill_size - 20.0)).length(),
        ) * 0.05;
    color += glint * mask;
    color = color.lerp(
        color * 1.5 + 0.1,
        (1.0 - anim_t) * smoothstep(80.0, 0.0, wave_dist) * ripple_active * 0.5,
    );

    let alpha = mask.max(shadow) * pill.alpha;
    // Keep shadows black while premultiplying the visible pill color.
    *out_color = (color * mask * pill.alpha).extend(alpha);
}
