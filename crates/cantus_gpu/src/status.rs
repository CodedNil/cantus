use crate::{
    fill, pill_fragment, pill_vertex, sd_capsule_box, sd_chevron, sd_rounded_box, stroke,
    weather::{cloud_mass, hash, sky_background},
};
use cantus_shared::{
    GlobalUniforms, ProcessorStatus, STATUS_HISTORY_SAMPLES, StatusPill, UsageHistory, smoothstep,
};
use core::f32::consts::{PI, TAU};
use spirv_std::{
    arch::kill,
    glam::{FloatExt, Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

const CHART_SIZE: Vec2 = Vec2::new(21.0, 9.2);
const CHART_LINE_WIDTH: f32 = 0.85;
const USAGE_COLOR: Vec3 = Vec3::new(0.32, 0.68, 1.0);
const MEMORY_COLOR: Vec3 = Vec3::new(0.78, 0.3, 1.0);
const TEMPERATURE_COLOR: Vec3 = Vec3::new(1.0, 0.5, 0.12);
const MUTED_COLOR: Vec3 = Vec3::new(1.0, 0.24, 0.3);
const HISTORY_END: usize = STATUS_HISTORY_SAMPLES - 1;
const HISTORY_STEP: f32 = CHART_SIZE.x * 2.0 / HISTORY_END as f32;

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
    let phase = (time * 0.18).fract();
    let alternate = (phase + 0.5).fract();
    let blend = (phase * PI).sin();
    let blend = blend * blend;
    let cloud = |phase, offset| cloud_mass(point * (1.0 - phase * 0.28) + offset, 4.0, 0.0);
    let cloud =
        cloud(phase, Vec2::ZERO) * blend + cloud(alternate, vec2(19.0, -11.0)) * (1.0 - blend);
    let envelope = smoothstep(-0.5, 1.5, outward) * smoothstep(14.0, 2.0, outward);
    vec2(
        envelope * (0.18 + cloud * 0.34),
        envelope * smoothstep(0.3, 0.62, cloud),
    ) * smoothstep(62.0, 84.0, temperature)
}

fn processor_pin(point: Vec2, boundary: Vec2, normal: Vec2) -> f32 {
    let tangent = vec2(-normal.y, normal.x);
    let local = point - boundary - normal * 0.9;
    sd_rounded_box(
        vec2(local.dot(tangent), local.dot(normal)),
        vec2(1.55, 2.05),
        0.65,
    )
}

fn cpu_pin_distance(point: Vec2) -> f32 {
    const SIZE: f32 = 13.0;
    let point = point.abs();
    let x = (point.x / 9.0).round().min(2.0) * 9.0;
    let curve_x = (x - SIZE).max(0.0);
    let curve_y = (SIZE * SIZE - curve_x * curve_x).sqrt();
    let long_edge = processor_pin(point, vec2(x, curve_y), vec2(curve_x, curve_y) / SIZE);
    let y = (point.y / 8.0).round().min(1.0) * 8.0;
    let curve_x = (SIZE * SIZE - y * y).sqrt();
    let end_cap = processor_pin(point, vec2(SIZE + curve_x, y), vec2(curve_x, y) / SIZE);
    long_edge.min(end_cap)
}

fn history_curve(
    point: Vec2,
    history: &UsageHistory,
    color: Vec3,
    fill_strength: f32,
    scroll: f32,
    inside: f32,
) -> Vec3 {
    let sample = ((point.x + CHART_SIZE.x) / HISTORY_STEP + scroll).clamp(0.0, HISTORY_END as f32);
    let index = sample.floor() as usize;
    let height = |index: usize| CHART_SIZE.y * (1.0 - history.get(index.min(HISTORY_END)) * 2.0);
    let previous = if index > 0 { index - 1 } else { 0 };
    let start = (height(previous) + height(index) * 2.0 + height(index + 1)) * 0.25;
    let end = (height(index) + height(index + 1) * 2.0 + height(index + 2)) * 0.25;
    let phase = sample.fract();
    let progress = smoothstep(0.0, 1.0, phase);
    let graph_y = start + (end - start) * progress;
    let slope = (end - start) * 6.0 * phase * (1.0 - phase) / HISTORY_STEP;
    let line = stroke(
        (point.y - graph_y).abs() / (1.0 + slope * slope).sqrt(),
        CHART_LINE_WIDTH,
    );
    color * inside * (fill(graph_y - point.y) * fill_strength + line)
}

fn processor_monitor(
    point: Vec2,
    processor: &ProcessorStatus,
    scroll: f32,
    background: Vec3,
    cpu: bool,
) -> Vec3 {
    let capsule = sd_capsule_box(point, 13.0, 13.0);
    let (pins, pin_alpha) = if cpu {
        (cpu_pin_distance(point), 1.0)
    } else {
        (1_000.0, 0.0)
    };
    let shape = crate::smooth_union(capsule, pins, 1.6, pin_alpha);
    let chart = fill(sd_capsule_box(
        point,
        CHART_SIZE.x - CHART_SIZE.y,
        CHART_SIZE.y,
    ));
    let graphs = history_curve(point, &processor.usage, USAGE_COLOR, 0.13, scroll, chart)
        + history_curve(point, &processor.memory, MEMORY_COLOR, 0.07, scroll, chart)
        + history_curve(
            point,
            &processor.temperature_history,
            TEMPERATURE_COLOR,
            0.1,
            scroll,
            chart,
        );
    let grid = (((point + CHART_SIZE) / vec2(7.0, 6.1)).fract() - 0.5).abs();
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

fn battery_icon(point: Vec2, time: f32, pill: &StatusPill) -> Vec3 {
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
        .lerp(
            vec3(0.22, 0.95, 0.55),
            smoothstep(0.18, 0.72, battery_level),
        );

    let cell_size = vec2(4.5, 6.0);
    let bubble_field = (point + vec2(0.0, time * 6.0)) / cell_size;
    let random = hash(bubble_field.floor());
    let bubble_point =
        bubble_field.fract() * cell_size - cell_size * 0.5 - vec2((random.x - 0.5) * 1.2, 0.0);
    let distance = bubble_point.length() - (0.75 + random.x * 0.55);
    let bubble = vec2(stroke(distance, 0.58), smoothstep(2.5, 0.2, distance))
        * smoothstep(0.58, 0.72, random.y)
        * liquid
        * charging;
    Vec3::splat(shell * 0.43 + terminal * 0.38)
        + liquid_color * liquid * 0.78
        + liquid_color.lerp(Vec3::ONE, 0.72) * (bubble.x * 0.9 + bubble.y * 0.16)
}

fn audio_icon(point: Vec2, time: f32, pill: &StatusPill) -> Vec3 {
    let volume = pill.volume;
    let muted = pill.muted;
    let active = pill.audio_activity * (1.0 - muted);
    let bar = ((point.x + 12.0) / 4.0).round().clamp(0.0, 6.0);
    let envelope = 1.0 - (bar - 3.0).abs() * 0.16;
    let pulse = ((time * (3.2 + bar * 0.17) + bar * 1.71).sin() * 0.5 + 0.5)
        * ((time * 1.37 - bar * 0.83).sin() * 0.18 + 0.82);
    let height = 1.2 + (2.2 + envelope * 5.5) * (0.46 + pulse * 0.54) * active;
    let distance = sd_rounded_box(
        point - vec2(-12.0 + bar * 4.0, -1.5),
        vec2(1.25, height),
        1.25,
    );
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
    let ease = charge * charge * (3.0 - 2.0 * charge);
    let pulse = (time * 8.0).sin() * charge * (1.0 - charge) * 0.16;
    let radius = 7.5 - charge * 4.6 + pulse;
    let ring_shape = stroke(
        (point - vec2(0.0, 0.65 * (1.0 - charge))).length() - radius,
        1.05 + ease * 0.7,
    );
    let gap_width = 3.1 * (1.0 - charge);
    let top_gap = smoothstep(gap_width + 0.8, gap_width - 0.2, point.x.abs())
        * smoothstep(-1.0, -5.0, point.y)
        * (1.0 - charge);
    let stem = fill_box(
        point - vec2(0.0, -5.0 + charge * 3.5),
        vec2(1.05 + ease * 0.45, 4.6 - charge * 3.0),
        0.7,
    );
    (ring_shape * (1.0 - top_gap)).max(stem)
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
    let hover = smoothstep(0.0, 1.0, hover);
    let point = point / (1.0 + hover * 0.07);
    let selected = smoothstep(0.4, 0.05, (pill.power_action - action - 1.0).abs());
    let charge = pill.power_progress * selected;
    let icon = if action < 0.5 {
        power_icon(point, time, charge + hover * (1.0 - charge) * 0.1)
    } else {
        reboot_icon(point, 1.0 - selected + charge)
    };
    let color = vec3(1.0, 0.24, 0.2).lerp(vec3(0.24, 0.78, 1.0), action);
    let backplate = fill(point.length() - (10.5 + hover * 1.5));
    let shimmer = 0.9 + (time * 8.0 + charge * 19.0).sin() * selected * 0.1;
    Vec3::splat(backplate * hover * 0.055 + icon * (0.48 + hover * 0.1))
        + color * backplate * hover * 0.075
        + color * icon * (hover * 0.22 + selected * (0.2 + charge * 0.55)) * shimmer
}

fn status_sections(
    local: Vec2,
    size: Vec2,
    pill: &StatusPill,
    background: Vec3,
    global: &GlobalUniforms,
) -> Vec3 {
    let time = global.time;
    let scroll = ((time - pill.sample_time) / 0.5).saturate();
    let layout = pill.layout();
    let section = layout.section(local.x);
    let section_center = |section| vec2(layout.center(section), size.y * 0.5);
    let center = section_center(section);
    let point = local - center;
    let smoke = thermal_smoke(local - section_center(0), time, pill.cpu.temperature).max(
        thermal_smoke(local - section_center(1), time, pill.gpu.temperature),
    );
    let smoke_color = vec3(0.07, 0.12, 0.18).lerp(
        heat_color(pill.cpu.temperature.max(pill.gpu.temperature)),
        0.24 + smoke.y * 0.12,
    );
    let background = background
        .lerp(vec3(0.002, 0.006, 0.012), smoke.x * 0.46)
        .lerp(smoke_color, smoke.y * 0.64);
    match section {
        0 => processor_monitor(point, &pill.cpu, scroll, background, true),
        1 => processor_monitor(point, &pill.gpu, scroll, background, false),
        2 => background + battery_icon(point, time, pill),
        3 => background + audio_icon(point, time, pill),
        _ => {
            let mouse = global.mouse_pos - vec2(pill.x, global.bar_height.x);
            let hover =
                smoothstep(20.0, 4.0, (mouse - center).length()) * global.mouse_pressure.saturate();
            background + action_icon(point, time, (5 - section) as f32, hover, pill)
        }
    }
}

#[spirv(vertex)]
pub fn vs_status(
    #[spirv(vertex_index)] vertex: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] status: &[StatusPill],
    #[spirv(position)] out_pos: &mut Vec4,
    #[spirv(location = 0)] out_pixel: &mut Vec2,
) {
    let pill = status[0];
    (*out_pos, *out_pixel) = pill_vertex(vertex, global, pill.x, vec2(pill.width, 0.0));
}

#[spirv(fragment)]
pub fn fs_status(
    #[spirv(location = 0)] pixel: Vec2,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] status: &[StatusPill],
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = status[0];
    let (interaction, local, size, dist) = pill_fragment(pixel, global, pill.x, pill.width);
    let (dist, mask, alpha) = interaction.surface(dist);
    if alpha <= 0.0 {
        kill();
    }
    let (background, refracted) = sky_background(
        global,
        interaction,
        local,
        size,
        dist,
        pill.sun[1],
        pill.conditions,
    );
    let color = status_sections(refracted, size, &pill, background, global)
        .lerp(Vec3::splat(0.95), interaction.ripple_flash * 0.35);
    *out_color = (color * mask).extend(alpha);
}
