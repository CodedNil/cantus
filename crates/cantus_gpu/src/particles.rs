use crate::common::{pixel_to_ndc, smoothstep, unpack4x8unorm};
use cantus_shared::{GlobalUniforms, Particle};
use spirv_std::{
    arch::kill,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

#[spirv(vertex)]
pub fn vs_particles(
    #[spirv(vertex_index)] v_idx: u32,
    #[spirv(instance_index)] i_idx: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] particles: &[Particle],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_color: &mut Vec4,
    #[spirv(location = 1)] out_uv: &mut Vec2,
) {
    let p = particles[i_idx as usize];
    let color_vec = unpack4x8unorm(p.color);
    let rgb = color_vec.truncate();
    let duration = (color_vec.w * 255.0) / 100.0;
    let spawn_time = p.end_time - duration;
    let dt = global.time - spawn_time;

    if dt < 0.0 || dt > duration {
        *out_pos = Vec4::ZERO;
        *out_color = Vec4::ZERO;
        *out_uv = Vec2::ZERO;
        return;
    }

    let p_life = dt / duration;
    let p_life_inv = 1.0 - p_life;
    let scale = global.scale_factor;
    let pos = p.spawn_pos + p.spawn_vel * dt * scale;
    let dir = (p.spawn_vel * scale).normalize();
    let perp = vec2(-dir.y, dir.x);
    let growth = p_life + 0.5;
    let half_len = 5.0 * scale * growth;
    let half_thick = 2.5 * scale * growth;
    let uv = match v_idx {
        0 => vec2(-1.0, -1.0),
        1 => vec2(1.0, -1.0),
        2 => vec2(-1.0, 1.0),
        _ => vec2(1.0, 1.0),
    };
    let world_pos = pos + (dir * uv.x * half_len) + (perp * uv.y * half_thick);
    let luma = rgb.dot(vec3(0.299, 0.587, 0.114));
    let spark_color = Vec3::splat(luma).lerp(rgb, 2.0).lerp(Vec3::ONE, 0.2) * 2.0;

    *out_pos = pixel_to_ndc(world_pos, global.screen_size);
    *out_color = spark_color.extend(p_life_inv * smoothstep(0.0, 0.15, dt) * 0.3);
    *out_uv = uv;
}

#[spirv(fragment)]
pub fn fs_particles(
    #[spirv(location = 0)] color: Vec4,
    #[spirv(location = 1)] uv: Vec2,
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let dist = (uv * vec2(0.8, 1.0)).length();
    let alpha = color.w * smoothstep(1.0, 0.2, dist);
    if alpha <= 0.0 {
        kill();
    }
    *out_color = (color.truncate() * alpha).extend(alpha);
}
