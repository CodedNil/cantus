use crate::{pill_fragment, pill_vertex, sd_rounded_box};
use cantus_shared::{GlobalUniforms, StatusPill, smoothstep};
use spirv_std::{
    arch::kill,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

fn shape(point: Vec2, size: Vec2, radius: f32) -> f32 {
    smoothstep(1.0, -1.0, sd_rounded_box(point, size, radius))
}

fn control_icon(point: Vec2, center: Vec2) -> f32 {
    let local = point - center;
    smoothstep(1.8, 0.0, (local.length() - 8.0).abs()).max(shape(
        local - vec2(0.0, -5.0),
        vec2(1.2, 5.0),
        0.5,
    ))
}

fn system(p: Vec2, size: Vec2) -> Vec3 {
    let y = size.y * 0.5;
    let centers = [13.0, 86.0, 149.0, 239.0];
    let mut icons: f32 = 0.0;
    #[allow(clippy::needless_range_loop)] // Rust-GPU cannot lower array iterators to SPIR-V.
    for index in 0..4 {
        let local = p - vec2(centers[index], y);
        let box_icon = shape(local, vec2(7.0, 5.0), 1.5);
        let detail = if index & 1 == 0 {
            smoothstep(1.5, 0.0, (local.length() - 3.0).abs())
        } else {
            shape(local, vec2(2.0, 6.5), 0.8)
        };
        icons = icons.max(box_icon * 0.45 + detail);
    }
    Vec3::splat(0.075 + icons * 0.82)
}

fn controls(p: Vec2, size: Vec2, pill: StatusPill) -> Vec3 {
    let mut color = Vec3::ZERO;

    if pill.battery[1] > 0.5 {
        let center = vec2(23.0, size.y * 0.5);
        let shell = sd_rounded_box(p - center, vec2(13.0, 8.0), 3.0);
        let level = pill.battery[0] * 22.0;
        let fill = smoothstep(level, level - 2.0, p.x - 12.0) * smoothstep(0.0, -1.0, shell);
        color += vec3(0.25, 0.95, 0.58) * fill * 0.65
            + Vec3::splat(smoothstep(1.2, 0.0, shell.abs()) * 0.45);
    }

    let volume_x = if pill.battery[1] > 0.5 { 82.0 } else { 18.0 };
    color += vec3(0.26, 0.66, 1.0)
        * smoothstep(
            1.8,
            0.0,
            ((p - vec2(volume_x, size.y * 0.5)).length() - 8.0 - pill.volume[0] * 8.0).abs(),
        )
        * (1.0 - pill.volume[1] * 0.75);

    let y = size.y * 0.5;
    color += vec3(1.0, 0.42, 0.45) * control_icon(p, vec2(size.x - 54.0, y)) * 0.9;
    color += vec3(0.42, 0.72, 1.0) * control_icon(p, vec2(size.x - 20.0, y)) * 0.9;
    color
}

#[spirv(vertex)]
pub fn vs_status(
    #[spirv(vertex_index)] vertex: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] status: &[StatusPill],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_pixel: &mut Vec2,
) {
    let pill = status[0];
    (*out_pos, *out_pixel) = pill_vertex(vertex, global, pill.x, pill.width);
}

#[spirv(fragment)]
pub fn fs_status(
    #[spirv(location = 0)] pixel: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] status: &[StatusPill],
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = status[0];
    let (local, size, dist) = pill_fragment(pixel, global, pill.x, pill.width, 0.0);
    let alpha = (1.0 - smoothstep(-0.6, 0.6, dist)) * 0.5;
    if alpha <= 0.0 {
        kill();
    }
    let controls_x = size.x - 190.0;
    let color =
        system(local, size) + controls(local - vec2(controls_x, 0.0), vec2(190.0, size.y), pill);
    *out_color = ((color + (1.0 - smoothstep(0.0, -3.0, dist)) * 0.08) * alpha).extend(alpha);
}
