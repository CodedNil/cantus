use spirv_std::glam::{Vec2, Vec3, vec2};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

pub fn mix_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn mix_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn step(edge: f32, x: f32) -> f32 {
    if x < edge { 0.0 } else { 1.0 }
}

pub fn sign_no_nan(x: f32) -> f32 {
    if x < 0.0 { -1.0 } else { 1.0 }
}

pub fn sd_squircle(p: Vec2, half_size: Vec2, radius: f32) -> f32 {
    let q = p.abs() - half_size + radius;
    (q.x.max(0.0).powf(4.0) + q.y.max(0.0).powf(4.0)).powf(0.25) - radius + q.x.max(q.y).min(0.0)
}

pub fn unpack4x8unorm(value: u32) -> spirv_std::glam::Vec4 {
    spirv_std::glam::vec4(
        (value & 0xff) as f32 / 255.0,
        ((value >> 8) & 0xff) as f32 / 255.0,
        ((value >> 16) & 0xff) as f32 / 255.0,
        ((value >> 24) & 0xff) as f32 / 255.0,
    )
}

pub fn unpack2x16unorm(value: u32) -> Vec2 {
    vec2(
        (value & 0xffff) as f32 / 65535.0,
        ((value >> 16) & 0xffff) as f32 / 65535.0,
    )
}
