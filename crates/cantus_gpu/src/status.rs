use crate::{pill_fragment, pill_vertex, sd_rounded_box};
use cantus_shared::{GlobalUniforms, StatusPill, smoothstep};
use spirv_std::{
    arch::kill,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

fn outline(point: Vec2, size: Vec2, radius: f32) -> f32 {
    smoothstep(1.3, 0.2, sd_rounded_box(point, size, radius).abs())
}

fn fill(point: Vec2, size: Vec2, radius: f32) -> f32 {
    smoothstep(0.7, -0.7, sd_rounded_box(point, size, radius))
}

fn ring(point: Vec2, radius: f32) -> f32 {
    smoothstep(1.8, 0.0, (point.length() - radius).abs())
}

fn system(p: Vec2, size: Vec2) -> Vec3 {
    let y = size.y * 0.5;
    let cpu = p - vec2(13.0, y);
    let mut icons = outline(cpu, Vec2::splat(7.0), 1.8);
    icons = icons.max(fill(cpu, Vec2::splat(2.6), 0.8));

    let memory = p - vec2(86.0, y);
    for index in 0..3 {
        let bar = memory - vec2(index as f32 * 4.7 - 4.7, 0.0);
        icons = icons.max(fill(bar, Vec2::splat(1.3), 0.5));
    }
    icons = icons.max(outline(memory, vec2(8.0, 5.5), 1.8));

    let gpu = p - vec2(149.0, y);
    icons = icons
        .max(outline(gpu, vec2(8.0, 6.0), 1.8))
        .max(smoothstep(1.2, 0.2, (gpu.length() - 3.8).abs()))
        .max(smoothstep(1.3, 0.4, gpu.length()));

    let vram = p - vec2(239.0, y);
    for index in 0..3 {
        icons = icons.max(fill(
            vram - vec2(0.0, index as f32 * 5.0 - 5.0),
            vec2(8.0, 1.0),
            1.0,
        ));
    }
    Vec3::splat(0.075 + icons * 0.82)
}

fn controls(p: Vec2, size: Vec2, pill: StatusPill) -> Vec3 {
    let mut color = Vec3::ZERO;

    if pill.battery[1] > 0.5 {
        let center = vec2(23.0, size.y * 0.5);
        let shell = sd_rounded_box(p - center, vec2(13.0, 8.0), 3.0);
        let inside = 1.0 - smoothstep(-1.0, 0.0, shell);
        let edge = 1.0 - smoothstep(0.0, 1.2, shell.abs());
        let level = pill.battery[0] * 22.0;
        let fill = smoothstep(level, level - 2.0, p.x - 12.0) * inside;
        color += vec3(0.25, 0.95, 0.58) * fill * 0.65 + Vec3::splat(edge * 0.45);
    }

    let volume_x = if pill.battery[1] > 0.5 { 82.0 } else { 18.0 };
    let radius = 8.0 + pill.volume[0] * 8.0;
    color += vec3(0.26, 0.66, 1.0)
        * ring(p - vec2(volume_x, size.y * 0.5), radius)
        * (1.0 - pill.volume[1] * 0.75);

    let power = p - vec2(size.x - 54.0, size.y * 0.5);
    let power_ring = ring(power, 8.0) * smoothstep(-1.0, 2.5, power.y);
    let power_stem = smoothstep(1.6, 0.0, power.x.abs()) * smoothstep(7.0, 2.0, power.y.abs());
    color += vec3(1.0, 0.42, 0.45) * power_ring.max(power_stem) * 0.9;

    let reboot = p - vec2(size.x - 20.0, size.y * 0.5);
    let reboot_ring = ring(reboot, 8.0) * (1.0 - smoothstep(-2.0, 1.5, reboot.x + reboot.y));
    let arrow = smoothstep(2.2, 0.0, (reboot - vec2(5.5, -5.5)).length());
    color += vec3(0.42, 0.72, 1.0) * reboot_ring.max(arrow) * 0.9;
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
