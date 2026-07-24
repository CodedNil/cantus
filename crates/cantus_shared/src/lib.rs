#![no_std]

use glam::{FloatExt, Vec2};

pub const STATUS_HISTORY_SAMPLES: usize = 40;
pub const RIPPLE_COUNT: usize = 4;
const STATUS_HISTORY_PACKS: usize = STATUS_HISTORY_SAMPLES / 4;
const STATUS_GAP: f32 = 8.0;
const STATUS_PADDING: f32 = 12.0;
const STATUS_WIDTHS: [f32; 6] = [60.0, 60.0, 32.0, 32.0, 24.0, 24.0];
const STATUS_CENTERS: [f32; 6] = [42.0, 110.0, 164.0, 164.0, 200.0, 232.0];
const BATTERY_SLOT: u32 = 2;

#[derive(Copy, Clone)]
pub struct StatusLayout {
    battery: bool,
}

impl StatusLayout {
    pub const fn new(battery: bool) -> Self {
        Self { battery }
    }

    pub const fn center(self, slot: u32) -> f32 {
        STATUS_CENTERS[slot as usize]
            + if self.battery && slot > BATTERY_SLOT {
                STATUS_WIDTHS[BATTERY_SLOT as usize] + STATUS_GAP
            } else {
                0.0
            }
    }

    pub const fn width(self) -> f32 {
        self.center(5) + STATUS_WIDTHS[5] * 0.5 + STATUS_PADDING
    }

    pub const fn bounds(self, first: u32, last: u32) -> (f32, f32) {
        (
            self.center(first) - (STATUS_WIDTHS[first as usize] + STATUS_GAP) * 0.5,
            self.center(last) + (STATUS_WIDTHS[last as usize] + STATUS_GAP) * 0.5,
        )
    }

    pub fn section(self, x: f32) -> u32 {
        (0..5)
            .find(|&slot| {
                (slot != BATTERY_SLOT || self.battery)
                    && x < self.center(slot) + (STATUS_WIDTHS[slot as usize] + STATUS_GAP) * 0.5
            })
            .unwrap_or(5)
    }
}

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
    _padding: f32,
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
pub struct UsageHistory {
    samples: [u32; STATUS_HISTORY_PACKS],
}

impl UsageHistory {
    pub const fn get(&self, index: usize) -> f32 {
        let shift = ((index & 3) * 8) as u32;
        ((self.samples[index / 4] >> shift) & 0xff) as f32 / 255.0
    }

    #[cfg(feature = "cpu")]
    pub fn push(&mut self, value: f32) {
        for index in 0..STATUS_HISTORY_PACKS {
            let carry = self.samples.get(index + 1).map_or(0, |next| next & 0xff);
            self.samples[index] = self.samples[index] >> 8 | carry << 24;
        }
        self.samples[STATUS_HISTORY_PACKS - 1] |= ((value.saturate() * 255.0 + 0.5) as u32) << 24;
    }
}

/// Range mapped to the 0.0..=1.0 fraction stored in `ProcessorStatus::temperature_history`.
pub const MIN_TEMPERATURE: f32 = 30.0;
pub const MAX_TEMPERATURE: f32 = 100.0;

pub fn normalize_temperature(celsius: f32) -> f32 {
    ((celsius - MIN_TEMPERATURE) / (MAX_TEMPERATURE - MIN_TEMPERATURE)).saturate()
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct ProcessorStatus {
    pub temperature: f32,
    pub usage: UsageHistory,
    pub memory: UsageHistory,
    pub temperature_history: UsageHistory,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct StatusPill {
    pub x: f32,
    pub width: f32,
    pub battery_level: f32,
    pub battery_present: f32,
    pub battery_charging: f32,
    pub volume: f32,
    pub muted: f32,
    /// RMS level sampled from the system audio monitor stream.
    pub audio_activity: f32,
    /// Global shader time at which the newest history samples arrived.
    pub sample_time: f32,
    pub cpu: ProcessorStatus,
    pub gpu: ProcessorStatus,
    /// 0 means idle, 1 means power off, and 2 means reboot.
    pub power_action: f32,
    pub power_progress: f32,
    /// Sky state copied from the weather pill.
    pub sun: [f32; 2],
    pub conditions: [WeatherCondition; 3],
}

impl StatusPill {
    pub const fn layout(&self) -> StatusLayout {
        StatusLayout::new(self.battery_present > 0.5)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct WeatherPill {
    pub x: f32,
    pub width: f32,
    pub sun: [f32; 2],
    pub sun_hours: [f32; 2],
    pub hourly_times: [f32; 6],
    pub today: Vec2,
    pub calendar_expansion: f32,
    pub conditions: [WeatherCondition; 3],
    pub hourly: [WeatherCondition; 6],
    pub daily: [WeatherCondition; 5],
    pub padding: f32,
}

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

/// Geometry shared by weather drawing, text placement, and hit testing.
pub struct WeatherLayout;

impl WeatherLayout {
    pub const WIDTH: f32 = 310.0;
    const COLUMN_GAP: f32 = 10.0;
    pub const FORECAST_X: f32 = Self::WIDTH + Self::COLUMN_GAP;
    pub const MONTH_SLIDE: f32 = 24.0;
    pub const EXTENSION: f32 = 239.0;
    pub const TITLE: Vec2 = Vec2::new(Self::WIDTH * 0.5, 46.0);
    pub const TITLE_HALF_SIZE: Vec2 = Vec2::new(62.0, 18.0);
    pub const DETAILS: Vec2 = Vec2::new(Self::FORECAST_X + Self::WIDTH * 0.5, Self::TITLE.y);
    pub const ARROW_RADIUS: f32 = 20.0;
    const FORECAST_INSET: f32 = 8.0;
    /// Gap between the pill's bottom edge and the top of the popup background.
    pub const TOP_GAP: f32 = 7.0;

    pub fn expanded_x(x: f32, expansion: f32) -> f32 {
        x - Self::FORECAST_X * expansion * 0.5
    }

    pub fn popup_size(expansion: f32) -> Vec2 {
        Vec2::new(
            Self::WIDTH + Self::FORECAST_X * expansion,
            Self::EXTENSION * expansion,
        )
    }

    pub fn pill_center(height: f32) -> Vec2 {
        Vec2::new((Self::WIDTH + Self::FORECAST_X) * 0.5, -height * 0.5)
    }

    pub fn cell(index: usize) -> Vec2 {
        Vec2::new(
            (index % 7) as f32 * Self::WIDTH / 7.0 + Self::WIDTH / 14.0,
            104.0 + (index / 7) as f32 * 23.0,
        )
    }

    pub fn weekday(index: usize) -> Vec2 {
        Vec2::new(Self::cell(index).x, 77.0)
    }

    pub fn forecast_center(height: f32, row: f32) -> f32 {
        64.0 + height * 0.5 + row * (height + Self::COLUMN_GAP)
    }

    pub fn forecast_center_at(height: f32, row: f32, reveal: f32) -> Vec2 {
        Self::pill_center(height).lerp(
            Vec2::new(Self::DETAILS.x, Self::forecast_center(height, row)),
            reveal,
        )
    }

    pub fn forecast_item(height: f32, row: f32, column: usize, count: usize, line: usize) -> Vec2 {
        let line = line as f32 * 2.0 - 1.0;
        Vec2::new(
            Self::FORECAST_X + (column as f32 + 0.5) * Self::WIDTH / count as f32,
            Self::forecast_center(height, row) + line * 9.0,
        )
    }

    pub fn forecast_row(height: f32, row: f32, reveal: f32) -> (Vec2, Vec2) {
        let size = Vec2::new(Self::WIDTH - Self::FORECAST_INSET * 2.0, height);
        (
            Self::forecast_center_at(height, row, reveal) - size * 0.5,
            size,
        )
    }

    pub fn forecast_reveal(expansion: f32, row: f32) -> f32 {
        smoothstep(0.48 + row * 0.08, 0.76 + row * 0.08, expansion)
    }

    pub fn header_reveal(expansion: f32) -> f32 {
        smoothstep(0.5, 0.8, expansion)
    }

    pub fn arrow(side: f32, reveal: f32) -> Vec2 {
        Vec2::new(
            Self::WIDTH * 0.5 + side * (Self::WIDTH * 0.5 - 28.0) * reveal,
            Self::TITLE.y - (1.0 - reveal) * 14.0,
        )
    }
}

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
        (self.count - 1.0).max(0.0) * ICON_SPACING * self.expansion * 0.5
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
    let t = ((x - edge0) / (edge1 - edge0)).saturate();
    t * t * (3.0 - 2.0 * t)
}

pub fn approach(current: &mut f32, target: f32, speed: f32) {
    *current += (target - *current).clamp(-speed, speed);
}
