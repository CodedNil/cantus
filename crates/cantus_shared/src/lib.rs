#![no_std]

use glam::{FloatExt, Vec2};

pub mod status;
pub mod tempo;
pub mod track;

/// Base spacing unit. Sizes and gaps should be whole multiples of it.
pub const UNIT: f32 = 4.0;
/// The standard small gap between adjacent elements.
pub const GAP: f32 = UNIT * 2.0;

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).saturate();
    t * t * (3.0 - 2.0 * t)
}

pub fn approach(current: &mut f32, target: f32, speed: f32) {
    *current += (target - *current).clamp(-speed, speed);
}

pub const RIPPLE_COUNT: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct RipplePulse {
    pub origin: Vec2,
    /// Start time and visual strength. A zero strength marks an unused slot.
    pub animation: Vec2,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct GlobalUniforms {
    pub screen_size: Vec2,
    pub bar_height: Vec2,
    pub mouse_pos: Vec2,
    pub mouse_pressure: f32,
    pub playhead_x: f32,
    pub time: f32,
    /// Current hour in the configured weather location's local time.
    pub weather_hour: f32,
    pub ripples: [RipplePulse; RIPPLE_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct PlayheadUniforms {
    pub bar_split: f32,
    pub icon_presence: f32,
    pub icon_morph: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Particle {
    pub spawn_pos: Vec2,
    pub spawn_vel: Vec2,
    pub end_time: f32,
    pub color: u32,
}

pub const MAX_GLYPH_INSTANCES: usize = 2048;
pub const GLYPH_ATLAS_SIZE: u32 = 2048;

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct GlyphInstance {
    /// Bottom-left corner of the glyph quad in logical pixels.
    pub pos: Vec2,
    /// Width and height of the glyph quad in logical pixels.
    pub size: Vec2,
    /// Packed top-left and bottom-right atlas coordinates.
    pub atlas: [u32; 2],
    /// Right clip edge in logical pixels.
    pub clip_right: f32,
    pub alpha: f32,
}

pub const fn pack_u16x2(value: [u32; 2]) -> u32 {
    value[0] | value[1] << 16
}

pub const fn unpack_u16x2(value: u32) -> Vec2 {
    Vec2::new((value & 0xffff) as f32, (value >> 16) as f32)
}
