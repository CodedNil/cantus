use crate::{AppUpdater, send_update};
use cantus_shared::{ProcessorStatus, StatusLayout, StatusPill};
use glam::{FloatExt, Vec2, vec2};
use std::{
    fs,
    io::{self, Read},
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    thread,
    time::Duration,
};
use sysinfo::{Components, Gpus, System};
use tracing::warn;

pub const GAP: f32 = 6.0;
pub const WIDTH: f32 = StatusLayout::new(true).width();
const VOLUME_STEP: f32 = 0.05;
const FULL_BATTERY_LEVEL: f32 = 0.995;
const SAMPLE_INTERVAL: Duration = Duration::from_millis(500);
const AUDIO_LEVEL_GAIN: f32 = 5.5;
const AUDIO_BUFFER_SIZE: usize = 8192;
const AUDIO_SLOT: u32 = 3;
const REBOOT_SLOT: u32 = 4;
const POWER_OFF_SLOT: u32 = 5;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    PowerOff,
    Reboot,
}

impl PowerAction {
    const fn shader_id(self) -> f32 {
        self as u32 as f32 + 1.0
    }
}

#[derive(Default)]
pub struct Status {
    cpu: ProcessorStatus,
    gpu: ProcessorStatus,
    battery: Option<f32>,
    battery_charging: bool,
    volume: f32,
    muted: bool,
    audio_level: Arc<AtomicU32>,
    sample_time: f32,
    /// Smoothed audio level and CPU/GPU temperatures, eased toward the live readings each frame.
    damped_audio: f32,
    damped_temperatures: [f32; 2],
}

impl Status {
    pub fn new(updater: AppUpdater) -> Self {
        let status = Self::default();
        let meter_level = Arc::clone(&status.audio_level);
        thread::spawn(move || monitor_playback(&meter_level));
        thread::spawn(move || monitor(&updater));
        status
    }

    /// Ease the displayed audio level and temperatures toward their live readings.
    pub fn damp_readings(&mut self, dt: f32) {
        let audio = f32::from_bits(self.audio_level.load(Ordering::Relaxed));
        let response = if audio > self.damped_audio { 12.0 } else { 7.0 };
        damp(&mut self.damped_audio, audio, response, dt);
        let targets = [self.cpu.temperature, self.gpu.temperature];
        for (temperature, target) in self.damped_temperatures.iter_mut().zip(targets) {
            if *temperature == 0.0 {
                *temperature = target;
            } else {
                damp(temperature, target, 4.0, dt);
            }
        }
    }

    pub fn adjust_volume(&mut self, direction: i32) {
        self.volume = (self.volume - direction as f32 * VOLUME_STEP).saturate();
        let volume = format!("{:.3}", self.volume);
        thread::spawn(move || {
            if let Err(error) = Command::new("wpctl")
                .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &volume])
                .status()
            {
                warn!(%error, "Failed to set PipeWire volume");
            }
        });
    }

    pub fn audio_at(position: Vec2, pill: &StatusPill, height: f32) -> bool {
        slot_hit(position, pill, height, AUDIO_SLOT, AUDIO_SLOT).is_some()
    }

    pub fn pill(
        &self,
        screen_width: f32,
        power_action: Option<PowerAction>,
        power_progress: f32,
    ) -> StatusPill {
        let battery = self.battery.filter(|level| *level < FULL_BATTERY_LEVEL);
        let width = StatusLayout::new(battery.is_some()).width();
        let [cpu_temperature, gpu_temperature] = self.damped_temperatures;
        StatusPill {
            x: screen_width - width - GAP,
            width,
            battery_level: battery.unwrap_or_default(),
            battery_present: f32::from(battery.is_some()),
            battery_charging: f32::from(self.battery_charging),
            volume: self.volume,
            muted: f32::from(self.muted),
            audio_activity: self.damped_audio,
            sample_time: self.sample_time,
            cpu: ProcessorStatus {
                temperature: cpu_temperature,
                ..self.cpu
            },
            gpu: ProcessorStatus {
                temperature: gpu_temperature,
                ..self.gpu
            },
            power_action: power_action.map_or(0.0, PowerAction::shader_id),
            power_progress,
            ..Default::default()
        }
    }

    pub fn power_action_at(position: Vec2, pill: &StatusPill, height: f32) -> Option<PowerAction> {
        let local = slot_hit(position, pill, height, REBOOT_SLOT, POWER_OFF_SLOT)?;
        let layout = pill.layout();
        Some(
            if local.x < (layout.center(REBOOT_SLOT) + layout.center(POWER_OFF_SLOT)) * 0.5 {
                PowerAction::Reboot
            } else {
                PowerAction::PowerOff
            },
        )
    }

    pub fn power_action_center(action: PowerAction, pill: &StatusPill, height: f32) -> Vec2 {
        let x = pill
            .layout()
            .center([POWER_OFF_SLOT, REBOOT_SLOT][action as usize]);
        vec2(pill.x + x, crate::PANEL_START + height * 0.5)
    }

    pub fn run_power_action(action: PowerAction) {
        let command = ["poweroff", "reboot"][action as usize];
        if let Err(error) = Command::new("systemctl").arg(command).spawn() {
            warn!(%error, %command, "Failed to run held power action");
        }
    }
}

fn damp(value: &mut f32, target: f32, response: f32, dt: f32) {
    *value += (target - *value) * (1.0 - (-response * dt).exp());
}

/// Position within a status slot, if it lies inside the slot's clickable bounds.
fn slot_hit(position: Vec2, pill: &StatusPill, height: f32, first: u32, last: u32) -> Option<Vec2> {
    let local = position - vec2(pill.x, crate::PANEL_START);
    let (start, end) = pill.layout().bounds(first, last);
    ((0.0..height).contains(&local.y) && (start..end).contains(&local.x)).then_some(local)
}

fn monitor(updater: &AppUpdater) {
    let (Ok(mut system), Ok(mut components)) =
        (System::new(), Components::new_with_refreshed_list())
    else {
        warn!("sysinfo unavailable; system status monitor disabled");
        return;
    };
    let mut cpu = ProcessorStatus::default();
    let mut gpu = ProcessorStatus::default();
    let mut gpus = Gpus::new_with_refreshed_list().ok();
    loop {
        system.refresh_cpu_usage();
        system.refresh_memory();
        components.refresh(false);
        if let Some(gpus) = &mut gpus {
            gpus.refresh(false);
        }
        let (battery, battery_charging) = fs::read_dir("/sys/class/power_supply")
            .ok()
            .and_then(|entries| {
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .find(|path| path.join("capacity").exists())
            })
            .map_or((None, false), |path| {
                let capacity: u64 = fs::read_to_string(path.join("capacity"))
                    .map_or(0, |value| value.trim().parse().unwrap_or_default());
                let charging = fs::read_to_string(path.join("status"))
                    .is_ok_and(|status| status.trim().eq_ignore_ascii_case("charging"));
                (Some(capacity as f32 / 100.0), charging)
            });

        let audio = Command::new("wpctl")
            .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
            .output()
            .ok()
            .and_then(|output| {
                String::from_utf8(output.status.success().then_some(output.stdout)?).ok()
            })
            .and_then(|text| {
                Some((
                    text.split_whitespace().nth(1)?.parse().ok()?,
                    text.contains("MUTED"),
                ))
            });

        cpu.temperature = components
            .iter()
            .find(|component| {
                ["k10temp", "coretemp", "zenpower", "cpu"]
                    .iter()
                    .any(|name| component.label().contains(name))
            })
            .and_then(sysinfo::Component::temperature)
            .unwrap_or_default();
        cpu.usage.push(system.global_cpu_usage() / 100.0);
        cpu.memory
            .push(ratio(system.used_memory(), system.total_memory()));

        if let Some(gpu_device) = gpus.as_ref().and_then(|gpus| gpus.list().first()) {
            gpu.temperature = gpu_device.temperature().unwrap_or_default();
            gpu.usage
                .push((gpu_device.usage().unwrap_or_default() / 100.0).saturate());
            gpu.memory.push(ratio(
                gpu_device.used_memory().unwrap_or_default(),
                gpu_device.total_memory().unwrap_or_default(),
            ));
        }

        if !send_update(updater, move |app| {
            let Some(status) = &mut app.status else {
                return;
            };
            status.cpu = cpu;
            status.gpu = gpu;
            status.battery = battery;
            status.battery_charging = battery_charging;
            if let Some(audio) = audio {
                (status.volume, status.muted) = audio;
            }
            status.sample_time = app.render.start_time.elapsed().as_secs_f32();
        }) {
            break;
        }
        thread::sleep(SAMPLE_INTERVAL);
    }
}

fn ratio(used: u64, total: u64) -> f32 {
    used as f32 / total.max(1) as f32
}

fn monitor_playback(level: &AtomicU32) {
    loop {
        if let Err(error) = capture_playback(level) {
            warn!(%error, "PipeWire playback meter stopped");
        }
        level.store(0.0f32.to_bits(), Ordering::Relaxed);
        thread::sleep(Duration::from_secs(1));
    }
}

fn capture_playback(level: &AtomicU32) -> io::Result<()> {
    let mut child = Command::new("pw-record")
        .args([
            "--properties",
            "stream.capture.sink=true",
            "--format",
            "f32",
            "--raw",
            "-",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let mut output = child.stdout.take().expect("piped PipeWire output");
    let mut bytes = [0; AUDIO_BUFFER_SIZE];
    loop {
        let count = output.read(&mut bytes)?;
        if count == 0 {
            break;
        }
        let aligned = count - count % size_of::<f32>();
        let samples: &[f32] = bytemuck::cast_slice(&bytes[..aligned]);
        let mean_square = samples
            .iter()
            .fold(0.0, |sum, sample| sum + f64::from(sample * sample))
            / samples.len().max(1) as f64;
        let rms = (mean_square.sqrt() as f32 * AUDIO_LEVEL_GAIN).saturate();
        level.store(rms.to_bits(), Ordering::Relaxed);
    }
    child.wait()?;
    Ok(())
}
