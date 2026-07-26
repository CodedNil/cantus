use crate::{
    AppUpdater,
    interaction::{InteractionState, Rect},
    send_update,
};
use cantus_gpu::{
    GAP,
    status::{AUDIO_SPECTRUM_BANDS, Data, ProcessorStatus, StatusSection, WIDTH, pill_x},
};
use glam::{FloatExt, vec2};
use std::{
    array,
    f32::consts::TAU,
    fs,
    io::{self, Read},
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    thread,
    time::{Duration, Instant},
};
use sysinfo::{Components, Gpus, System};
use tracing::warn;

const VOLUME_STEP: f32 = 0.05;
const FULL_BATTERY_LEVEL: f32 = 0.995;
const SAMPLE_INTERVAL: Duration = Duration::from_millis(500);
const AUDIO_BUFFER_SIZE: usize = 8192;
const AUDIO_SAMPLE_RATE: f32 = 48_000.0;
const AUDIO_WINDOW_SIZE: usize = 1024;
const AUDIO_BAND_EDGES: [f32; AUDIO_SPECTRUM_BANDS + 1] =
    [60.0, 120.0, 250.0, 500.0, 1_000.0, 2_000.0, 4_000.0, 12_000.0];
const POWER_HOLD_DURATION: Duration = Duration::from_millis(1_500);

#[derive(Default)]
pub struct Status {
    cpu: ProcessorStatus,
    gpu: ProcessorStatus,
    battery: Option<f32>,
    battery_charging: bool,
    volume: f32,
    audio_spectrum: Arc<[AtomicU32; AUDIO_SPECTRUM_BANDS]>,
    sample_time: f32,
    /// Smoothed spectrum and CPU/GPU temperatures, eased toward live readings each frame.
    damped_spectrum: [f32; AUDIO_SPECTRUM_BANDS],
    damped_temperatures: [f32; 2],
    power_hold: Option<(usize, Instant)>,
}

impl Status {
    pub fn new(updater: AppUpdater) -> Self {
        let status = Self::default();
        let spectrum = Arc::clone(&status.audio_spectrum);
        thread::spawn(move || monitor_playback(&spectrum));
        thread::spawn(move || monitor(&updater));
        status
    }

    pub fn pill(&mut self, time: f32, dt: f32) -> Data {
        for (damped, level) in self.damped_spectrum.iter_mut().zip(self.audio_spectrum.iter()) {
            let target = f32::from_bits(level.load(Ordering::Relaxed));
            let response = if target > *damped { 18.0 } else { 6.0 };
            damp(damped, target, response, dt);
        }
        let targets = [self.cpu.temperature, self.gpu.temperature];
        for (temperature, target) in self.damped_temperatures.iter_mut().zip(targets) {
            if *temperature == 0.0 {
                *temperature = target;
            } else {
                damp(temperature, target, 4.0, dt);
            }
        }
        let battery = self.battery.filter(|level| *level < FULL_BATTERY_LEVEL);
        let [cpu_temperature, gpu_temperature] = self.damped_temperatures;
        Data {
            battery_level: battery.unwrap_or(-1.0),
            battery_charging: f32::from(self.battery_charging),
            volume: self.volume,
            audio_spectrum: self.damped_spectrum,
            history_scroll: ((time - self.sample_time) / SAMPLE_INTERVAL.as_secs_f32()).saturate(),
            cpu: ProcessorStatus {
                temperature: cpu_temperature,
                ..self.cpu
            },
            gpu: ProcessorStatus {
                temperature: gpu_temperature,
                ..self.gpu
            },
            ..Default::default()
        }
    }

    fn adjust_volume(&mut self, direction: i32) {
        let sign = self.volume.signum();
        self.volume = (self.volume.abs() - direction as f32 * VOLUME_STEP).saturate() * sign;
        let volume = format!("{:.3}", self.volume.abs());
        thread::spawn(move || {
            if let Err(error) = Command::new("wpctl")
                .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &volume])
                .status()
            {
                warn!(%error, "Failed to set PipeWire volume");
            }
        });
    }

    pub fn interact(
        &mut self,
        pill: &mut Data,
        screen_width: f32,
        height: f32,
        ui: &mut InteractionState,
    ) {
        let x = pill_x(screen_width);
        let scroll = ui.scroll(section_rect(pill, x, height, StatusSection::Audio));
        if scroll != 0 {
            self.adjust_volume(scroll);
        }

        let buttons = StatusSection::POWER_ACTIONS
            .map(|section| ui.surface(section_rect(pill, x, height, section)));
        pill.power_hover = buttons
            .iter()
            .position(|response| response.hovered)
            .map_or(0.0, |action| action as f32 + 1.0);
        if let Some(action) = buttons.iter().position(|response| response.pressed) {
            self.power_hold = Some((action, Instant::now()));
        }
        if let Some((action, started)) = self.power_hold {
            let progress = started.elapsed().as_secs_f32() / POWER_HOLD_DURATION.as_secs_f32();
            if !ui.down() || !buttons[action].hovered {
                self.power_hold = None;
            } else if progress >= 1.0 {
                self.power_hold = None;
                let command = ["poweroff", "reboot"][action];
                if let Err(error) = Command::new("systemctl").arg(command).spawn() {
                    warn!(%error, %command, "Failed to run held power action");
                }
            } else {
                pill.power_state = action as f32 + 1.0 + progress;
            }
        }
        ui.surface(Rect::pill(x, WIDTH, height));
    }
}

fn damp(value: &mut f32, target: f32, response: f32, dt: f32) {
    *value += (target - *value) * (1.0 - (-response * dt).exp());
}

fn section_rect(pill: &Data, x: f32, height: f32, section: StatusSection) -> Rect {
    Rect::from_center(
        vec2(
            x + pill.section_center(section),
            crate::PANEL_START + height * 0.5,
        ),
        vec2((pill.section_width(section) + GAP) * 0.5, height * 0.5),
    )
}

fn monitor(updater: &AppUpdater) {
    let (Ok(mut system), Ok(mut components)) = (System::new(), Components::new_with_refreshed_list())
    else {
        warn!("sysinfo unavailable; system status monitor disabled");
        return;
    };
    let mut cpu = ProcessorStatus::default();
    let mut gpu = ProcessorStatus::default();
    let mut gpus = Gpus::new_with_refreshed_list().ok();
    loop {
        system.refresh_cpu_usage();
        system.refresh_cpu_temperature();
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
            .and_then(|output| String::from_utf8(output.status.success().then_some(output.stdout)?).ok())
            .and_then(|text| {
                Some((
                    text.split_whitespace().nth(1)?.parse::<f32>().ok()?,
                    text.contains("MUTED"),
                ))
            });

        if let Some(cpu_device) = system.cpus().as_ref().first() {
            cpu.temperature = cpu_device.temperature();
        }
        cpu.usage.push(system.global_cpu_usage() / 100.0);
        cpu.memory
            .push(system.used_memory() as f32 / system.total_memory().max(1) as f32);

        if let Some(gpu_device) = gpus.as_ref().and_then(|gpus| gpus.list().first()) {
            gpu.temperature = gpu_device.temperature().unwrap_or_default();
            gpu.usage
                .push((gpu_device.usage().unwrap_or_default() / 100.0).saturate());
            gpu.memory.push(
                gpu_device.used_memory().unwrap_or_default() as f32
                    / gpu_device.total_memory().unwrap_or_default().max(1) as f32,
            );
        }

        if !send_update(updater, move |app| {
            if let Some(status) = &mut app.status {
                status.cpu = cpu;
                status.gpu = gpu;
                status.battery = battery;
                status.battery_charging = battery_charging;
                if let Some(audio) = audio {
                    status.volume = audio.0 * if audio.1 { -1.0 } else { 1.0 };
                }
                status.sample_time = app.render.start_time.elapsed().as_secs_f32();
            }
        }) {
            break;
        }
        thread::sleep(SAMPLE_INTERVAL);
    }
}

fn monitor_playback(levels: &[AtomicU32; AUDIO_SPECTRUM_BANDS]) {
    loop {
        if let Err(error) = capture_playback(levels) {
            warn!(%error, "PipeWire playback meter stopped");
        }
        for level in levels {
            level.store(0.0f32.to_bits(), Ordering::Relaxed);
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn capture_playback(levels: &[AtomicU32; AUDIO_SPECTRUM_BANDS]) -> io::Result<()> {
    let mut child = Command::new("pw-record")
        .args([
            "--properties",
            "stream.capture.sink=true",
            "--rate",
            "48000",
            "--channels",
            "1",
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
    let mut analyzer = SpectrumAnalyzer::default();
    loop {
        let count = output.read(&mut bytes)?;
        if count == 0 {
            break;
        }
        let aligned = count - count % size_of::<f32>();
        let samples: &[f32] = bytemuck::cast_slice(&bytes[..aligned]);
        analyzer.push(samples, |spectrum| {
            for (level, value) in levels.iter().zip(spectrum) {
                level.store(value.to_bits(), Ordering::Relaxed);
            }
        });
    }
    child.wait()?;
    Ok(())
}

struct SpectrumAnalyzer {
    filters: [Bandpass; AUDIO_SPECTRUM_BANDS],
    samples: usize,
}

impl Default for SpectrumAnalyzer {
    fn default() -> Self {
        Self {
            filters: array::from_fn(|band| {
                Bandpass::new(AUDIO_BAND_EDGES[band], AUDIO_BAND_EDGES[band + 1])
            }),
            samples: 0,
        }
    }
}

impl SpectrumAnalyzer {
    fn push(&mut self, input: &[f32], mut publish: impl FnMut([f32; AUDIO_SPECTRUM_BANDS])) {
        for &sample in input {
            for filter in &mut self.filters {
                filter.push(sample);
            }
            self.samples += 1;
            if self.samples == AUDIO_WINDOW_SIZE {
                publish(self.filters.each_mut().map(Bandpass::level));
                self.samples = 0;
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Bandpass {
    coefficients: [f32; 4],
    state: [f32; 4],
    power: f32,
}

impl Bandpass {
    fn new(low: f32, high: f32) -> Self {
        let center = (low * high).sqrt();
        let omega = TAU * center / AUDIO_SAMPLE_RATE;
        let alpha = omega.sin() * (high - low) / (center * 2.0);
        let scale = 1.0 / (1.0 + alpha);
        Self {
            coefficients: [
                alpha * scale,
                -alpha * scale,
                -2.0 * omega.cos() * scale,
                (1.0 - alpha) * scale,
            ],
            ..Self::default()
        }
    }

    fn push(&mut self, sample: f32) {
        let [b0, b2, a1, a2] = self.coefficients;
        let output = b0 * sample + b2 * self.state[1] - a1 * self.state[2] - a2 * self.state[3];
        self.state = [sample, self.state[0], output, self.state[2]];
        self.power += output * output;
    }

    fn level(&mut self) -> f32 {
        let rms = (self.power / AUDIO_WINDOW_SIZE as f32).sqrt();
        self.power = 0.0;
        if rms <= 0.000_001 {
            0.0
        } else {
            ((20.0 * rms.log10() + 60.0) / 48.0).clamp(0.0, 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn analyze(frequency: f32) -> [f32; AUDIO_SPECTRUM_BANDS] {
        let samples = array::from_fn::<_, { AUDIO_WINDOW_SIZE * 4 }, _>(|index| {
            (TAU * frequency * index as f32 / AUDIO_SAMPLE_RATE).sin() * 0.25
        });
        let mut levels = [0.0; AUDIO_SPECTRUM_BANDS];
        SpectrumAnalyzer::default().push(&samples, |spectrum| levels = spectrum);
        levels
    }

    #[test]
    fn silence_has_no_spectrum() {
        assert!(analyze(0.0).iter().all(|level| level.abs() < f32::EPSILON));
    }

    #[test]
    fn tone_appears_in_its_frequency_band() {
        let levels = analyze(750.0);
        let strongest = levels
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .unwrap()
            .0;
        assert_eq!(strongest, 3, "750 Hz spectrum: {levels:?}");
    }

    #[test]
    fn analyzer_publishes_complete_windows() {
        let mut frames = 0;
        SpectrumAnalyzer::default().push(&[0.0; AUDIO_WINDOW_SIZE * 2], |_| frames += 1);
        assert_eq!(frames, 2);
    }
}
