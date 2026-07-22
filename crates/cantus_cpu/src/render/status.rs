use crate::{AppUpdater, send_update};
use cantus_shared::{ProcessorStatus, StatusLayout, StatusPill};
use glam::{FloatExt, Vec2, vec2};
use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    thread,
    time::Duration,
};
use sysinfo::{Components, System};
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

#[derive(Clone, Copy, Default)]
struct GpuMetrics {
    usage: f32,
    memory: f32,
    temperature: f32,
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
        set_system_volume(self.volume);
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
    let mut cpu = ProcessorStatus::default();
    let mut gpu = ProcessorStatus::default();
    let mut system = System::new();
    let mut components = Components::new_with_refreshed_list();
    loop {
        system.refresh_cpu_usage();
        system.refresh_memory();
        components.refresh(false);
        let (battery, battery_charging) = battery();
        let gpu_metrics = nvidia_metrics()
            .or_else(|| drm_metrics(&components))
            .unwrap_or_default();
        let audio = system_volume();
        cpu.temperature =
            component_temperature(&components, &["k10temp", "coretemp", "zenpower", "cpu"]);
        cpu.usage.push(system.global_cpu_usage() / 100.0);
        cpu.memory
            .push(ratio(system.used_memory(), system.total_memory()));
        gpu.temperature = gpu_metrics.temperature;
        gpu.usage.push(gpu_metrics.usage);
        gpu.memory.push(gpu_metrics.memory);
        if !send_update(updater, move |app| {
            app.status.cpu = cpu;
            app.status.gpu = gpu;
            app.status.battery = battery;
            app.status.battery_charging = battery_charging;
            if let Some(audio) = audio {
                (app.status.volume, app.status.muted) = audio;
            }
            app.status.sample_time = app.render.start_time.elapsed().as_secs_f32();
        }) {
            break;
        }
        thread::sleep(SAMPLE_INTERVAL);
    }
}

fn ratio(used: u64, total: u64) -> f32 {
    used as f32 / total.max(1) as f32
}

fn number(path: impl AsRef<Path>) -> u64 {
    fs::read_to_string(path).map_or(0, |value| value.trim().parse().unwrap_or_default())
}

fn entry_with(root: &str, child: &str) -> Option<PathBuf> {
    fs::read_dir(root)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| path.join(child).exists())
}

fn component_temperature(components: &Components, names: &[&str]) -> f32 {
    components
        .iter()
        .find(|component| names.iter().any(|name| component.label().contains(name)))
        .and_then(sysinfo::Component::temperature)
        .unwrap_or_default()
}

fn battery() -> (Option<f32>, bool) {
    let Some(path) = entry_with("/sys/class/power_supply", "capacity") else {
        return (None, false);
    };
    let charging = fs::read_to_string(path.join("status"))
        .is_ok_and(|status| status.trim().eq_ignore_ascii_case("charging"));
    (Some(number(path.join("capacity")) as f32 / 100.0), charging)
}

fn nvidia_metrics() -> Option<GpuMetrics> {
    let output = command_output(
        "nvidia-smi",
        &[
            "--query-gpu=utilization.gpu,memory.used,memory.total,temperature.gpu",
            "--format=csv,noheader,nounits",
        ],
    )?;
    let mut values = output
        .lines()
        .next()?
        .split(',')
        .map(|value| value.trim().parse::<f32>());
    let mut next = || values.next()?.ok();
    let (usage, used, total, temperature) = (next()?, next()?, next()?, next()?);
    Some(GpuMetrics {
        usage: (usage / 100.0).saturate(),
        memory: (used / total.max(1.0)).saturate(),
        temperature,
    })
}

fn drm_metrics(components: &Components) -> Option<GpuMetrics> {
    let device = entry_with("/sys/class/drm", "device/gpu_busy_percent")?.join("device");
    Some(GpuMetrics {
        usage: number(device.join("gpu_busy_percent")) as f32 / 100.0,
        memory: ratio(
            number(device.join("mem_info_vram_used")),
            number(device.join("mem_info_vram_total")),
        ),
        temperature: component_temperature(components, &["amdgpu"]),
    })
}

fn set_system_volume(volume: f32) {
    let volume = format!("{:.3}", volume.saturate());
    thread::spawn(move || {
        if let Err(error) = Command::new("wpctl")
            .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &volume])
            .status()
        {
            warn!(%error, "Failed to set PipeWire volume");
        }
    });
}

fn system_volume() -> Option<(f32, bool)> {
    let text = command_output("wpctl", &["get-volume", "@DEFAULT_AUDIO_SINK@"])?;
    Some((
        text.split_whitespace().nth(1)?.parse().ok()?,
        text.contains("MUTED"),
    ))
}

fn command_output(command: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(command).args(args).output().ok()?;
    String::from_utf8(output.status.success().then_some(output.stdout)?).ok()
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
