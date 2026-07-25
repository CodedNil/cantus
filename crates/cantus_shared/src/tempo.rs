use crate::{GAP, UNIT, smoothstep};
use bitfields::bitfield;
use core::f32::consts::PI;
use glam::Vec2;

/// Number of conditions shown in the hourly forecast row.
pub const HOURLY_FORECASTS: usize = 6;
/// Hours between adjacent conditions in the hourly forecast row.
pub const HOURLY_STEP_HOURS: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct WeatherPill {
    /// Left edge of the pill, in bar pixels.
    pub x: f32,
    /// How open the calendar popup is, from 0 (closed) to 1 (fully expanded).
    pub calendar_expansion: f32,
    /// `[sunrise, sunset]`, as hour-of-day.
    pub sun_hours: [f32; 2],
    /// Hour-of-day of the first hourly forecast sample.
    pub hourly_start: f32,
    /// Conditions at four-hour intervals; the first sample is the current condition.
    pub hourly: [WeatherCondition; HOURLY_FORECASTS],
    /// Condition for each of the next 5 days.
    pub daily: [WeatherCondition; 5],
}

/// Cloud/rain/snow/lightning/hail conditions, packed into a `u32`.
#[bitfield(u32, new = false)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct WeatherCondition {
    #[bits(4)]
    pub fog_raw: u32,
    #[bits(7)]
    pub cloud_raw: u32,
    #[bits(8)]
    pub rain_raw: u32,
    #[bits(8)]
    pub snow_raw: u32,
    pub lightning: bool,
    #[bits(4)]
    pub hail_raw: u32,
}

impl WeatherCondition {
    const FOUR_BIT_MAX: f32 = 15.0;
    const SEVEN_BIT_MAX: f32 = 127.0;
    const EIGHT_BIT_MAX: f32 = 255.0;

    fn pack(value: f32, max: f32) -> u32 {
        (value.clamp(0.0, 1.0) * max) as u32
    }

    #[must_use]
    pub fn from_values(
        fog: f32,
        cloud: f32,
        rain: f32,
        snow: f32,
        lightning: bool,
        hail: f32,
    ) -> Self {
        let mut result = Self::from_bits(0);
        result.set_fog_raw(Self::pack(fog, Self::FOUR_BIT_MAX));
        result.set_cloud_raw(Self::pack(cloud, Self::SEVEN_BIT_MAX));
        result.set_rain_raw(Self::pack(rain, Self::EIGHT_BIT_MAX));
        result.set_snow_raw(Self::pack(snow, Self::EIGHT_BIT_MAX));
        result.set_lightning(lightning);
        result.set_hail_raw(Self::pack(hail, Self::FOUR_BIT_MAX));
        result
    }

    /// Unpacks `[fog, cloud, rain, snow, lightning, hail]` as 0-1 values.
    pub fn values(self) -> [f32; 6] {
        [
            self.fog_raw() as f32 / Self::FOUR_BIT_MAX,
            self.cloud_raw() as f32 / Self::SEVEN_BIT_MAX,
            self.rain_raw() as f32 / Self::EIGHT_BIT_MAX,
            self.snow_raw() as f32 / Self::EIGHT_BIT_MAX,
            if self.lightning() { 1.0 } else { 0.0 },
            self.hail_raw() as f32 / Self::FOUR_BIT_MAX,
        ]
    }
}

pub const WIDTH: f32 = UNIT * 77.0;
pub const FORECAST_X: f32 = WIDTH + GAP;
pub const TOP_GAP: f32 = GAP;
pub const EXTENSION: f32 = UNIT * 61.0;
const HEADER_BOTTOM: f32 = UNIT * 14.0;
const REVEAL_START: f32 = 0.5;
const REVEAL_SPREAD: f32 = 0.18;
const REVEAL_DURATION: f32 = 0.24;

pub fn expanded_x(x: f32, expansion: f32) -> f32 {
    x - FORECAST_X * expansion * 0.5
}

pub fn popup_size(expansion: f32) -> Vec2 {
    Vec2::new(WIDTH + FORECAST_X * expansion, EXTENSION * expansion)
}

pub fn forecast_center(height: f32, row: f32) -> f32 {
    HEADER_BOTTOM + height * 0.5 + row * (height + GAP)
}

pub fn forecast_row(height: f32, row: f32) -> (Vec2, Vec2) {
    let size = Vec2::new(WIDTH - GAP * 2.0, height);
    let center = Vec2::new(FORECAST_X + WIDTH * 0.5, forecast_center(height, row));
    (center - size * 0.5, size)
}

pub fn reveal_progress(expansion: f32, y: f32) -> f32 {
    let delay = REVEAL_START + (y / EXTENSION) * REVEAL_SPREAD;
    smoothstep(delay, delay + REVEAL_DURATION, expansion)
}

/// Approximates `sin(x)` for `x` in `[0, PI]`
fn sin_arch(x: f32) -> f32 {
    let y = PI - x;
    16.0 * x * y / (5.0 * PI * PI - 4.0 * x * y)
}

/// Sun phase (0 at sunrise, 1 at sunset) and height (-1 to 1) for the given hour.
pub fn sun_position(hour: f32, [sunrise, sunset]: [f32; 2]) -> [f32; 2] {
    let daylight = sunset - sunrise;
    if hour >= sunrise && hour <= sunset {
        let phase = (hour - sunrise) / daylight;
        [phase, sin_arch(phase * PI)]
    } else {
        let night = 24.0 - daylight;
        let phase = if hour < sunrise {
            (hour + 24.0 - sunset) / night
        } else {
            (hour - sunset) / night
        };
        let past_sunset = if hour >= sunset { 1.0 } else { 0.0 };
        [past_sunset, -sin_arch(phase * PI)]
    }
}
