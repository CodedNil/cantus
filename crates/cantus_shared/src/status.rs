use crate::{GAP, tempo::WeatherCondition};
#[cfg(feature = "cpu")]
use glam::FloatExt;

pub const STATUS_HISTORY_SAMPLES: usize = 40;
const STATUS_HISTORY_PACKS: usize = STATUS_HISTORY_SAMPLES / 4;
const STATUS_PADDING: f32 = crate::UNIT * 3.0;
const STATUS_WIDTHS: [f32; 6] = [60.0, 60.0, 32.0, 32.0, 24.0, 24.0];
const STATUS_CENTERS: [f32; 6] = [42.0, 110.0, 164.0, 164.0, 200.0, 232.0];
const BATTERY_SLOT: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "cpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct StatusPill {
    /// Left edge of the pill, in bar pixels.
    pub x: f32,
    /// Width of the pill, in pixels.
    pub width: f32,
    /// Battery charge, 0 to 1.
    pub battery_level: f32,
    /// Whether this machine has a battery.
    pub battery_present: f32,
    /// Whether the battery is currently charging.
    pub battery_charging: f32,
    /// System output volume, 0 to 1.
    pub volume: f32,
    /// Whether audio output is muted.
    pub muted: f32,
    /// RMS level sampled from the system audio monitor stream.
    pub audio_activity: f32,
    /// Global shader time at which the newest history samples arrived.
    pub sample_time: f32,
    /// CPU temperature and usage history.
    pub cpu: ProcessorStatus,
    /// GPU temperature and usage history.
    pub gpu: ProcessorStatus,
    /// 0 means idle, 1 means power off, and 2 means reboot.
    pub power_action: f32,
    /// How far the held-down confirmation for `power_action` has progressed, 0 to 1.
    pub power_progress: f32,
    /// Sky state copied from the weather pill.
    pub sun: [f32; 2],
    /// Current sky condition, for the pill's background.
    pub conditions: WeatherCondition,
}

impl StatusPill {
    pub const fn layout(&self) -> StatusLayout {
        StatusLayout::new(self.battery_present > 0.5)
    }
}

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
                STATUS_WIDTHS[BATTERY_SLOT as usize] + GAP
            } else {
                0.0
            }
    }

    pub const fn width(self) -> f32 {
        self.center(5) + STATUS_WIDTHS[5] * 0.5 + STATUS_PADDING
    }

    pub const fn bounds(self, first: u32, last: u32) -> (f32, f32) {
        (
            self.center(first) - (STATUS_WIDTHS[first as usize] + GAP) * 0.5,
            self.center(last) + (STATUS_WIDTHS[last as usize] + GAP) * 0.5,
        )
    }

    pub fn section(self, x: f32) -> u32 {
        (0..5)
            .find(|&slot| {
                (slot != BATTERY_SLOT || self.battery)
                    && x < self.center(slot) + (STATUS_WIDTHS[slot as usize] + GAP) * 0.5
            })
            .unwrap_or(5)
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
    pub temperature_history: UsageHistory,
}
