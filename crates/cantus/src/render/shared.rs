use isthmus::glam::{FloatExt, Vec2};

/// Base spacing unit. Sizes and gaps should be whole multiples of it.
pub const UNIT: f32 = 4.0;
/// The standard small gap between adjacent elements.
pub const GAP: f32 = UNIT * 2.0;
/// The standard inset between a container edge and its contents.
pub const PADDING: f32 = UNIT * 3.0;

#[isthmus::data]
#[derive(Default)]
pub struct RipplePulse {
    pub origin: Vec2,
    pub start_time: f32,
    pub strength: f32,
}

#[isthmus::data]
#[derive(Default)]
pub struct FrameData {
    pub screen_size: Vec2,
    pub panel_top: f32,
    pub panel_height: f32,
    pub mouse_pos: Vec2,
    pub mouse_pressure: f32,
    pub playhead_x: f32,
    pub px_per_ms: f32,
    /// Space reserved for the status pill this frame (its current width plus a gap), or 0 if hidden.
    pub status_width: f32,
    pub time: f32,
    /// Current hour in the configured weather location's local time.
    pub weather_hour: f32,
    pub ripples: [RipplePulse; 4],
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).saturate();
    t * t * (3.0 - 2.0 * t)
}
