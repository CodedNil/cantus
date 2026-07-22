use crate::{direction_and_length, pixel_to_ndc, quad_coord, unpack3x8unorm};
use cantus_shared::{GlobalUniforms, Particle, smoothstep};
use spirv_std::{
    arch::kill,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

#[spirv(vertex)]
pub fn vs_particles(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(instance_index)] i_idx: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] particles: &[Particle],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_color: &mut Vec4,
    #[spirv(location = 1)] out_uv: &mut Vec2,
) {
    let p = particles[i_idx as usize];
    let duration = ((p.color >> 24) & 0xff) as f32 / 100.0;
    let dt = global.time - (p.end_time - duration);

    if dt < 0.0 || dt > duration {
        *out_pos = Vec4::ZERO;
        *out_color = Vec4::ZERO;
        *out_uv = Vec2::ZERO;
        return;
    }

    let p_life = dt / duration;
    let rgb = unpack3x8unorm(p.color);
    let (dir, _) = direction_and_length(p.spawn_vel);
    let uv = quad_coord(v_idx) * 2.0 - 1.0;
    let extent = uv * vec2(5.0, 2.5) * (p_life + 0.5);
    let world_pos = p.spawn_pos + p.spawn_vel * dt + dir * extent.x + dir.perp() * extent.y;
    let luma = rgb.dot(vec3(0.299, 0.587, 0.114));
    let spark_color = Vec3::splat(luma).lerp(rgb, 2.0).lerp(Vec3::ONE, 0.2) * 2.0;

    *out_pos = pixel_to_ndc(world_pos, global.screen_size);
    *out_color = spark_color.extend((1.0 - p_life) * smoothstep(0.0, 0.15, dt) * 0.3);
    *out_uv = uv;
}

#[spirv(fragment)]
pub fn fs_particles(
    #[spirv(location = 0)] color: Vec4,
    #[spirv(location = 1)] uv: Vec2,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let alpha = color.w * smoothstep(1.0, 0.2, (uv * vec2(0.8, 1.0)).length());
    if alpha <= 0.0 {
        kill();
    }
    *out_color = (color.truncate() * alpha).extend(alpha);
}
