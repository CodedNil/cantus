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
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct TrackPill {
    pub x: f32,
    pub width: f32,
    pub colors: [u32; 4],
    pub visibility: f32,
    pub image_index: i32,
    pub rating: i32,
    pub primary_playlist_count: u32,
    pub secondary_playlist_count: u32,
    pub primary_alpha: f32,
    pub secondary_expansion: f32,
    pub audio_features: AudioFeatures,
    pub playlist_images: [i32; MAX_PILL_PLAYLIST_ICONS],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct StatusPill {
    pub x: f32,
    pub width: f32,
    pub battery: [f32; 2],
    pub volume: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct WeatherPill {
    pub x: f32,
    pub width: f32,
    pub sun: [f32; 2],
    pub today: Vec2,
    pub calendar_expansion: f32,
    pub conditions: [WeatherCondition; 3],
    /// Pads the storage-buffer array stride to the eight-byte alignment required by `today`.
    pub padding: f32,
}

const _: () = assert!(size_of::<WeatherPill>().is_multiple_of(8));

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct WeatherCondition {
    pub cloud: f32,
    pub fog: f32,
    pub lightning: f32,
    pub rain: f32,
    pub snow: f32,
    pub hail: f32,
}

impl WeatherCondition {
    #[must_use]
    pub fn lerp(self, other: Self, amount: f32) -> Self {
        let lerp = |from, to| from + (to - from) * amount;
        Self {
            cloud: lerp(self.cloud, other.cloud),
            fog: lerp(self.fog, other.fog),
            lightning: lerp(self.lightning, other.lightning),
            rain: lerp(self.rain, other.rain),
            snow: lerp(self.snow, other.snow),
            hail: lerp(self.hail, other.hail),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(
    feature = "cpu",
    derive(bytemuck::Pod, bytemuck::Zeroable, serde::Deserialize)
)]
pub struct AudioFeatures {
    pub energy: f32,
    pub danceability: f32,
    pub acousticness: f32,
    pub tempo: f32,
    pub valence: f32,
    pub instrumentalness: f32,
    pub loudness: f32,
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
    /// Packed top-left and bottom-right atlas coordinates.
    pub atlas: [u32; 2],
    /// Right clip edge in logical pixels.
    pub clip_right: f32,
    pub alpha: f32,
}

pub const GLYPH_ATLAS_SIZE: u32 = 2048;

pub const fn pack_u16x2(value: [u32; 2]) -> u32 {
    value[0] | value[1] << 16
}

pub const fn unpack_u16x2(value: u32) -> Vec2 {
    Vec2::new((value & 0xffff) as f32, (value >> 16) as f32)
}

/// Maximum number of playlist artwork icons carried by one pill instance.
pub const MAX_PILL_PLAYLIST_ICONS: usize = 8;

/// Visual width, in pixels, of rating and playlist icons before hover growth.
pub const ICON_WIDTH: f32 = 21.6;

/// Center-to-center icon spacing for rating stars and playlist artwork.
pub const ICON_SPACING: f32 = 18.0;

/// Total distance added below the weather pill while the calendar is open.
pub const WEATHER_CALENDAR_EXTENSION: f32 = 246.0;
/// Calendar title and arrow center, relative to the submenu's top edge.
pub const WEATHER_CALENDAR_TITLE_Y: f32 = 38.0;
/// Horizontal inset of each calendar arrow's center.
pub const WEATHER_CALENDAR_ARROW_X: f32 = 28.0;
/// Visual and clickable radius of the calendar arrow buttons.
pub const WEATHER_CALENDAR_ARROW_RADIUS: f32 = 20.0;

impl TrackPill {
    pub const fn star_count(&self) -> f32 {
        if self.rating >= 0 { 5.0 } else { 0.0 }
    }

    pub fn icon_rows(&self, bar_start_y: f32, bar_height: f32) -> (PillIconRow, PillIconRow) {
        pill_icon_rows(
            self.x + self.width * 0.5,
            pill_icon_primary_center_y(bar_start_y, bar_height),
            self.star_count() + self.primary_playlist_count as f32,
            self.secondary_playlist_count as f32,
            self.secondary_expansion,
        )
    }
}

pub fn pill_icon_primary_center_y(bar_start_y: f32, bar_height: f32) -> f32 {
    bar_start_y + bar_height * 0.975 - 3.0
}

#[derive(Copy, Clone)]
pub struct PillIconRow {
    pub center: Vec2,
    pub count: f32,
    pub expansion: f32,
}

impl PillIconRow {
    pub fn hit(self, point: Vec2) -> Option<(usize, bool)> {
        if self.expansion <= 0.0 {
            return None;
        }
        let index = (point.x - self.center.x) / (ICON_SPACING * self.expansion)
            + (self.count - 1.0).max(0.0) * 0.5
            + 0.5;
        if !(0.0..self.count).contains(&index) {
            return None;
        }
        let index = index as usize;
        let center = self.icon_center(index as f32);
        let delta = (point - center).abs();
        (delta.x <= ICON_WIDTH * 0.5 && delta.y <= ICON_WIDTH * 0.5)
            .then_some((index, point.x >= center.x))
    }

    pub fn half_span(self) -> f32 {
        let icon_span = (self.count - 1.0).max(0.0) * ICON_SPACING * self.expansion;
        icon_span * 0.5
    }

    pub fn half_size(self, radius: f32) -> Vec2 {
        Vec2::new(self.half_span() + radius, radius)
    }

    pub fn backplate_center(self) -> Vec2 {
        self.center + Vec2::new(0.0, -ICON_WIDTH * 0.25)
    }

    pub fn icon_center(self, index: f32) -> Vec2 {
        let row_center = (self.count - 1.0).max(0.0) * 0.5;
        Vec2::new(
            self.center.x + (index - row_center) * ICON_SPACING * self.expansion,
            self.center.y + 2.0,
        )
    }
}

pub fn pill_icon_rows(
    center_x: f32,
    primary_center_y: f32,
    primary_count: f32,
    secondary_count: f32,
    secondary_expansion: f32,
) -> (PillIconRow, PillIconRow) {
    (
        PillIconRow {
            center: Vec2::new(center_x, primary_center_y),
            count: primary_count,
            expansion: 1.0,
        },
        PillIconRow {
            center: Vec2::new(
                center_x,
                primary_center_y + ICON_SPACING * secondary_expansion,
            ),
            count: secondary_count,
            expansion: secondary_expansion,
        },
    )
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn approach(current: &mut f32, target: f32, speed: f32) {
    *current += (target - *current).clamp(-speed, speed);
}
