use crate::render::shared::{FrameData, smoothstep};
use isthmus::glam::{FloatExt, UVec2, Vec2, Vec4, uvec2, vec2, vec4};
use spirv_std::arch::Derivative;

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

/// Core 2-lane avalanche mixer for hash functions
pub fn avalanche(mut value: UVec2) -> UVec2 {
    value = value
        .wrapping_mul(UVec2::splat(1_664_525))
        .wrapping_add(UVec2::splat(1_013_904_223));
    value.x = value.x.wrapping_add(value.y.wrapping_mul(1_664_525));
    value.y = value.y.wrapping_add(value.x.wrapping_mul(1_664_525));
    value ^= value >> 16;
    value.x = value.x.wrapping_add(value.y.wrapping_mul(1_664_525));
    value.y = value.y.wrapping_add(value.x.wrapping_mul(1_664_525));
    value ^= value >> 16;
    value
}

#[isthmus::outline]
pub fn hash(p: Vec2) -> Vec2 {
    let value = avalanche(uvec2(p.x as i32 as u32, p.y as i32 as u32));
    vec2(value.x as f32, value.y as f32) * 2.328_306_4e-10
}

#[isthmus::outline]
fn simplex_noise(p: Vec2) -> f32 {
    const K1: f32 = 0.366_025_42;
    const K2: f32 = 0.211_324_87;
    let cell = (p + (p.x + p.y) * K1).floor();
    let a = p - cell + (cell.x + cell.y) * K2;
    let corner = if a.x > a.y { vec2(1.0, 0.0) } else { vec2(0.0, 1.0) };
    let b = a - corner + K2;
    let c = a - 1.0 + 2.0 * K2;
    let contribution = |offset: Vec2, point: Vec2| {
        let falloff = (0.5 - point.dot(point)).max(0.0);
        falloff * falloff * falloff * falloff * point.dot(hash(cell + offset) * 2.0 - 1.0)
    };
    70.0 * (contribution(Vec2::ZERO, a) + contribution(corner, b) + contribution(Vec2::ONE, c))
}

pub fn fbm(mut p: Vec2) -> f32 {
    let mut density = 0.0;
    let mut amplitude = 0.5;
    let mut octave = 0;
    while octave < 4 {
        density += simplex_noise(p) * amplitude;
        p = vec2(p.x * 1.6 + p.y * 1.2, p.y * 1.6 - p.x * 1.2);
        amplitude *= 0.5;
        octave += 1;
    }
    0.5 + density * 0.5
}

pub fn cloud_mass(p: Vec2, scale: f32, time: f32) -> f32 {
    fbm(p / scale * 0.14 + vec2(time * 0.012, 6.1))
}

pub const fn quad_coord(vertex_index: u32) -> Vec2 {
    vec2((vertex_index & 1) as f32, (vertex_index >> 1) as f32)
}

#[isthmus::outline]
pub fn pixel_to_ndc(pixel: Vec2, screen_size: Vec2) -> Vec4 {
    let ndc = pixel / screen_size * 2.0 - 1.0;
    vec4(ndc.x, -ndc.y, 0.0, 1.0)
}

pub fn pill_vertex(vertex: u32, frame: &FrameData, x: f32, size: Vec2) -> (Vec4, Vec2) {
    let pixel = vec2(x - 48.0, frame.panel_top - 48.0)
        + quad_coord(vertex) * (size + vec2(96.0, frame.panel_height + 96.0));
    (pixel_to_ndc(pixel, frame.screen_size), pixel)
}

pub fn pill_fragment(
    pixel: Vec2,
    frame: &FrameData,
    x: f32,
    width: f32,
) -> (PillInteraction, Vec2, Vec2, SdfSurface) {
    let size = vec2(width, frame.panel_height);
    let local = pixel - vec2(x, frame.panel_top);
    let distance = sd_capsule_box(local - size * 0.5, (size.x - size.y) * 0.5, size.y * 0.5);
    let mouse = frame.mouse_pos - vec2(x, frame.panel_top) - size * 0.5;
    let mouse_distance = sd_capsule_box(mouse, (size.x - size.y) * 0.5, size.y * 0.5);
    (
        pill_interaction(pixel, frame),
        local,
        size,
        SdfSurface::new(distance, mouse_distance),
    )
}

/// Return a direction and length without `glam::normalize_or_zero`, whose infinity literal is rejected by Naga when translating SPIR-V.
pub fn direction_and_length(vector: Vec2) -> (Vec2, f32) {
    let length = vector.length();
    if length > 0.001 {
        (vector / length, length)
    } else {
        (Vec2::ZERO, length)
    }
}

/// Shared hover/click deformation used by every pill-shaped surface.
#[derive(Clone, Copy)]
pub struct PillInteraction {
    mouse_bulge: f32,
    pub ripple: Vec2,
    pub ripple_flash: f32,
}

#[derive(Clone, Copy)]
pub struct SdfSurface {
    pub distance: f32,
    pub mouse_distance: f32,
}

impl SdfSurface {
    pub const fn new(distance: f32, mouse_distance: f32) -> Self {
        Self {
            distance,
            mouse_distance,
        }
    }

    pub fn smooth_union(self, other: Self, radius: f32, blend: f32) -> Self {
        Self {
            distance: smooth_union(self.distance, other.distance, radius, blend),
            mouse_distance: smooth_union(self.mouse_distance, other.mouse_distance, radius, blend),
        }
    }
}

impl PillInteraction {
    pub fn bulge(self, surface: SdfSurface) -> f32 {
        self.mouse_bulge * hover_mask(surface.mouse_distance) + self.ripple.length() * 22.0
    }

    /// Apply the shared hover/click expansion to an assembled signed-distance field.
    pub fn expand(self, surface: SdfSurface) -> f32 {
        surface.distance - self.bulge(surface) * 0.5
    }

    /// Return the expanded distance, fill coverage, and combined fill/shadow alpha.
    pub fn surface(self, surface: SdfSurface) -> (f32, f32, f32) {
        let distance = self.expand(surface);
        let mask = sdf_coverage(distance);
        let shadow = (-distance.max(0.0) * 0.3).exp() * 0.16;
        (distance, mask, mask.max(shadow))
    }

    pub fn refract(self, local: Vec2, size: Vec2, distance: f32) -> Vec2 {
        let uv = local / size;
        uv - (uv - 0.5) * (1.0 + distance.min(0.0) / 120.0).clamp(0.0, 0.6) * 0.08 - self.ripple * 0.04
    }
}

pub fn hover_mask(mouse_distance: f32) -> f32 {
    smoothstep(0.5, -0.5, mouse_distance)
}

/// Derivative-aware coverage for an anti-aliased signed-distance edge.
pub fn sdf_coverage(distance: f32) -> f32 {
    let width = distance.fwidth().max(0.55);
    smoothstep(width, -width, distance)
}

pub fn pill_interaction(pixel: Vec2, frame: &FrameData) -> PillInteraction {
    let mut ripple = Vec2::ZERO;
    let mut ripple_flash = 0.0;
    let mut index = 0;
    while index < frame.ripples.len() {
        let pulse = frame.ripples[index];
        let progress = ((frame.time - pulse.animation.x) * 1.2).saturate();
        let (direction, distance) = direction_and_length(pixel - pulse.origin);
        let wave = smoothstep(80.0, 0.0, (distance - progress * 600.0).abs())
            * pulse.animation.y
            * (1.0 - progress);
        ripple += direction * wave * (1.0 - progress) * 0.5;
        ripple_flash = (ripple_flash + wave * 0.5).min(1.0);
        index += 1;
    }

    let mouse_bulge = if frame.mouse_pressure > 0.0 {
        smoothstep(150.0, 0.0, pixel.distance(frame.mouse_pos)) * frame.mouse_pressure * 8.0
    } else {
        0.0
    };
    PillInteraction {
        mouse_bulge,
        ripple,
        ripple_flash,
    }
}

pub fn pill_sheen(uv_y: f32, distance: f32) -> f32 {
    smoothstep(0.12, 0.0, uv_y) * 0.12 + smoothstep(5.0, -3.0, distance) * 0.08
}

#[isthmus::outline]
pub fn sd_rounded_box(point: Vec2, half_size: Vec2, radius: f32) -> f32 {
    let corner = point.abs() - half_size + radius;
    corner.max(Vec2::ZERO).length() + corner.x.max(corner.y).min(0.0) - radius
}

#[isthmus::outline]
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
    let edge_t = (point.dot(edge) / edge.dot(edge)).saturate();
    let cross = point.y * edge.x - point.x * edge.y;
    (point - edge * edge_t).length() * if cross < 0.0 { -1.0 } else { 1.0 }
}

pub fn sd_rounded_triangle(point: Vec2, side_len: f32, radius: f32) -> f32 {
    let k = 1.732_050_8;
    let mut point = vec2(point.x.abs(), point.y);
    let h = (point.x + k * point.y).max(0.0);
    point -= 0.5 * vec2(h, h * k);
    point -= vec2(
        point
            .x
            .clamp(-0.5 * (side_len - radius) * k, 0.5 * (side_len - radius) * k),
        -0.5 * (side_len - radius),
    );
    point.length() * if point.y > 0.0 { -1.0 } else { 1.0 } - radius
}

/// Shortest distance from `point` to the line segment between `start` and `end`.
pub fn segment_distance(point: Vec2, start: Vec2, end: Vec2) -> f32 {
    let segment = end - start;
    let along = ((point - start).dot(segment) / segment.dot(segment).max(0.001)).saturate();
    (point - start - segment * along).length()
}

const ANTIALIAS_WIDTH: f32 = 0.55;

/// Antialiased coverage of the outline of a shape at the given signed `distance`, `width` pixels wide.
pub fn stroke(distance: f32, width: f32) -> f32 {
    smoothstep(width + ANTIALIAS_WIDTH, width - ANTIALIAS_WIDTH, distance.abs())
}

/// Antialiased coverage of the inside of a shape at the given signed `distance`.
pub fn fill(distance: f32) -> f32 {
    smoothstep(ANTIALIAS_WIDTH, -ANTIALIAS_WIDTH, distance)
}

/// "‹" chevron with its tip at the origin, spanning to `extent` and its mirror; negate `extent.x` for a "›".
pub fn sd_chevron(point: Vec2, extent: Vec2) -> f32 {
    segment_distance(point, Vec2::ZERO, extent).min(segment_distance(
        point,
        Vec2::ZERO,
        vec2(extent.x, -extent.y),
    ))
}

pub fn smooth_union(base: f32, shape: f32, smoothing: f32, amount: f32) -> f32 {
    let blend = (0.5 + 0.5 * (shape - base) / smoothing).saturate();
    let union = shape + (base - shape) * blend - smoothing * blend * (1.0 - blend);
    base + (union - base) * amount
}
