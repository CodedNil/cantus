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
const PRIMARY_SMOOTHING: f32 = 7.0;
const SECONDARY_SMOOTHING: f32 = 10.0;

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
    let pill_size = vec2(pill.rect.y, global.bar_height.y);
    let pill_origin = vec2(pill.rect.x, global.bar_height.x);

    let primary_fade = pill.icon_span.w.fract() * 2.0;
    let icon_visibility = primary_fade.max(pill.icon_span.z);
    let icon_center_x = pill_origin.x + pill_size.x * 0.5;
    let icon_radius = (ICON_RADIUS + ICON_GROWTH + SECONDARY_SMOOTHING * 0.25)
        * global.scale_factor
        * icon_visibility;
    let icon_half_width = ((pill.icon_span.x - 1.0)
        .max(pill.icon_span.y * pill.icon_span.z)
        .max(0.0)
        + ICON_END_PADDING * global.scale_factor)
        * icon_visibility
        + icon_radius;
    let render_min = vec2(
        (pill_origin.x - margin).min(icon_center_x - icon_half_width),
        pill_origin.y - margin,
    );
    let render_max = vec2(
        (pill_origin.x + pill_size.x + margin).max(icon_center_x + icon_half_width),
        (pill_origin.y + pill_size.y + margin).max(
            global.bar_height.x
                + global.bar_height.y * 0.975
                + ICON_ROW_GAP * pill.icon_span.z
                + icon_radius,
        ),
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
    let pill_size = vec2(pill.rect.y, global.bar_height.y);
    let pill_origin = vec2(pill.rect.x, global.bar_height.x);
    let local_pixel = pixel_pos - pill_origin;
    let local_uv = local_pixel / pill_size;
    let world_uv = local_pixel / global.screen_size.y;
    let scale = global.scale_factor;
    let rounding = 22.0 * scale;

    let anim_t = (global.time - global.expansion_time) * 1.2;
    let ripple_active = if anim_t * (1.0 - anim_t) >= 0.0 {
        1.0
    } else {
        0.0
    };
    let ripple_vec = pixel_pos - global.expansion_xy;
    let ripple_dir = (ripple_vec + 0.001).normalize();
    let wave_dist = (ripple_vec.length() - anim_t * 600.0).abs();
    let wave = smoothstep(80.0, 0.0, wave_dist);
    let ripple_decay = 1.0 - anim_t;
    let ripple_str = ripple_decay * ripple_decay * wave * 0.5 * ripple_active;

    let mouse_vec = pixel_pos - global.mouse_pos;
    let mouse_d = mouse_vec.length();
    let mouse_inf = smoothstep(120.0 * scale, 0.0, mouse_d);
    let mouse_inf = mouse_inf * mouse_inf * global.mouse_pressure;
    let mouse_pull = (mouse_vec + 0.001).normalize() * mouse_inf * 15.0 * scale;

    let bulge = ripple_str * 22.0 * scale + mouse_inf * 8.0;
    let stretched_uv_y = (local_uv.y - 0.5) * (pill_size.y / (pill_size.y + bulge)) + 0.5;
    let pill_dist = sd_squircle(
        (local_uv - 0.5) * pill_size,
        (pill_size + vec2(0.0, bulge)) * 0.5,
        rounding,
    );
    let icon_center_x = pill.rect.x + pill.rect.y * 0.5;
    let primary_center_y = global.bar_height.x + global.bar_height.y * 0.975;
    let local_icon_growth = mouse_inf * ICON_GROWTH * scale;
    let primary_dist = sd_capsule_box(
        pixel_pos - vec2(icon_center_x, primary_center_y),
        (pill.icon_span.x - 1.0).max(0.0) + ICON_END_PADDING * scale,
        ICON_RADIUS * scale + local_icon_growth,
    );
    let primary_fade = pill.icon_span.w.fract() * 2.0;
    let dist = smooth_union(
        pill_dist,
        primary_dist,
        PRIMARY_SMOOTHING * scale,
        primary_fade,
    );
    let secondary_center = vec2(
        icon_center_x,
        primary_center_y + ICON_ROW_GAP * pill.icon_span.z,
    );
    let secondary_dist = sd_capsule_box(
        pixel_pos - secondary_center,
        pill.icon_span.y * pill.icon_span.z + ICON_END_PADDING * scale,
        ICON_RADIUS * scale + local_icon_growth,
    );
    let dist = smooth_union(
        dist,
        secondary_dist,
        SECONDARY_SMOOTHING * scale,
        pill.icon_span.z,
    );
    let mask = (0.5 - dist).clamp(0.0, 1.0);
    let shadow = (1.0 - smoothstep(0.0, 14.0, dist)) * 0.16;
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
    let c0 = unpack4x8unorm(pill.color0).truncate();
    let c1 = unpack4x8unorm(pill.color1).truncate();
    let c2 = unpack4x8unorm(pill.color2).truncate();
    let c3 = unpack4x8unorm(pill.color3).truncate();

    let mut color = c0
        .lerp(c1, s1 * 0.5 + 0.5)
        .lerp(c2.lerp(c3, s2 * 0.5 + 0.5), 0.5);
    color = color.lerp((c0 + c1 + c2 + c3) * 0.25, 0.1);
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

    color = color.lerp(
        color * 1.5 + 0.1,
        (1.0 - anim_t) * wave * ripple_active * 0.5,
    );

    let alpha = mask.max(shadow) * pill.alpha;
    // Keep shadows black while premultiplying the visible pill color.
    *out_color = (color * mask * pill.alpha).extend(alpha);
}
