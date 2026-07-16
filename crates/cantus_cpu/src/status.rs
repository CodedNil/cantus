use crate::{AppUpdater, model::Rect, send_update};
use cantus_shared::StatusPill;
use std::path::{Path, PathBuf};
use std::{fs, process::Command, thread, time::Duration};
use tracing::warn;

pub const GAP: f32 = 6.0;
pub const WIDTH: f32 = 496.0;
const LABEL_LAYOUT: [(f32, f32); 4] = [(24.0, 55.0), (96.0, 42.0), (159.0, 58.0), (249.0, 48.0)];
const SENSORS: [&str; 2] = ["k10temp", "amdgpu"];

#[derive(Default)]
pub struct Status {
    labels: [String; 4],
    battery: Option<f32>,
    volume: [f32; 2],
}

impl Status {
    pub fn new(updater: AppUpdater) -> Self {
        thread::spawn(move || monitor(&updater));
        Self::default()
    }

    pub fn pill(&self, screen_width: f32) -> StatusPill {
        StatusPill {
            x: screen_width - WIDTH - GAP,
            width: WIDTH,
            battery: self.battery.map_or([0.0; 2], |level| [level, 1.0]),
            volume: self.volume,
        }
    }

    pub fn labels(&self, pill: StatusPill, mut draw: impl FnMut(&str, f32, f32)) {
        for (text, (offset, width)) in self.labels.iter().zip(LABEL_LAYOUT) {
            draw(text, pill.x + offset, width);
        }
        let controls_x = pill.x + pill.width - 190.0;
        if let Some(level) = self.battery {
            draw(&percent(level), controls_x + 32.0, 35.0);
        }
        let volume_x = controls_x + if self.battery.is_some() { 92.0 } else { 32.0 };
        draw(&percent(self.volume[0]), volume_x, 38.0);
    }

    pub fn run_power_action(position: glam::Vec2, pill: StatusPill) -> bool {
        let action = match ((position.x - (pill.x + pill.width - 72.0)) / 34.0).floor() as i32 {
            0 => "poweroff",
            1 => "reboot",
            _ => return false,
        };
        if let Err(error) = Command::new("systemctl").arg(action).spawn() {
            warn!(%error, %action, "Failed to run power action");
        }
        true
    }

    pub fn controls_rect(pill: StatusPill, height: f32) -> Rect {
        let right = pill.x + pill.width;
        Rect::new(right - 72.0, 6.0, right, 6.0 + height)
    }
}

fn monitor(updater: &AppUpdater) {
    let mut previous_cpu = [0; 2];
    let sensors = SENSORS.map(|name| sensor(name).unwrap_or_default());
    loop {
        let gpu = number("/sys/class/drm/renderD128/device/gpu_busy_percent") as f32 / 100.0;
        let vram = ratio(
            number("/sys/class/drm/renderD128/device/mem_info_vram_used"),
            number("/sys/class/drm/renderD128/device/mem_info_vram_total"),
        );
        let [cpu_temp, gpu_temp] = sensors.each_ref().map(|path| number(path) as f32 / 1000.0);
        let with_temp = |usage, temp| format!("{} {temp:.0}°", percent(usage));
        let metrics = Status {
            labels: [
                with_temp(cpu_usage(&mut previous_cpu), cpu_temp),
                percent(memory_usage()),
                with_temp(gpu, gpu_temp),
                percent(vram),
            ],
            battery: battery_level().filter(|level| *level < 0.995),
            volume: volume().unwrap_or_default(),
        };
        if !send_update(updater, move |app| app.status = metrics) {
            break;
        }
        thread::sleep(Duration::from_secs(2));
    }
}

fn ratio(used: u64, total: u64) -> f32 {
    used as f32 / total.max(1) as f32
}

fn percent(value: f32) -> String {
    format!("{:.0}%", value * 100.0)
}

fn cpu_usage(previous: &mut [u64; 2]) -> f32 {
    let stat = fs::read_to_string("/proc/stat").unwrap_or_default();
    let times = stat
        .lines()
        .next()
        .unwrap_or_default()
        .split_ascii_whitespace()
        .skip(1)
        .filter_map(|value| value.parse::<u64>().ok());
    let current: [u64; 2] = [times.clone().skip(3).take(2).sum(), times.sum()];
    let idle = current[0].saturating_sub(previous[0]);
    let total = current[1].saturating_sub(previous[1]);
    *previous = current;
    ratio(total.saturating_sub(idle), total)
}

fn memory_usage() -> f32 {
    let info = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut values = info
        .lines()
        .filter_map(|line| line.split_ascii_whitespace().nth(1)?.parse::<u64>().ok());
    let total = values.next().unwrap_or_default();
    let used = total.saturating_sub(values.nth(1).unwrap_or_default());
    ratio(used, total)
}

fn sensor(name: &str) -> Option<PathBuf> {
    fs::read_dir("/sys/class/hwmon")
        .ok()?
        .flatten()
        .find_map(|entry| {
            let path = entry.path();
            fs::read_to_string(path.join("name"))
                .ok()?
                .contains(name)
                .then(|| path.join("temp1_input"))
        })
}

fn number(path: impl AsRef<Path>) -> u64 {
    fs::read_to_string(path).map_or(0, |value| value.trim().parse().unwrap_or_default())
}

fn battery_level() -> Option<f32> {
    fs::read_dir("/sys/class/power_supply")
        .ok()?
        .flatten()
        .map(|entry| entry.path().join("capacity"))
        .find(|path| path.exists())
        .map(|path| number(path) as f32 / 100.0)
}

fn volume() -> Option<[f32; 2]> {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
        .ok()?;
    let text = String::from_utf8(output.stdout).ok()?;
    Some([
        text.split_whitespace().nth(1)?.parse().ok()?,
        f32::from(text.contains("MUTED")),
    ])
}
