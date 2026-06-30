use crate::common::{mix_vec3, sd_squircle, sign_no_nan, smoothstep, unpack2x16unorm};
use cantus_shared::{GlobalUniforms, IconInstance};
use spirv_std::{
    Sampler,
    arch::{Derivative, kill},
    glam::{Vec2, Vec4, vec2, vec3, vec4},
    image::Image2dArray,
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

#[spirv(vertex)]
pub fn vs_icons(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(instance_index)] i_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] icons: &[IconInstance],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_local_uv: &mut Vec2,
    #[spirv(location = 1, flat)] out_icon_id: &mut u32,
    #[spirv(location = 2)] out_pixel_radius: &mut f32,
) {
    let icon = icons[i_idx as usize];
    let unit_coord = vec2((v_idx % 2) as f32, (v_idx / 2) as f32);
    let pressure = global.mouse_pressure.clamp(0.001, 1.0);
    let dist = icon.pos.distance(global.mouse_pos) / global.scale_factor / pressure;
    let proximity = smoothstep(30.0, 8.0, dist);
    let growth = 1.0 + (0.6 * proximity);
    let pixel_radius = 9.0 * global.scale_factor * growth;
    let x_push = (icon.pos.x - global.mouse_pos.x) * proximity * 0.5;
    let offset_pos = icon.pos + vec2(x_push, 0.0);
    let angle = x_push * 0.01;
    let rotation = (unit_coord - 0.5) * (pixel_radius * 2.0);
    let rotated_pos = vec2(
        rotation.x * angle.cos() - rotation.y * angle.sin(),
        rotation.x * angle.sin() + rotation.y * angle.cos(),
    );
    let screen_pixel = offset_pos + rotated_pos;
    let ndc_pos = (screen_pixel / global.screen_size) * 2.0 - 1.0;
    *out_pos = vec4(ndc_pos.x, -ndc_pos.y, 0.0, 1.0);
    *out_local_uv = unit_coord;
    *out_icon_id = i_idx;
    *out_pixel_radius = pixel_radius;
}

fn sd_star(p: Vec2, radius: f32, indent: f32) -> f32 {
    let k1 = vec2(0.809_017, -0.587_785_25);
    let k2 = vec2(-k1.x, k1.y);
    let mut p_sym = vec2(p.x, -p.y);
    p_sym.x = p_sym.x.abs();
    p_sym -= 2.0 * k1.dot(p_sym).max(0.0) * k1;
    p_sym -= 2.0 * k2.dot(p_sym).max(0.0) * k2;
    p_sym.x = p_sym.x.abs();
    p_sym.y -= radius;
    let ba = indent * vec2(-k1.y, k1.x) - vec2(0.0, radius);
    let h = (p_sym.dot(ba) / ba.dot(ba)).clamp(0.0, 1.0);
    (p_sym - ba * h).length() * sign_no_nan(p_sym.y * ba.x - p_sym.x * ba.y)
}

#[spirv(fragment)]
pub fn fs_icons(
    #[spirv(location = 0)] local_uv: Vec2,
    #[spirv(location = 1, flat)] icon_id: u32,
    #[spirv(location = 2)] pixel_radius: f32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] icons: &[IconInstance],
    #[spirv(descriptor_set = 0, binding = 2)] images: &Image2dArray,
    #[spirv(descriptor_set = 0, binding = 3)] sampler: &Sampler,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let icon = icons[icon_id as usize];
    let local_pixel = (local_uv - 0.5) * (pixel_radius * 2.0);
    let data = unpack2x16unorm(icon.data);
    let param = data.x;
    let alpha = data.y;

    let (mut color, dist_to_shape) = if param >= 0.5 {
        let dist = sd_star(local_pixel, pixel_radius * 0.5, pixel_radius * 0.32)
            - pixel_radius * 0.1 * global.scale_factor;
        let star_fullness = (param - 0.5) * 2.0;
        let split_line = local_uv.x - star_fullness;
        let selection_mask = (split_line / split_line.fwidth() + 0.5).clamp(0.0, 1.0);
        (
            mix_vec3(vec3(1.0, 0.85, 0.2), vec3(0.33, 0.33, 0.33), selection_mask),
            dist,
        )
    } else {
        let dist = sd_squircle(
            local_pixel,
            vec2(pixel_radius * 0.6, pixel_radius * 0.6),
            6.0 * global.scale_factor,
        );
        let tex = images.sample(*sampler, local_uv.extend(icon.image_index as f32));
        let icon_saturation = if param > 0.0 { 0.7 } else { 0.0 };
        (
            mix_vec3(tex.truncate(), vec3(0.24, 0.24, 0.24), icon_saturation),
            dist,
        )
    };

    let mask = (0.5 - dist_to_shape).clamp(0.0, 1.0);
    let shadow = 1.0 - smoothstep(0.0, 6.0, dist_to_shape);
    let shadow = shadow * shadow * 0.2;
    if mask <= 0.0 && shadow <= 0.0 {
        kill();
    }
    let highlighting = 1.0 - smoothstep(0.0, -5.0, dist_to_shape);
    let highlighting2 = highlighting * highlighting;
    let highlighting = highlighting2 * highlighting2 * 0.04;
    color += highlighting * mask;
    *out_color = vec4(
        color.x * mask * alpha,
        color.y * mask * alpha,
        color.z * mask * alpha,
        mask.max(shadow) * alpha,
    );
}
