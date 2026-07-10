#![no_std]

use glam::Vec2;

/// Center-to-center spacing for playlist and rating icons, in logical pixels.
pub const ICON_SPACING: f32 = 20.0;

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
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct PlayheadUniforms {
    pub bar_split: f32,
    pub icon_presence: f32,
    pub icon_morph: f32,
    pub icon_scale: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct BackgroundPill {
    pub rect: Vec2,
    pub primary_icon_count: f32,
    pub secondary_icon_count: f32,
    pub secondary_expansion: f32,
    pub colors: [u32; 4],
    pub alpha: f32,
    pub image_index: i32,
    _padding: u32,
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

/// Maximum number of glyph instances that can be drawn in a single frame.
pub const MAX_GLYPH_INSTANCES: usize = 2048;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct GlyphInstance {
    /// Bottom-left corner of the glyph quad in logical pixels.
    pub pos: Vec2,
    /// Width and height of the glyph quad in logical pixels.
    pub size: Vec2,
    /// Top-left UV coordinate (normalized 0..1) into the glyph atlas.
    pub atlas_min: Vec2,
    /// Bottom-right UV coordinate (normalized 0..1) into the glyph atlas.
    pub atlas_max: Vec2,
    /// Right clip edge in logical pixels.
    pub clip_right: f32,
    /// Packed RGBA colour.
    pub color: u32,
}
