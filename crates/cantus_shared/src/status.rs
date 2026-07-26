use crate::tempo::WeatherConditions;
use crate::{GAP, PADDING};
#[cfg(feature = "cpu")]
use glam::FloatExt;

pub const STATUS_HISTORY_SAMPLES: usize = 40;
pub const AUDIO_SPECTRUM_BANDS: usize = 7;
const STATUS_HISTORY_PACKS: usize = STATUS_HISTORY_SAMPLES / 4;
const PROCESSOR_WIDTH: f32 = 60.0;
const DATA_WIDTH: f32 = 32.0;
const ACTION_WIDTH: f32 = 24.0;
pub const CPU_SECTION: u32 = 0;
pub const GPU_SECTION: u32 = 1;
pub const BATTERY_SECTION: u32 = 2;
pub const AUDIO_SECTION: u32 = 3;
pub const REBOOT_SECTION: u32 = 4;
pub const POWER_SECTION: u32 = 5;
pub const WIDTH: f32 = PADDING * 2.0 + (PROCESSOR_WIDTH + DATA_WIDTH + ACTION_WIDTH) * 2.0 + GAP * 5.0;

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct StatusPill {
    /// Battery charge from 0 to 1, or negative when no battery is present.
    pub battery_level: f32,
    /// Whether the battery is currently charging.
    pub battery_charging: f32,
    /// Signed volume: magnitude is the level, negative means muted.
    pub volume: f32,
    /// Logarithmic frequency-band levels sampled from the system audio monitor stream.
    pub audio_spectrum: [f32; AUDIO_SPECTRUM_BANDS],
    /// Fractional scroll between the two newest history samples.
    pub history_scroll: f32,
    /// CPU temperature and usage history.
    pub cpu: ProcessorStatus,
    /// GPU temperature and usage history.
    pub gpu: ProcessorStatus,
    /// Action ID plus hold progress; zero means no action is active.
    pub power_state: f32,
    /// Hovered power action using the same IDs as `power_action`.
    pub power_hover: f32,
    /// Current sun height and decoded sky condition.
    pub sun_height: f32,
    pub conditions: WeatherConditions,
}

impl StatusPill {
    pub const fn section_center(&self, section: u32) -> f32 {
        let mut x = PADDING;
        let mut index = 0;
        while index < section {
            if self.battery_level >= 0.0 || index != BATTERY_SECTION {
                x += self.section_width(index) + GAP;
            }
            index += 1;
        }
        x + self.section_width(section) * 0.5
    }

    pub const fn section_width(&self, section: u32) -> f32 {
        let width = if section < BATTERY_SECTION {
            PROCESSOR_WIDTH
        } else if section < REBOOT_SECTION {
            DATA_WIDTH
        } else {
            ACTION_WIDTH
        };
        if self.battery_level < 0.0 && section < BATTERY_SECTION {
            width + (DATA_WIDTH + GAP) * 0.5
        } else {
            width
        }
    }

    pub fn section_at(&self, x: f32) -> u32 {
        (0..5)
            .find(|&section| {
                (section != BATTERY_SECTION || self.battery_level >= 0.0)
                    && x < self.section_center(section) + (self.section_width(section) + GAP) * 0.5
            })
            .unwrap_or(POWER_SECTION)
    }
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

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct ProcessorStatus {
    pub temperature: f32,
    pub usage: UsageHistory,
    pub memory: UsageHistory,
}
