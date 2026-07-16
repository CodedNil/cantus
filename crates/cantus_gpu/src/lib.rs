#![no_std]

use cantus_shared::GlobalUniforms;
use spirv_std::glam::{Vec2, Vec3, Vec4, vec2, vec3, vec4};

pub mod particles;
pub mod playhead;
pub mod status;
pub mod text;
pub mod track;
pub mod weather;

pub const fn quad_coord(vertex_index: u32) -> Vec2 {
    vec2((vertex_index & 1) as f32, (vertex_index >> 1) as f32)
}

pub fn pixel_to_ndc(pixel: Vec2, screen_size: Vec2) -> Vec4 {
    let ndc = pixel / screen_size * 2.0 - 1.0;
    vec4(ndc.x, -ndc.y, 0.0, 1.0)
}

pub fn pill_vertex(vertex: u32, global: &GlobalUniforms, x: f32, width: f32) -> (Vec4, Vec2) {
    let pixel =
        vec2(x, global.bar_height.x) + quad_coord(vertex) * vec2(width, global.bar_height.y);
    (pixel_to_ndc(pixel, global.screen_size), pixel)
}

pub fn pill_fragment(
    pixel: Vec2,
    global: &GlobalUniforms,
    x: f32,
    width: f32,
    bulge: f32,
) -> (Vec2, Vec2, f32) {
    let size = vec2(width, global.bar_height.y);
    let local = pixel - vec2(x, global.bar_height.x);
    let distance = sd_capsule_box(
        local - size * 0.5,
        (size.x - size.y) * 0.5,
        (size.y + bulge) * 0.5,
    );
    (local, size, distance)
}

/// Return a direction and length without `glam::normalize_or_zero`, whose infinity literal is rejected by Naga when translating SPIR-V.
pub fn direction_and_length(vector: Vec2) -> (Vec2, f32) {
    let length = vector.length();
    let direction = if length > 0.001 {
        vector / length
    } else {
        Vec2::ZERO
    };
    (direction, length)
}

pub fn sd_rounded_box(point: Vec2, half_size: Vec2, radius: f32) -> f32 {
    let corner = point.abs() - half_size + radius;
    corner.max(Vec2::ZERO).length() + corner.x.max(corner.y).min(0.0) - radius
}

pub fn sd_capsule_box(point: Vec2, half_span: f32, radius: f32) -> f32 {
    let offset = point.abs() - vec2(half_span, 0.0);
    offset.max(Vec2::ZERO).length() + offset.x.max(offset.y).min(0.0) - radius
}

pub fn sd_star(point: Vec2, radius: f32, indent: f32) -> f32 {
    let k1 = vec2(0.809_017, -0.587_785_25);
    let k2 = vec2(-k1.x, k1.y);
    let mut point = vec2(point.x.abs(), -point.y);
    point -= 2.0 * k1.dot(point).max(0.0) * k1;
    point -= 2.0 * k2.dot(point).max(0.0) * k2;
    point.x = point.x.abs();
    point.y -= radius;
    let edge = indent * vec2(-k1.y, k1.x) - vec2(0.0, radius);
    let edge_t = (point.dot(edge) / edge.dot(edge)).clamp(0.0, 1.0);
    let cross = point.y * edge.x - point.x * edge.y;
    (point - edge * edge_t).length() * if cross < 0.0 { -1.0 } else { 1.0 }
}

pub fn sd_rounded_triangle(point: Vec2, side_len: f32, radius: f32) -> f32 {
    let k = 1.732_050_8;
    let mut point = vec2(point.x.abs(), point.y);
    let h = (point.x + k * point.y).max(0.0);
    point -= 0.5 * vec2(h, h * k);
    point -= vec2(
        point.x.clamp(
            -0.5 * (side_len - radius) * k,
            0.5 * (side_len - radius) * k,
        ),
        -0.5 * (side_len - radius),
    );
    point.length() * if point.y > 0.0 { -1.0 } else { 1.0 } - radius
}

pub fn smooth_union(base: f32, shape: f32, smoothing: f32, amount: f32) -> f32 {
    let blend = (0.5 + 0.5 * (shape - base) / smoothing).clamp(0.0, 1.0);
    let union = shape + (base - shape) * blend - smoothing * blend * (1.0 - blend);
    base + (union - base) * amount
}

pub fn unpack3x8unorm(value: u32) -> Vec3 {
    vec3(
        (value & 0xff) as f32 / 255.0,
        ((value >> 8) & 0xff) as f32 / 255.0,
        ((value >> 16) & 0xff) as f32 / 255.0,
    )
}
