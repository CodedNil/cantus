#![no_std]

#[cfg(not(target_arch = "spirv"))]
use glam::{Vec2, Vec4};
#[cfg(target_arch = "spirv")]
use spirv_std::glam::{Vec2, Vec4};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct GlobalUniforms {
    pub screen_size: Vec2,
    pub bar_height: Vec2,
    pub mouse_pos: Vec2,
    pub mouse_pressure: f32,
    pub playhead_x: f32,
    pub expansion_xy: Vec2,
    pub expansion_time: f32,
    pub time: f32,
    pub scale_factor: f32,
    _padding0: f32,
    _padding1: f32,
    _padding2: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct PlayheadUniforms {
    pub volume: f32,
    pub bar_lerp: f32,
    pub play_lerp: f32,
    pub pause_lerp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct BackgroundPill {
    /// Encoded primary span, secondary span/expansion, and packed CPU key/primary fade.
    pub icon_span: Vec4,
    pub rect: Vec2,
    pub color0: u32,
    pub color1: u32,
    pub color2: u32,
    pub color3: u32,
    pub alpha: f32,
    pub image_index: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Particle {
    pub spawn_pos: Vec2,
    pub spawn_vel: Vec2,
    pub end_time: f32,
    pub color: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct IconInstance {
    pub pos: Vec2,
    pub data: u32,
    pub image_index: i32,
}
