use crate::{AppUpdater, send_update};
use cantus_shared::{ProcessorStatus, StatusLayout, StatusPill};
use glam::{Vec2, vec2};
use std::{
    fs,
    io::{self, Read},
    path::Path,
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

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    PowerOff,
    Reboot,
}

impl PowerAction {
    pub const fn shader_id(self) -> f32 {
        self as u32 as f32 + 1.0
    }

    const fn slot(self) -> u32 {
        5 - self as u32
    }

    const fn command(self) -> &'static str {
        ["poweroff", "reboot"][self as usize]
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

    pub fn render_targets(&self) -> (f32, [f32; 2]) {
        (
            f32::from_bits(self.audio_level.load(Ordering::Relaxed)),
            [self.cpu.temperature, self.gpu.temperature],
        )
    }

    pub fn adjust_volume(&mut self, direction: i32) {
        let delta = if direction < 0 { 0.05 } else { -0.05 };
        self.volume = (self.volume + delta).clamp(0.0, 1.0);
        set_system_volume(self.volume);
    }

    pub fn audio_at(position: Vec2, pill: &StatusPill, height: f32) -> bool {
        let local = position - vec2(pill.x, crate::PANEL_START);
        let (start, end) = StatusLayout::new(pill.battery_present > 0.5).bounds(3, 3);
        (0.0..height).contains(&local.y) && (start..end).contains(&local.x)
    }

    pub fn pill(
        &self,
        screen_width: f32,
        power_action: Option<PowerAction>,
        power_progress: f32,
        audio_level: f32,
        cpu_temperature: f32,
        gpu_temperature: f32,
    ) -> StatusPill {
        let battery = self.battery.filter(|level| *level < 0.995);
        let width = StatusLayout::new(battery.is_some()).width();
        StatusPill {
            x: screen_width - width - GAP,
            width,
            battery_level: battery.unwrap_or_default(),
            battery_present: f32::from(battery.is_some()),
            battery_charging: f32::from(self.battery_charging),
            volume: self.volume,
            muted: f32::from(self.muted),
            audio_activity: audio_level,
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
            sun: [0.0; 2],
            conditions: Default::default(),
        }
    }

    pub fn power_action_at(position: Vec2, pill: &StatusPill, height: f32) -> Option<PowerAction> {
        let local = position - vec2(pill.x, crate::PANEL_START);
        let layout = StatusLayout::new(pill.battery_present > 0.5);
        let (start, end) = layout.bounds(4, 5);
        if !(0.0..height).contains(&local.y) || !(start..end).contains(&local.x) {
            return None;
        }
        if local.x < (layout.center(4) + layout.center(5)) * 0.5 {
            Some(PowerAction::Reboot)
        } else {
            Some(PowerAction::PowerOff)
        }
    }

    pub fn power_action_center(action: PowerAction, pill: &StatusPill, height: f32) -> Vec2 {
        let x = StatusLayout::new(pill.battery_present > 0.5).center(action.slot());
        vec2(pill.x + x, crate::PANEL_START + height * 0.5)
    }

    pub fn run_power_action(action: PowerAction) {
        let command = action.command();
        if let Err(error) = Command::new("systemctl").arg(command).spawn() {
            warn!(%error, %command, "Failed to run held power action");
        }
    }
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
            if let Some((volume, muted)) = audio {
                app.status.volume = volume;
                app.status.muted = muted;
            }
            app.status.sample_time = app.render.start_time.elapsed().as_secs_f32();
        }) {
            break;
        }
        thread::sleep(Duration::from_millis(500));
    }
}

fn ratio(used: u64, total: u64) -> f32 {
    used as f32 / total.max(1) as f32
}

fn number(path: impl AsRef<Path>) -> u64 {
    fs::read_to_string(path).map_or(0, |value| value.trim().parse().unwrap_or_default())
}

fn component_temperature(components: &Components, names: &[&str]) -> f32 {
    components
        .iter()
        .find(|component| names.iter().any(|name| component.label().contains(name)))
        .and_then(sysinfo::Component::temperature)
        .unwrap_or_default()
}

fn battery() -> (Option<f32>, bool) {
    let Some(path) = fs::read_dir("/sys/class/power_supply")
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .find(|path| path.join("capacity").exists())
    else {
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
    let mut values = output.lines().next()?.split(',').map(str::trim);
    let usage = values.next()?.parse::<f32>().ok()? / 100.0;
    let used = values.next()?.parse::<f32>().ok()?;
    let total = values.next()?.parse::<f32>().ok()?;
    let temperature = values.next()?.parse().ok()?;
    Some(GpuMetrics {
        usage: usage.clamp(0.0, 1.0),
        memory: (used / total.max(1.0)).clamp(0.0, 1.0),
        temperature,
    })
}

fn drm_metrics(components: &Components) -> Option<GpuMetrics> {
    let device = fs::read_dir("/sys/class/drm")
        .ok()?
        .flatten()
        .map(|entry| entry.path().join("device"))
        .find(|path| path.join("gpu_busy_percent").exists())?;
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
    let volume = format!("{:.3}", volume.clamp(0.0, 1.0));
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
    output.status.success().then_some(())?;
    String::from_utf8(output.stdout).ok()
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
    let mut bytes = [0; 8192];
    loop {
        let count = output.read(&mut bytes)?;
        if count == 0 {
            break;
        }
        let aligned = count - count % size_of::<f32>();
        let samples: &[f32] = bytemuck::cast_slice(&bytes[..aligned]);
        let energy = samples
            .iter()
            .fold(0.0, |sum, sample| sum + f64::from(sample * sample));
        if !samples.is_empty() {
            let rms = ((energy / samples.len() as f64).sqrt() as f32 * 5.5).clamp(0.0, 1.0);
            level.store(rms.to_bits(), Ordering::Relaxed);
        }
    }
    child.wait()?;
    Ok(())
}
