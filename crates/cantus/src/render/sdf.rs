use crate::render::Globals;
use isthmus::{
    FloatExt, Quad,
    glam::{UVec2, Vec2, Vec3, Vec4, uvec2, vec2},
    spirv_std::arch::Derivative,
};

/// Where the drop shadow fades below the fragment kill threshold, plus an AA pixel.
const SHADOW_REACH: f32 = 18.0;
const ANTIALIAS_WIDTH: f32 = 0.55;
/// Smallest coverage worth shading; analytic shadows never reach exact zero.
pub const VISIBLE_ALPHA: f32 = 1.0 / 1024.0;

pub fn sample_pill(quad: Quad, pixel: Vec2, globals: Globals, time: f32) -> PillSample {
    let size = quad.size;
    let local = quad.local(pixel) + size * 0.5;
    let distance = sd_capsule_box(local - size * 0.5, (size.x - size.y) * 0.5, size.y * 0.5);
    let mouse_distance = if globals.pressure > 0.0 {
        sd_capsule_box(quad.local(globals.pointer), (size.x - size.y) * 0.5, size.y * 0.5)
    } else {
        1.0
    };
    let (mouse_bulge, ripple_bulge, ripple, ripple_flash) = interaction(pixel, globals, time);
    PillSample::new(local, size, distance, mouse_distance, mouse_bulge, ripple_bulge, ripple, ripple_flash)
}

#[derive(Clone, Copy)]
pub struct PillSample {
    shape_distance: f32,
    mouse_distance: f32,
    mouse_bulge: f32,
    ripple_bulge: f32,
    /// Unmodified top-left-relative surface coordinates.
    pub local: Vec2,
    /// Ripple- and edge-refracted top-left-relative coordinates.
    pub refracted: Vec2,
    pub size: Vec2,
    pub distance: f32,
    pub mask: f32,
    pub alpha: f32,
    pub ripple: Vec2,
    pub ripple_flash: f32,
}

impl PillSample {
    fn new(local: Vec2, size: Vec2, shape_distance: f32, mouse_distance: f32, mouse_bulge: f32, ripple_bulge: f32, ripple: Vec2, ripple_flash: f32) -> Self {
        let mut surface = Self {
            shape_distance,
            mouse_distance,
            mouse_bulge,
            ripple_bulge,
            local,
            refracted: local,
            size,
            distance: shape_distance,
            mask: 0.0,
            alpha: 0.0,
            ripple,
            ripple_flash,
        };
        surface.resolve();
        surface
    }

    fn resolve(&mut self) {
        self.distance = self.shape_distance - self.bulge() * 0.5;
        let width = self.distance.fwidth().max(ANTIALIAS_WIDTH);
        self.mask = self.distance.smoothstep(width, -width);
        let shadow = (-self.distance.max(0.0) * 0.3).exp() * 0.16;
        self.alpha = self.mask.max(shadow);
        let uv = self.local / self.size;
        self.refracted = (uv - (uv - 0.5) * (1.0 + self.distance.min(0.0) / 120.0).clamp(0.0, 0.6) * 0.08 - self.ripple * 0.04) * self.size;
    }

    pub fn union(mut self, shape_distance: f32, mouse_distance: f32, smoothing: f32, amount: f32) -> Self {
        self.shape_distance = smooth_union(self.shape_distance, shape_distance, smoothing, amount);
        self.mouse_distance = smooth_union(self.mouse_distance, mouse_distance, smoothing, amount);
        self.resolve();
        self
    }

    pub fn uv(self) -> Vec2 {
        self.local / self.size
    }

    pub fn refracted_uv(self) -> Vec2 {
        self.refracted / self.size
    }

    pub fn bulge(self) -> f32 {
        self.mouse_bulge * self.mouse_distance.smoothstep(0.5, -0.5) + self.ripple_bulge
    }

    pub fn color(self, color: Vec3) -> Vec4 {
        (color * self.mask).extend(self.alpha)
    }
}

/// Core 2-lane avalanche mixer for hash functions
pub fn avalanche(mut value: UVec2) -> UVec2 {
    value = value.wrapping_mul(UVec2::splat(1_664_525)).wrapping_add(UVec2::splat(1_013_904_223));
    value.x = value.x.wrapping_add(value.y.wrapping_mul(1_664_525));
    value.y = value.y.wrapping_add(value.x.wrapping_mul(1_664_525));
    value ^= value >> 16;
    value.x = value.x.wrapping_add(value.y.wrapping_mul(1_664_525));
    value.y = value.y.wrapping_add(value.x.wrapping_mul(1_664_525));
    value ^= value >> 16;
    value
}

pub fn hash(p: Vec2) -> Vec2 {
    let value = avalanche(uvec2(p.x as i32 as u32, p.y as i32 as u32));
    vec2(value.x as f32, value.y as f32) * 2.328_306_4e-10
}

// This substantial helper is shared across shader entry points; keeping it as
// one SPIR-V function avoids duplicating the complete noise graph in each one.
#[inline(never)]
pub fn simplex_noise(p: Vec2) -> f32 {
    const K1: f32 = 0.366_025_42;
    const K2: f32 = 0.211_324_87;
    let cell = (p + (p.x + p.y) * K1).floor();
    let a = p - cell + (cell.x + cell.y) * K2;
    let corner = if a.x > a.y { vec2(1.0, 0.0) } else { vec2(0.0, 1.0) };
    let b = a - corner + K2;
    let c = a - 1.0 + 2.0 * K2;
    let contribution = |offset: Vec2, point: Vec2| {
        let falloff = (0.5 - point.length_squared()).max(0.0);
        falloff * falloff * falloff * falloff * point.dot(hash(cell + offset) * 2.0 - 1.0)
    };
    70.0 * (contribution(Vec2::ZERO, a) + contribution(corner, b) + contribution(Vec2::ONE, c))
}

pub fn fbm(mut p: Vec2) -> f32 {
    let mut density = 0.0;
    let mut amplitude = 0.5;
    for _ in 0..4 {
        density += simplex_noise(p) * amplitude;
        p = vec2(p.x * 1.6 + p.y * 1.2, p.y * 1.6 - p.x * 1.2);
        amplitude *= 0.5;
    }
    0.5 + density * 0.5
}

pub fn cloud_mass(p: Vec2, scale: f32, time: f32) -> f32 {
    fbm(p / scale * 0.14 + vec2(time * 0.012, 6.1))
}

/// Pixels the pill can cover beyond its bounds: shadow always, bulges only while active.
pub fn pill_margin(globals: Globals, time: f32) -> f32 {
    let mut bulge = globals.pressure * 8.0;
    for pulse in &globals.ripples {
        let fade = 1.0 - ((time - pulse.start_time) * 1.2).saturate();
        bulge += presence(pulse.start_time) * fade * fade * 11.0;
    }
    SHADOW_REACH + bulge * 0.5
}

/// 1.0 when positive, else 0.0; core lowers `f32::from(bool)` through `u8`, which costs an extra conversion.
pub fn presence(value: f32) -> f32 {
    if value > 0.0 { 1.0 } else { 0.0 }
}

fn interaction(pixel: Vec2, globals: Globals, time: f32) -> (f32, f32, Vec2, f32) {
    let mut ripple = Vec2::ZERO;
    let mut ripple_flash = 0.0;
    // Rust-GPU cannot lower this slice iterator without a pointer-to-integer conversion.
    for index in 0..globals.ripples.len() {
        let pulse = globals.ripples[index];
        let progress = ((time - pulse.start_time) * 1.2).saturate();
        // Uniform across the draw, so expired slots skip all per-pixel distance work.
        if pulse.start_time > 0.0 && progress < 1.0 {
            let offset = pixel - pulse.origin;
            let distance = offset.length();
            let direction = offset.normalize_or_zero();
            let wave = (distance - progress * 600.0).abs().smoothstep(80.0, 0.0) * (1.0 - progress);
            ripple += direction * wave * (1.0 - progress) * 0.5;
            ripple_flash = (ripple_flash + wave * 0.5).min(1.0);
        }
    }

    let mouse_bulge = if globals.pressure > 0.0 {
        pixel.distance(globals.pointer).smoothstep(150.0, 0.0) * globals.pressure * 8.0
    } else {
        0.0
    };
    (mouse_bulge, if ripple == Vec2::ZERO { 0.0 } else { ripple.length() * 22.0 }, ripple, ripple_flash)
}

pub fn ripple_flash(pixel: Vec2, globals: Globals, time: f32) -> f32 {
    interaction(pixel, globals, time).3
}

pub fn ripple_light(color: Vec3, flash: f32) -> Vec3 {
    color.lerp(color * 1.5 + 0.1, flash)
}

/// Edge light derived from the final SDF, so it follows every deformation.
pub fn pill_sheen(distance: f32) -> f32 {
    distance.smoothstep(5.0, -3.0) * 0.14
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
    let edge_t = (point.dot(edge) / edge.length_squared()).saturate();
    let cross = point.y * edge.x - point.x * edge.y;
    (point - edge * edge_t).length() * if cross < 0.0 { -1.0 } else { 1.0 }
}

pub fn sd_rounded_triangle(point: Vec2, side_len: f32, radius: f32) -> f32 {
    let k = 1.732_050_8;
    let mut point = vec2(point.x.abs(), point.y);
    let h = (point.x + k * point.y).max(0.0);
    point -= 0.5 * vec2(h, h * k);
    point -= vec2(point.x.clamp(-0.5 * (side_len - radius) * k, 0.5 * (side_len - radius) * k), -0.5 * (side_len - radius));
    point.length() * if point.y > 0.0 { -1.0 } else { 1.0 } - radius
}

/// Shortest distance from `point` to the line segment between `start` and `end`.
pub fn segment_distance(point: Vec2, start: Vec2, end: Vec2) -> f32 {
    let segment = end - start;
    let along = ((point - start).dot(segment) / segment.length_squared().max(0.001)).saturate();
    (point - start - segment * along).length()
}

/// Antialiased coverage of the outline of a shape at the given signed `distance`, `width` pixels wide.
pub fn stroke(distance: f32, width: f32) -> f32 {
    distance.abs().smoothstep(width + ANTIALIAS_WIDTH, width - ANTIALIAS_WIDTH)
}

/// Antialiased coverage of the inside of a shape at the given signed `distance`.
pub fn fill(distance: f32) -> f32 {
    distance.smoothstep(ANTIALIAS_WIDTH, -ANTIALIAS_WIDTH)
}

pub fn fill_rounded_box(point: Vec2, half_size: Vec2, radius: f32) -> f32 {
    fill(sd_rounded_box(point, half_size, radius))
}

/// "‹" chevron with its tip at the origin, spanning to `extent` and its mirror; negate `extent.x` for a "›".
pub fn sd_chevron(point: Vec2, extent: Vec2) -> f32 {
    segment_distance(point, Vec2::ZERO, extent).min(segment_distance(point, Vec2::ZERO, vec2(extent.x, -extent.y)))
}

pub fn smooth_union(base: f32, shape: f32, smoothing: f32, amount: f32) -> f32 {
    let blend = (0.5 + 0.5 * (shape - base) / smoothing).saturate();
    let union = shape.lerp(base, blend) - smoothing * blend * (1.0 - blend);
    base.lerp(union, amount)
}
