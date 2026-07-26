use crate::{
    GAP, GlobalUniforms, PADDING, cloud_mass, fill, hash, pill_fragment, pill_vertex, sd_capsule_box,
    sd_chevron, sd_rounded_box, smoothstep, stroke, tempo::scene,
};
use core::f32::consts::TAU;
use spirv_std::{
    arch::kill,
    glam::{FloatExt, Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

const STATUS_HISTORY_SAMPLES: usize = 40;
pub const AUDIO_SPECTRUM_BANDS: usize = 7;
const STATUS_HISTORY_PACKS: usize = STATUS_HISTORY_SAMPLES / 4;
const PROCESSOR_WIDTH: f32 = 60.0;
const DATA_WIDTH: f32 = 32.0;
const ACTION_WIDTH: f32 = 24.0;
pub const WIDTH: f32 = PADDING * 2.0 + (PROCESSOR_WIDTH + DATA_WIDTH + ACTION_WIDTH) * 2.0 + GAP * 5.0;

pub const fn pill_x(screen_width: f32) -> f32 {
    screen_width - WIDTH - GAP
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StatusSection {
    Cpu,
    Gpu,
    Battery,
    Audio,
    Reboot,
    Power,
}

impl StatusSection {
    const ALL: [Self; 6] = [
        Self::Cpu,
        Self::Gpu,
        Self::Battery,
        Self::Audio,
        Self::Reboot,
        Self::Power,
    ];
    pub const POWER_ACTIONS: [Self; 2] = [Self::Power, Self::Reboot];
}

gpu_data!(Data {
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
    /// One-based hovered power action index; zero means neither action is hovered.
    pub power_hover: f32,
    /// Current sun height and decoded sky condition.
    pub sun_height: f32,
    pub conditions: [f32; 6],
});

impl Data {
    pub const fn section_center(&self, section: StatusSection) -> f32 {
        let mut x = PADDING;
        let mut index = 0;
        while index < section as usize {
            let previous = StatusSection::ALL[index];
            if self.battery_level >= 0.0 || !matches!(previous, StatusSection::Battery) {
                x += self.section_width(previous) + GAP;
            }
            index += 1;
        }
        x + self.section_width(section) * 0.5
    }

    pub const fn section_width(&self, section: StatusSection) -> f32 {
        let width = match section {
            StatusSection::Cpu | StatusSection::Gpu => PROCESSOR_WIDTH,
            StatusSection::Battery | StatusSection::Audio => DATA_WIDTH,
            StatusSection::Reboot | StatusSection::Power => ACTION_WIDTH,
        };
        if self.battery_level < 0.0 && matches!(section, StatusSection::Cpu | StatusSection::Gpu) {
            width + (DATA_WIDTH + GAP) * 0.5
        } else {
            width
        }
    }

    fn section_at(&self, x: f32) -> (StatusSection, f32) {
        let mut edge = PADDING;
        let mut center = 0.0;
        let mut index = 0;
        while index < StatusSection::ALL.len() {
            let section = StatusSection::ALL[index];
            if !matches!(section, StatusSection::Battery) || self.battery_level >= 0.0 {
                let width = self.section_width(section);
                edge += width + GAP;
                center = edge - GAP - width * 0.5;
                if x < edge - GAP * 0.5 {
                    return (section, center);
                }
            }
            index += 1;
        }
        (StatusSection::Power, center)
    }
}

gpu_data!(UsageHistory {
    samples: [u32; STATUS_HISTORY_PACKS],
});

impl UsageHistory {
    const fn get(&self, index: usize) -> f32 {
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

gpu_data!(ProcessorStatus {
    pub temperature: f32,
    pub usage: UsageHistory,
    pub memory: UsageHistory,
});

const CHART_HEIGHT: f32 = 9.2;
const PROCESSOR_RADIUS: f32 = 13.0;
const CHART_LINE_WIDTH: f32 = 0.85;
const USAGE_COLOR: Vec3 = Vec3::new(0.32, 0.68, 1.0);
const MEMORY_COLOR: Vec3 = Vec3::new(0.78, 0.3, 1.0);
const MUTED_COLOR: Vec3 = Vec3::new(1.0, 0.24, 0.3);
const HISTORY_END: usize = STATUS_HISTORY_SAMPLES - 1;

fn fill_box(point: Vec2, half_size: Vec2, radius: f32) -> f32 {
    fill(sd_rounded_box(point, half_size, radius))
}

fn heat_color(temperature: f32) -> Vec3 {
    vec3(0.22, 0.62, 1.0)
        .lerp(vec3(1.0, 0.38, 0.08), smoothstep(60.0, 72.0, temperature))
        .lerp(vec3(1.0, 0.08, 0.035), smoothstep(72.0, 88.0, temperature))
}

fn thermal_smoke(point: Vec2, time: f32, temperature: f32) -> Vec2 {
    let outward = sd_capsule_box(point, 13.0, 13.0);
    let cloud = cloud_mass(point + vec2(time * 1.8, -time), 4.0, 0.0);
    let envelope = smoothstep(-0.5, 1.5, outward) * smoothstep(14.0, 2.0, outward);
    vec2(
        envelope * (0.18 + cloud * 0.34),
        envelope * smoothstep(0.3, 0.62, cloud),
    ) * smoothstep(62.0, 84.0, temperature)
}

fn cpu_pin_distance(point: Vec2, frame_half_width: f32) -> f32 {
    let point = point.abs();
    let half_span = frame_half_width - PROCESSOR_RADIUS;
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
    let curve_y = (PROCESSOR_RADIUS * PROCESSOR_RADIUS - curve_x * curve_x).sqrt();
    let long_edge = pin(vec2(x, curve_y), vec2(curve_x, curve_y) / PROCESSOR_RADIUS);
    let y = (point.y / 8.0).round().min(1.0) * 8.0;
    let curve_x = (PROCESSOR_RADIUS * PROCESSOR_RADIUS - y * y).sqrt();
    let end_cap = pin(vec2(half_span + curve_x, y), vec2(curve_x, y) / PROCESSOR_RADIUS);
    long_edge.min(end_cap)
}

fn processor_monitor(
    point: Vec2,
    processor: ProcessorStatus,
    scroll: f32,
    background: Vec3,
    cpu: bool,
    slot_width: f32,
) -> Vec3 {
    let frame_half_width = slot_width * 0.5 - GAP * 0.5;
    let capsule = sd_capsule_box(point, frame_half_width - PROCESSOR_RADIUS, PROCESSOR_RADIUS);
    let (pins, pin_alpha) = if cpu {
        (cpu_pin_distance(point, frame_half_width), 1.0)
    } else {
        (1_000.0, 0.0)
    };
    let shape = crate::smooth_union(capsule, pins, 1.6, pin_alpha);
    let chart_half_width = slot_width * 0.5 - PADDING;
    let chart = fill(sd_capsule_box(
        point,
        chart_half_width - CHART_HEIGHT,
        CHART_HEIGHT,
    ));
    let history_step = chart_half_width * 2.0 / HISTORY_END as f32;
    let sample = ((point.x + chart_half_width) / history_step + scroll).clamp(0.0, HISTORY_END as f32);
    let index = sample.floor() as usize;
    let progress = smoothstep(0.0, 1.0, sample.fract());
    let curve = |history: &UsageHistory, color: Vec3, fill_strength: f32| {
        let height = |i: usize| CHART_HEIGHT * (1.0 - history.get(i.min(HISTORY_END)) * 2.0);
        let graph_y = height(index).lerp(height(index + 1), progress);
        let line = stroke((point.y - graph_y).abs(), CHART_LINE_WIDTH);
        color * chart * (fill(graph_y - point.y) * fill_strength + line)
    };
    let graphs =
        curve(&processor.usage, USAGE_COLOR, 0.13) + curve(&processor.memory, MEMORY_COLOR, 0.07);
    let grid = (((point + vec2(chart_half_width, CHART_HEIGHT)) / vec2(7.0, 6.1)).fract() - 0.5).abs();
    let grid = smoothstep(0.49, 0.46, grid.x).max(smoothstep(0.49, 0.45, grid.y));
    let frame_color = vec3(0.025, 0.09, 0.15)
        .lerp(USAGE_COLOR, 0.18 + processor.usage.get(HISTORY_END) * 0.24)
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

fn battery_icon(point: Vec2, time: f32, pill: &Data) -> Vec3 {
    let point = point / 0.8;
    let charging = pill.battery_charging;
    let battery_level = pill.battery_level;
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

    let cell_size = vec2(4.5, 6.0);
    let bubble_field = (point + vec2(0.0, time * 6.0)) / cell_size;
    let random = hash(bubble_field.floor());
    let bubble_point =
        bubble_field.fract() * cell_size - cell_size * 0.5 - vec2((random.x - 0.5) * 1.2, 0.0);
    let distance = bubble_point.length() - (0.75 + random.x * 0.55);
    let bubble = stroke(distance, 0.58) * smoothstep(0.58, 0.72, random.y) * liquid * charging;
    Vec3::splat(shell * 0.43 + terminal * 0.38)
        + liquid_color * liquid * 0.78
        + liquid_color.lerp(Vec3::ONE, 0.72) * bubble * 0.9
}

fn audio_icon(point: Vec2, pill: &Data) -> Vec3 {
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

fn action_icon(point: Vec2, time: f32, action: f32, hover: f32, pill: &Data) -> Vec3 {
    let point = point / (1.0 + hover * 0.07);
    let action_state = pill.power_state;
    let selected = smoothstep(0.4, 0.05, (action_state.floor() - action - 1.0).abs());
    let charge = action_state.fract() * selected;
    let icon = if action < 0.5 {
        power_icon(point, time, charge + hover * (1.0 - charge) * 0.1)
    } else {
        reboot_icon(point, 1.0 - selected + charge)
    };
    let color =
        Vec3::splat(0.48).lerp(vec3(0.78, 0.3, 0.28), hover.max(selected * (0.5 + charge * 0.5)));
    color * icon * (1.0 + charge * 0.45)
}

#[spirv(vertex)]
pub fn vertex(
    #[spirv(vertex_index)] vertex: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] _status: &[Data],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_pixel: &mut Vec2,
) {
    let x = pill_x(global.screen_size.x);
    (*out_pos, *out_pixel) = pill_vertex(vertex, global, x, vec2(WIDTH, 0.0));
}

#[spirv(fragment)]
pub fn fragment(
    #[spirv(location = 0)] pixel: Vec2,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] status: &[Data],
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = status[0];
    let x = pill_x(global.screen_size.x);
    let (interaction, local, size, dist) = pill_fragment(pixel, global, x, WIDTH);
    let (dist, mask, alpha) = interaction.surface(dist);
    if alpha <= 0.0 {
        kill();
    }
    let refracted = interaction.refract(local, size, dist);
    let background = scene(global, refracted, size, dist, pill.sun_height, pill.conditions);
    let local = refracted * size;
    let (section, center_x) = pill.section_at(local.x);
    let section_center = |section| vec2(pill.section_center(section), size.y * 0.5);
    let point = local - vec2(center_x, size.y * 0.5);
    let smoke = thermal_smoke(
        local - section_center(StatusSection::Cpu),
        global.time,
        pill.cpu.temperature,
    )
    .max(thermal_smoke(
        local - section_center(StatusSection::Gpu),
        global.time,
        pill.gpu.temperature,
    ));
    let smoke_color = vec3(0.07, 0.12, 0.18).lerp(
        heat_color(pill.cpu.temperature.max(pill.gpu.temperature)),
        0.24 + smoke.y * 0.12,
    );
    let background = background
        .lerp(vec3(0.002, 0.006, 0.012), smoke.x * 0.46)
        .lerp(smoke_color, smoke.y * 0.64);
    let color = match section {
        StatusSection::Cpu | StatusSection::Gpu => processor_monitor(
            point,
            if section == StatusSection::Cpu {
                pill.cpu
            } else {
                pill.gpu
            },
            pill.history_scroll,
            background,
            section == StatusSection::Cpu,
            pill.section_width(section),
        ),
        StatusSection::Battery => background + battery_icon(point, global.time, &pill),
        StatusSection::Audio => background + audio_icon(point, &pill),
        StatusSection::Reboot | StatusSection::Power => {
            let action = if section == StatusSection::Power { 0.0 } else { 1.0 };
            let hover = smoothstep(0.4, 0.05, (pill.power_hover - action - 1.0).abs());
            background + action_icon(point, global.time, action, hover, &pill)
        }
    }
    .lerp(Vec3::splat(0.95), interaction.ripple_flash * 0.35);
    *out_color = (color * mask).extend(alpha);
}
