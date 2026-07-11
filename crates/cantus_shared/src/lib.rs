#![no_std]

use glam::Vec2;

#[repr(C)]
#[derive(Copy, Clone, Default)]
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
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct PlayheadUniforms {
    pub bar_split: f32,
    pub icon_presence: f32,
    pub icon_morph: f32,
    pub icon_scale: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct BackgroundPill {
    pub x: f32,
    pub width: f32,
    pub colors: [u32; 4],
    pub alpha: f32,
    pub image_index: i32,
    pub rating: i32,
    pub primary_playlist_count: u32,
    pub secondary_playlist_count: u32,
    pub secondary_expansion: f32,
    pub playlist_images: [i32; MAX_PILL_PLAYLIST_ICONS],
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

/// Maximum number of glyph instances that can be drawn in a single frame.
pub const MAX_GLYPH_INSTANCES: usize = 2048;

#[repr(C)]
#[derive(Copy, Clone)]
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

pub const ICON_SPACING: f32 = 20.0;
pub const MAX_PILL_PLAYLIST_ICONS: usize = 8;
pub const ICON_HITBOX_HALF_SIZE: f32 = ICON_SPACING * 0.6;
pub const BACKPLATE_RADIUS: f32 = 10.0;
pub const BACKPLATE_HOVER_GROWTH: f32 = ICON_SPACING * 0.4;
pub const BACKPLATE_END_PADDING: f32 = ICON_SPACING * 0.15;
pub const BACKPLATE_Y_OFFSET: f32 = -ICON_HITBOX_HALF_SIZE * 0.5;

impl BackgroundPill {
    pub const fn star_count(&self) -> f32 {
        if self.rating >= 0 { 5.0 } else { 0.0 }
    }

    pub const fn primary_icon_count(&self) -> f32 {
        self.star_count() + self.primary_playlist_count as f32
    }

    pub fn icon_rows(&self, primary_center_y: f32) -> (PillIconRow, PillIconRow) {
        pill_icon_rows(
            self.x + self.width * 0.5,
            primary_center_y,
            self.primary_icon_count(),
            self.secondary_playlist_count as f32,
            self.secondary_expansion,
        )
    }
}

pub fn pill_icon_primary_center_y(bar_start_y: f32, bar_height: f32) -> f32 {
    bar_start_y + bar_height * 0.975
}

#[derive(Copy, Clone)]
pub struct PillIconRow {
    pub center: Vec2,
    pub count: f32,
    pub expansion: f32,
}

impl PillIconRow {
    pub fn padded_half_span(self) -> f32 {
        ((self.count - 1.0).max(0.0) * ICON_SPACING * self.expansion * 0.5) + BACKPLATE_END_PADDING
    }

    pub fn half_size(self, radius: f32) -> Vec2 {
        Vec2::new(self.padded_half_span() + radius, radius)
    }

    pub fn backplate_center(self) -> Vec2 {
        self.center + Vec2::new(0.0, BACKPLATE_Y_OFFSET)
    }

    pub fn icon_center(self, index: f32) -> Vec2 {
        let row_center = (self.count - 1.0).max(0.0) * 0.5;
        Vec2::new(
            self.center.x + (index - row_center) * ICON_SPACING * self.expansion,
            self.center.y,
        )
    }

    pub fn hit_icon(self, index: f32, point: Vec2) -> Option<Vec2> {
        let center = self.icon_center(index);
        let delta = (point - center).abs();
        (delta.x <= ICON_HITBOX_HALF_SIZE && delta.y <= ICON_HITBOX_HALF_SIZE).then_some(center)
    }
}

pub fn pill_icon_rows(
    center_x: f32,
    primary_center_y: f32,
    primary_count: f32,
    secondary_count: f32,
    secondary_expansion: f32,
) -> (PillIconRow, PillIconRow) {
    let row = |center_y, count, expansion| PillIconRow {
        center: Vec2::new(center_x, center_y),
        count,
        expansion,
    };
    (
        row(primary_center_y, primary_count, 1.0),
        row(
            primary_center_y + ICON_SPACING * secondary_expansion,
            secondary_count,
            secondary_expansion,
        ),
    )
}
