use crate::render::{
    shader::{
        cloud_mass, fill, hash, pill_fragment, pill_sheen, pill_vertex, sd_capsule_box, sd_chevron,
        sd_rounded_box, segment_distance, smooth_union, stroke,
    },
    shared::{FrameData, GAP, PADDING, smoothstep},
    tempestas::WeatherCondition,
    text,
};
use core::f32::consts::TAU;
use isthmus::glam::{FloatExt, Vec2, Vec3, Vec4, vec2, vec3};
use spirv_std::arch::kill;

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use isthmus::Vertex;
#[cfg(feature = "cpu")]
use {
    crate::{
        app::{AppUpdater, interaction::Rect, platform::linux as platform},
        render::{
            cpu::{Frame, Passes},
            shared::PANEL_START,
            text::TextStyle,
        },
    },
    arrayvec::ArrayString,
    isthmus::StatePass,
    std::{
        fmt::Write,
        sync::{
            Arc,
            atomic::{AtomicU32, Ordering},
        },
    },
};

const STATUS_HISTORY_SAMPLES: usize = 40;
pub const AUDIO_SPECTRUM_BANDS: usize = 7;
#[cfg(feature = "cpu")]
pub(crate) const BATTERY_HIDDEN: f32 = 2.0;
const DATA_WIDTH: f32 = 32.0;
const ACTION_WIDTH: f32 = 24.0;
/// CPU/GPU graphs stay this wide regardless of whether the battery slot is present.
const GRAPH_WIDTH: f32 = 60.0 + (DATA_WIDTH + GAP) * 0.5;

const fn pill_x(screen_width: f32, width: f32) -> f32 {
    screen_width - width - GAP
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum StatusSection {
    Cpu,
    Gpu,
    Battery,
    Audio,
    Reboot,
    Power,
}

impl StatusSection {
    #[cfg(feature = "cpu")]
    const POWER_ACTIONS: [Self; 2] = [Self::Power, Self::Reboot];

    const fn width(self) -> f32 {
        match self {
            Self::Cpu | Self::Gpu => GRAPH_WIDTH,
            Self::Battery | Self::Audio => DATA_WIDTH,
            Self::Reboot | Self::Power => ACTION_WIDTH,
        }
    }
}

/// Width the battery slot adds to every section after it when shown.
const BATTERY_SLOT: f32 = DATA_WIDTH + GAP;
const CPU_X: f32 = PADDING;
const GPU_X: f32 = CPU_X + GRAPH_WIDTH + GAP;
const BATTERY_X: f32 = GPU_X + GRAPH_WIDTH + GAP;
const AUDIO_X: f32 = BATTERY_X;
const REBOOT_X: f32 = AUDIO_X + DATA_WIDTH + GAP;
const POWER_X: f32 = REBOOT_X + ACTION_WIDTH + GAP;

#[isthmus::data]
#[derive(Default)]
pub struct StatusPill {
    /// Battery charge magnitude; negative means charging and values above one hide it.
    pub battery_level: f32,
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
    /// Power action being held, or -1 when none is.
    pub power_action: i32,
    /// How far through its hold `power_action` is.
    pub power_progress: f32,
    /// Power action under the pointer, or -1 when none is.
    pub power_hover: i32,
    /// Current sun height and decoded sky condition.
    pub sun_height: f32,
    pub conditions: WeatherCondition,
    pub text: text::Text<2, { MAX_LABEL_CHARS * 2 }>,
}

const MAX_LABEL_CHARS: usize = 16;
const CPU_TEXT: usize = 0;
const GPU_TEXT: usize = 1;

impl StatusPill {
    const fn battery_present(&self) -> bool {
        self.battery_level >= -1.0 && self.battery_level <= 1.0
    }

    const fn battery_charge(&self) -> f32 {
        self.battery_level.abs()
    }

    /// Left edge of `section`; everything after the battery shifts when it is shown.
    const fn section_x(&self, section: StatusSection) -> f32 {
        let shift = if self.battery_present() { BATTERY_SLOT } else { 0.0 };
        match section {
            StatusSection::Cpu => CPU_X,
            StatusSection::Gpu => GPU_X,
            StatusSection::Battery => BATTERY_X,
            StatusSection::Audio => AUDIO_X + shift,
            StatusSection::Reboot => REBOOT_X + shift,
            StatusSection::Power => POWER_X + shift,
        }
    }

    const fn section_center(&self, section: StatusSection) -> f32 {
        self.section_x(section) + section.width() * 0.5
    }

    /// Total on-screen width of the pill; grows to include the battery slot when shown.
    pub const fn width(&self) -> f32 {
        self.section_x(StatusSection::Power) + ACTION_WIDTH + PADDING
    }

    const fn ends_before(&self, section: StatusSection, x: f32) -> bool {
        x < self.section_x(section) + section.width() + GAP * 0.5
    }

    const fn section_at(&self, x: f32) -> (StatusSection, f32) {
        let section = if self.ends_before(StatusSection::Cpu, x) {
            StatusSection::Cpu
        } else if self.ends_before(StatusSection::Gpu, x) {
            StatusSection::Gpu
        } else if self.battery_present() && self.ends_before(StatusSection::Battery, x) {
            StatusSection::Battery
        } else if self.ends_before(StatusSection::Audio, x) {
            StatusSection::Audio
        } else if self.ends_before(StatusSection::Reboot, x) {
            StatusSection::Reboot
        } else {
            StatusSection::Power
        };
        (section, self.section_center(section))
    }
}

#[isthmus::data]
pub struct UsageHistory {
    samples: [f32; STATUS_HISTORY_SAMPLES],
}

impl Default for UsageHistory {
    fn default() -> Self {
        Self {
            samples: [0.0; STATUS_HISTORY_SAMPLES],
        }
    }
}

#[isthmus::data]
#[derive(Default)]
pub struct ProcessorStatus {
    pub temperature: f32,
    pub usage: UsageHistory,
    pub memory: UsageHistory,
}

#[cfg(feature = "cpu")]
impl UsageHistory {
    pub fn push(&mut self, value: f32) {
        self.samples.copy_within(1.., 0);
        self.samples[STATUS_HISTORY_SAMPLES - 1] = value.saturate();
    }

    pub const fn latest(&self) -> f32 {
        self.samples[STATUS_HISTORY_SAMPLES - 1]
    }
}

#[cfg(feature = "cpu")]
fn section_rect(pill: &StatusPill, x: f32, height: f32, section: StatusSection) -> Rect {
    Rect::from_center(
        vec2(x + pill.section_center(section), PANEL_START + height * 0.5),
        vec2((section.width() + GAP) * 0.5, height * 0.5),
    )
}

const CHART_LINE_WIDTH: f32 = 0.85;
const USAGE_COLOR: Vec3 = Vec3::new(0.32, 0.68, 1.0);
const MEMORY_COLOR: Vec3 = Vec3::new(0.78, 0.3, 1.0);
const MUTED_COLOR: Vec3 = Vec3::new(1.0, 0.24, 0.3);
const TEXT_COLOR: Vec3 = Vec3::splat(0.94);
#[cfg(feature = "cpu")]
const LABEL_STYLE: TextStyle = TextStyle::new(11.0, 700.0);
const HISTORY_END: usize = STATUS_HISTORY_SAMPLES - 1;

fn status_sky(local: Vec2, distance: f32, sun: f32, weather: WeatherCondition, time: f32) -> Vec3 {
    let WeatherCondition {
        fog,
        cloud,
        rain,
        snow,
        lightning,
        hail,
    } = weather;
    let vertical = smoothstep(1.0, 0.0, local.y);
    let daylight = smoothstep(-0.04, 0.2, sun);
    let twilight = smoothstep(-0.2, 0.02, sun) * (1.0 - daylight);
    let mut color = vec3(0.008, 0.015, 0.04)
        .lerp(vec3(0.03, 0.06, 0.13), vertical)
        .lerp(
            vec3(0.09, 0.37, 0.65).lerp(vec3(0.34, 0.7, 0.9), vertical),
            daylight,
        )
        .lerp(
            vec3(0.65, 0.25, 0.2).lerp(vec3(0.3, 0.2, 0.4), vertical),
            twilight,
        );
    color = color.lerp(vec3(0.16, 0.2, 0.27), cloud * 0.34 + rain * 0.16 + hail * 0.08);
    color = color.lerp(Vec3::splat(0.82), snow * 0.16);
    color = color.lerp(vec3(0.62, 0.68, 0.72), fog * 0.62);
    color = color.lerp(
        vec3(0.65, 0.74, 0.96),
        smoothstep(0.92, 1.0, (time * 2.7).sin()) * lightning * 0.45,
    );
    color + pill_sheen(local.y, distance)
}

fn fill_box(point: Vec2, half_size: Vec2, radius: f32) -> f32 {
    fill(sd_rounded_box(point, half_size, radius))
}

fn heat_color(temperature: f32) -> Vec3 {
    vec3(0.22, 0.62, 1.0)
        .lerp(vec3(1.0, 0.38, 0.08), smoothstep(60.0, 72.0, temperature))
        .lerp(vec3(1.0, 0.08, 0.035), smoothstep(72.0, 88.0, temperature))
}

fn thermal_smoke(point: Vec2, time: f32, temperature: f32) -> Vec2 {
    if temperature <= 62.0 {
        return Vec2::ZERO;
    }
    let outward = sd_capsule_box(point, 13.0, 13.0);
    let cloud = cloud_mass(point + vec2(time * 1.8, -time), 4.0, 0.0);
    let envelope = smoothstep(-0.5, 1.5, outward) * smoothstep(14.0, 2.0, outward);
    vec2(
        envelope * (0.18 + cloud * 0.34),
        envelope * smoothstep(0.3, 0.62, cloud),
    ) * smoothstep(62.0, 84.0, temperature)
}

fn cpu_pin_distance(point: Vec2, frame_half_width: f32, radius: f32) -> f32 {
    let point = point.abs();
    let half_span = frame_half_width - radius;
    let pin = |boundary: Vec2, normal: Vec2| {
        let local = point - boundary - normal * 0.9;
        let tangent = vec2(-normal.y, normal.x);
        sd_rounded_box(
            vec2(local.dot(tangent), local.dot(normal)),
            vec2(1.55, 2.05),
            0.65,
        )
    };
    let x = ((point.x / 9.0).round() * 9.0).min(frame_half_width);
    let curve_x = (x - half_span).max(0.0);
    let curve_y = (radius * radius - curve_x * curve_x).sqrt();
    let long_edge = pin(vec2(x, curve_y), vec2(curve_x, curve_y) / radius);
    let y = (point.y / 8.0).round().min(1.0) * 8.0;
    let curve_x = (radius * radius - y * y).sqrt();
    let end_cap = pin(vec2(half_span + curve_x, y), vec2(curve_x, y) / radius);
    long_edge.min(end_cap)
}

fn processor_monitor(
    point: Vec2,
    processor: &ProcessorStatus,
    scroll: f32,
    background: Vec3,
    cpu: bool,
    slot_width: f32,
    pill_height: f32,
) -> Vec3 {
    let frame_half_width = slot_width * 0.5 - GAP * 0.5;
    let radius = pill_height * 0.5 - GAP;
    let capsule = sd_capsule_box(point, frame_half_width - radius, radius);
    let (pins, pin_alpha) = if cpu {
        (cpu_pin_distance(point, frame_half_width, radius), 1.0)
    } else {
        (1_000.0, 0.0)
    };
    let shape = smooth_union(capsule, pins, 1.6, pin_alpha);
    let chart = fill(capsule);
    let history_step = frame_half_width * 2.0 / HISTORY_END as f32;
    let sample = ((point.x + frame_half_width) / history_step + scroll).clamp(0.0, HISTORY_END as f32);
    let index = sample.floor() as usize;
    let graph_height = radius - 2.0;
    let curve = |history: &UsageHistory, color: Vec3, fill_strength: f32| {
        let height = |i: usize| graph_height * (1.0 - history.samples[i.min(HISTORY_END)] * 2.0);
        let sample_point =
            |i: usize| vec2((i as f32 - scroll) * history_step - frame_half_width, height(i));
        let start = sample_point(index);
        let end = sample_point(index + 1);
        let line = stroke(segment_distance(point, start, end), CHART_LINE_WIDTH);
        let graph_y = start.y.lerp(end.y, smoothstep(0.0, 1.0, sample.fract()));
        color * chart * (fill(graph_y - point.y) * fill_strength + line)
    };
    let graphs =
        curve(&processor.usage, USAGE_COLOR, 0.156) + curve(&processor.memory, MEMORY_COLOR, 0.084);
    let grid = (((point + vec2(frame_half_width, radius)) / vec2(7.0, 6.1)).fract() - 0.5).abs();
    let grid = smoothstep(0.49, 0.46, grid.x).max(smoothstep(0.49, 0.45, grid.y));
    let frame_color = vec3(0.025, 0.09, 0.15)
        .lerp(USAGE_COLOR, 0.18 + processor.usage.samples[HISTORY_END] * 0.24)
        .lerp(
            heat_color(processor.temperature),
            smoothstep(60.0, 86.0, processor.temperature) * 0.9,
        );
    background
        .lerp(vec3(0.004, 0.012, 0.026), fill(shape) * 0.82)
        .lerp(frame_color, stroke(capsule, 1.55) * 0.92)
        .lerp(frame_color, fill(pins) * pin_alpha * 0.78)
        + Vec3::splat(chart * grid * 0.045)
        + graphs
}

fn battery_icon(point: Vec2, time: f32, pill: &StatusPill) -> Vec3 {
    let point = point / 0.8;
    let charging = if pill.battery_level < 0.0 { 1.0 } else { 0.0 };
    let battery_level = pill.battery_charge();
    let shell = stroke(
        sd_rounded_box(point - vec2(0.0, 1.0), vec2(11.5, 15.0), 3.2),
        1.875,
    );
    let terminal = fill_box(point - vec2(0.0, -15.6), vec2(4.0, 1.8), 0.8);
    let inside = fill_box(point - vec2(0.0, 1.0), vec2(8.5, 12.0), 1.7);
    let level = 12.0 - battery_level.saturate() * 24.0;
    let wave = (point.x * 0.62 + time * (1.4 + charging * 1.2)).sin() * 1.15
        + (point.x * 0.27 - time * 0.8).sin() * 0.45;
    let liquid = inside * smoothstep(level + wave - 0.7, level + wave + 0.7, point.y - 1.0);
    let liquid_color = vec3(1.0, 0.18, 0.10)
        .lerp(vec3(1.0, 0.72, 0.12), smoothstep(0.08, 0.28, battery_level))
        .lerp(vec3(0.22, 0.95, 0.55), smoothstep(0.18, 0.72, battery_level));

    let cell_size = vec2(3.0, 3.4);
    let cell = (point / cell_size).floor();
    let seed = hash(cell);
    let center = (cell + 0.2 + seed * 0.6) * cell_size;
    let cycle = (time * (0.5 + seed.y) + seed.x * 11.0).fract();
    let distance = (point - center + vec2(0.0, cycle * 5.0)).length() - (0.4 + seed.y * 0.5);
    let fade = smoothstep(0.0, 0.25, cycle) * smoothstep(1.0, 0.7, cycle);
    let bubble = stroke(distance, 0.45) * fade * liquid * charging;
    Vec3::splat(shell * 0.43 + terminal * 0.38)
        + liquid_color * liquid * 0.78
        + liquid_color.lerp(Vec3::ONE, 0.72) * bubble * 0.9
}

fn audio_icon(point: Vec2, pill: &StatusPill) -> Vec3 {
    let muted = if pill.volume < 0.0 { 1.0 } else { 0.0 };
    let volume = pill.volume.abs();
    let bar = ((point.x + 12.0) / 4.0).round().clamp(0.0, 6.0);
    let active = pill.audio_spectrum[bar as usize] * (1.0 - muted);
    let height = 1.2 + 7.7 * active;
    let distance = sd_rounded_box(point - vec2(-12.0 + bar * 4.0, -1.5), vec2(1.25, height), 1.25);
    let rail_point = point - vec2(0.0, 11.5);
    let rail = fill_box(rail_point, vec2(14.0, 1.25), 1.25);
    let level = -14.0 + volume.saturate() * 28.0;
    let level = rail * smoothstep(level + 0.8, level - 0.8, rail_point.x);

    let audio_color = vec3(0.08, 0.88, 1.0).lerp(vec3(0.65, 0.34, 1.0), volume * 0.65);
    audio_color
        * (smoothstep(0.7, -0.7, distance) * (0.58 + active * 0.35)
            + smoothstep(3.2, 0.0, distance) * active * 0.12)
        + audio_color.lerp(MUTED_COLOR, muted) * (level + rail * (1.0 - level) * 0.22)
}

fn power_icon(point: Vec2, time: f32, charge: f32) -> f32 {
    let ease = smoothstep(0.0, 1.0, charge);
    let radius = 7.5 - charge * 4.6 + (time * 8.0).sin() * charge * (1.0 - charge) * 0.16;
    let ring = stroke(point.length() - radius, 1.05 + ease * 0.7);
    let gap = fill_box(point - vec2(0.0, -7.0), vec2(3.0 * (1.0 - charge), 3.0), 0.5);
    let stem = fill_box(
        point - vec2(0.0, -5.0 + charge * 3.5),
        vec2(1.05 + ease * 0.45, 4.6 - charge * 3.0),
        0.7,
    );
    (ring * (1.0 - gap)).max(stem)
}

fn reboot_icon(point: Vec2, progress: f32) -> f32 {
    const START: f32 = TAU * 0.08;
    const SWEEP: f32 = TAU * 0.82;

    let phase = ((point.y.atan2(point.x) - START) / TAU + 1.0).fract();
    let arc_end = (progress * 0.82 - 0.045).max(0.0);
    let arc = stroke(point.length() - 7.1, 1.05)
        * smoothstep(arc_end + 0.008, arc_end - 0.008, phase)
        * smoothstep(0.0, 0.02, progress);

    let angle = START + SWEEP * progress;
    let direction = vec2(angle.cos(), angle.sin());
    let tangent = vec2(-direction.y, direction.x);
    let arrow_point = point - direction * 7.1;
    let arrow_point = vec2(arrow_point.dot(tangent), arrow_point.dot(direction));
    let arrow = smoothstep(0.7, -0.7, sd_chevron(arrow_point, vec2(-3.2, 2.1)) - 1.0);
    arc.max(arrow)
}

fn action_icon(point: Vec2, time: f32, action: f32, hover: f32, pill: &StatusPill) -> Vec3 {
    let point = point / (1.0 + hover * 0.07);
    let selected = smoothstep(0.4, 0.05, (pill.power_action as f32 - action).abs());
    let charge = pill.power_progress * selected;
    let icon = if action < 0.5 {
        power_icon(point, time, charge)
    } else {
        reboot_icon(point, 1.0 - selected + charge)
    };
    let color =
        Vec3::splat(0.48).lerp(vec3(0.78, 0.3, 0.28), hover.max(selected * (0.5 + charge * 0.5)));
    color * icon * (1.0 + charge * 0.45)
}

#[derive(isthmus::Varyings)]
pub struct Varyings {
    pub pixel: Vec2,
}

#[isthmus::pass]
pub struct StatusPass {
    pub pill: StatePass<Self>,
    audio_spectrum: Arc<[AtomicU32; AUDIO_SPECTRUM_BANDS]>,
}

#[isthmus::pass]
impl StatusPass {
    pub fn new(passes: &Passes<'_>, updater: AppUpdater, font: &text::Font) -> Self {
        let audio_spectrum = Arc::<[AtomicU32; AUDIO_SPECTRUM_BANDS]>::default();
        platform::start_status_monitor(updater, Arc::clone(&audio_spectrum));
        let pill = passes.state_with(
            Resources {
                glyphs: &font.glyphs,
                edges: &font.edges,
            },
            StatusPill {
                battery_level: BATTERY_HIDDEN,
                power_action: -1,
                power_hover: -1,
                ..Default::default()
            },
        );
        Self { pill, audio_spectrum }
    }

    pub fn update(&mut self, font: &text::Font, frame: &mut Frame) {
        let height = frame.config.height;
        let pill = &mut *self.pill;
        for (damped, level) in pill.audio_spectrum.iter_mut().zip(self.audio_spectrum.iter()) {
            let target = f32::from_bits(level.load(Ordering::Relaxed));
            let response = if target > *damped { 18.0 } else { 6.0 };
            *damped += (target - *damped) * (1.0 - (-response * frame.delta_time).exp());
        }
        pill.history_scroll = (pill.history_scroll
            + frame.delta_time / platform::STATUS_SAMPLE_INTERVAL.as_secs_f32())
        .saturate();

        let width = pill.width();
        let x = pill_x(frame.shared.screen_size.x, width);
        let scroll = frame
            .interaction
            .scroll(section_rect(pill, x, height, StatusSection::Audio));
        if scroll != 0 {
            let sign = pill.volume.signum();
            pill.volume = (pill.volume.abs() - scroll as f32 * 0.05).saturate() * sign;
            platform::set_volume(pill.volume.abs());
        }

        let buttons = StatusSection::POWER_ACTIONS
            .map(|section| frame.interaction.surface(section_rect(pill, x, height, section)));
        pill.power_hover = buttons
            .iter()
            .position(|response| response.hovered)
            .map_or(-1, |action| action as i32);
        if let Some(action) = buttons.iter().position(|response| response.pressed) {
            pill.power_action = action as i32;
            pill.power_progress = 0.0;
        }
        if pill.power_action >= 0 {
            let action = pill.power_action as usize;
            let progress = pill.power_progress + frame.delta_time / 1.5;
            if !frame.interaction.down() || !buttons[action].hovered {
                pill.power_action = -1;
            } else if progress >= 1.0 {
                pill.power_action = -1;
                platform::run_power_action(action);
            } else {
                pill.power_progress = progress;
            }
        }
        frame.interaction.surface(Rect::pill(x, width, height));

        pill.text.clear();
        for section in [StatusSection::Cpu, StatusSection::Gpu] {
            let processor = match section {
                StatusSection::Cpu => &pill.cpu,
                _ => &pill.gpu,
            };
            let mut label = ArrayString::<MAX_LABEL_CHARS>::new();
            write!(
                label,
                "{:.0}% {:.0}% {:.0}\u{b0}C",
                processor.usage.latest() * 100.0,
                processor.memory.latest() * 100.0,
                processor.temperature,
            )
            .unwrap();
            let section_center = pill.section_center(section);
            let half_width = section.width() * 0.5 - GAP * 0.5;
            let offset = pill.text.fit(
                font,
                &label,
                LABEL_STYLE,
                GAP + 5.0,
                section_center - half_width,
                section_center + half_width,
            );
            debug_assert_eq!(
                offset,
                if section == StatusSection::Cpu {
                    CPU_TEXT
                } else {
                    GPU_TEXT
                }
            );
        }
    }

    #[gpu]
    pub fn vertex(
        #[gpu(vertex_index)] vertex: u32,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] pill: StatusPill,
    ) -> Vertex<Varyings> {
        let width = pill.width();
        let x = pill_x(frame.screen_size.x, width);
        let (position, pixel) = pill_vertex(vertex, frame, x, vec2(width, 0.0));
        Vertex {
            position,
            varyings: Varyings { pixel },
        }
    }

    #[gpu]
    pub fn fragment(
        Varyings { pixel }: Varyings,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] pill: StatusPill,
        #[gpu(resource)] glyphs: &[text::Glyph],
        #[gpu(resource)] edges: &[text::Edge],
    ) -> Vec4 {
        let width = pill.width();
        let x = pill_x(frame.screen_size.x, width);
        let (interaction, raw_local, size, surface) = pill_fragment(pixel, frame, x, width);
        let (dist, mask, alpha) = interaction.surface(surface);
        if alpha <= 1.0 / 1024.0 {
            kill();
        }
        let refracted = interaction.refract(raw_local, size, dist);
        let background = status_sky(refracted, dist, pill.sun_height, pill.conditions, frame.time);
        let local = refracted * size;
        let (section, center_x) = pill.section_at(local.x);
        let section_center = |section| vec2(pill.section_center(section), size.y * 0.5);
        let point = local - vec2(center_x, size.y * 0.5);
        let smoke = if matches!(section, StatusSection::Cpu | StatusSection::Gpu) {
            thermal_smoke(
                local - section_center(StatusSection::Cpu),
                frame.time,
                pill.cpu.temperature,
            )
            .max(thermal_smoke(
                local - section_center(StatusSection::Gpu),
                frame.time,
                pill.gpu.temperature,
            ))
        } else {
            Vec2::ZERO
        };
        let smoke_color = vec3(0.07, 0.12, 0.18).lerp(
            heat_color(pill.cpu.temperature.max(pill.gpu.temperature)),
            0.24 + smoke.y * 0.12,
        );
        let background = background
            .lerp(vec3(0.002, 0.006, 0.012), smoke.x * 0.46)
            .lerp(smoke_color, smoke.y * 0.64);
        let color = match section {
            StatusSection::Cpu => processor_monitor(
                point,
                &pill.cpu,
                pill.history_scroll,
                background,
                true,
                section.width(),
                size.y,
            ),
            StatusSection::Gpu => processor_monitor(
                point,
                &pill.gpu,
                pill.history_scroll,
                background,
                false,
                section.width(),
                size.y,
            ),
            StatusSection::Battery => background + battery_icon(point, frame.time, pill),
            StatusSection::Audio => background + audio_icon(point, pill),
            StatusSection::Reboot | StatusSection::Power => {
                let action = if section == StatusSection::Power { 0.0 } else { 1.0 };
                let hover = smoothstep(0.4, 0.05, (pill.power_hover as f32 - action).abs());
                background + action_icon(point, frame.time, action, hover, pill)
            }
        };

        let text_offset = if section == StatusSection::Cpu {
            CPU_TEXT
        } else {
            GPU_TEXT
        };
        let text_distance = if matches!(section, StatusSection::Cpu | StatusSection::Gpu) {
            text::line_distance(&pill.text, text_offset, glyphs, edges, raw_local)
        } else {
            -1e6
        };
        let color = color.lerp(TEXT_COLOR, text::alpha(text_distance));

        let color = color.lerp(Vec3::splat(0.95), interaction.ripple_flash * 0.35);
        (color * mask).extend(alpha)
    }
}
