use crate::{pill_fragment, pill_sheen, pill_vertex, sd_capsule_box, sd_rounded_box};
use cantus_shared::{
    GlobalUniforms, ProcessorStatus, STATUS_HISTORY_SAMPLES, StatusLayout, StatusPill,
    UsageHistory, smoothstep,
};
use core::f32::consts::TAU;
use spirv_std::{
    arch::kill,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

const CHART_SIZE: Vec2 = Vec2::new(21.0, 9.2);

fn fill_box(point: Vec2, half_size: Vec2, radius: f32) -> f32 {
    fill(sd_rounded_box(point, half_size, radius))
}

fn fill_capsule(point: Vec2, half_size: Vec2) -> f32 {
    fill(sd_capsule_box(
        point,
        half_size.x - half_size.y,
        half_size.y,
    ))
}

fn fill(distance: f32) -> f32 {
    smoothstep(0.8, -0.8, distance)
}

fn ring(point: Vec2, radius: f32, width: f32) -> f32 {
    stroke(point.length() - radius, width)
}

fn stroke(distance: f32, width: f32) -> f32 {
    smoothstep(width + 0.7, width - 0.7, distance.abs())
}

fn heat_color(temperature: f32) -> Vec3 {
    let warm = smoothstep(60.0, 72.0, temperature);
    let hot = smoothstep(72.0, 88.0, temperature);
    vec3(0.22, 0.62, 1.0)
        .lerp(vec3(1.0, 0.38, 0.08), warm)
        .lerp(vec3(1.0, 0.08, 0.035), hot)
}

fn thermal_wisps(point: Vec2, time: f32, temperature: f32) -> Vec3 {
    let amount = smoothstep(66.0, 90.0, temperature);
    let altitude = -point.y - 13.0;
    let vertical_fade = smoothstep(0.0, 2.0, altitude) * smoothstep(15.0, 8.0, altitude);
    let mut vapor: f32 = 0.0;
    #[allow(clippy::needless_range_loop)]
    for index in 0..3 {
        let seed = index as f32;
        let travel = (time * (0.19 + seed * 0.018) + seed * 0.37).fract() * 14.0;
        let source_x = (seed - 1.0) * 13.0;
        let curl = (altitude * 0.42 + time * 1.1 + seed * 2.3).sin() * 1.8
            + (altitude * 0.16 - time * 0.7 + seed).sin() * 0.7;
        let strand = smoothstep(2.0, 0.25, (point.x - source_x - curl).abs());
        let packet = smoothstep(5.5, 0.0, (altitude - travel).abs());
        let wisp = strand * packet * vertical_fade;
        vapor = vapor.max(wisp);
    }
    let color = heat_color(temperature).lerp(Vec3::ONE, 0.58);
    color * vapor * amount * 0.22
}

fn history_curve(
    point: Vec2,
    history: &UsageHistory,
    color: Vec3,
    fill_strength: f32,
    scroll: f32,
) -> Vec3 {
    let inside = fill_capsule(point, CHART_SIZE);
    let u = ((point.x + CHART_SIZE.x) / (CHART_SIZE.x * 2.0)).clamp(0.0, 0.999);
    let sample =
        (u * (STATUS_HISTORY_SAMPLES - 1) as f32 + scroll).min((STATUS_HISTORY_SAMPLES - 1) as f32);
    let index = sample.floor() as usize;
    let p1 = smooth_history_sample(history, index);
    let p2 = smooth_history_sample(history, (index + 1).min(STATUS_HISTORY_SAMPLES - 1));
    let t = sample.fract();
    let value = p1 + (p2 - p1) * smoothstep(0.0, 1.0, t);
    let derivative = (p2 - p1) * 6.0 * t * (1.0 - t);
    let graph_y = CHART_SIZE.y - value.clamp(0.0, 1.0) * CHART_SIZE.y * 2.0;
    let slope = -CHART_SIZE.y * derivative * (STATUS_HISTORY_SAMPLES - 1) as f32 / CHART_SIZE.x;
    let line_distance = (point.y - graph_y).abs() / (1.0 + slope * slope).sqrt();
    let glow = inside * smoothstep(3.4, 0.2, line_distance);
    let line = inside * smoothstep(1.25, 0.18, line_distance);
    let area = inside * smoothstep(graph_y - 0.6, graph_y + 0.6, point.y);
    color * (area * fill_strength + glow * 0.16 + line * 1.08)
}

fn smooth_history_sample(history: &UsageHistory, index: usize) -> f32 {
    (history.get(if index > 0 { index - 1 } else { 0 })
        + history.get(index) * 2.0
        + history.get((index + 1).min(STATUS_HISTORY_SAMPLES - 1)))
        * 0.25
}

fn grid_line(value: f32, offset: f32, spacing: f32, edge: f32) -> f32 {
    smoothstep(
        0.49,
        edge,
        (((value + offset) / spacing).fract() - 0.5).abs(),
    )
}

fn processor_monitor(
    point: Vec2,
    time: f32,
    processor: &ProcessorStatus,
    scroll: f32,
    background: Vec3,
) -> Vec3 {
    let body_distance = sd_capsule_box(point, 13.0, 13.0);
    let body = fill(body_distance);
    let hardware = stroke(body_distance, 1.55);
    let usage_color = vec3(0.32, 0.68, 1.0);
    let usage = history_curve(point, &processor.usage, usage_color, 0.13, scroll);
    let memory = history_curve(point, &processor.memory, vec3(0.78, 0.3, 1.0), 0.07, scroll);
    let chart = fill_capsule(point, CHART_SIZE);
    let grid_x = grid_line(point.x, CHART_SIZE.x, 7.0, 0.46);
    let grid_y = grid_line(point.y, CHART_SIZE.y, 6.1, 0.45);
    let grid = chart * grid_x.max(grid_y) * 0.045;
    let glass = body * 0.82;
    let load = processor.usage.get(STATUS_HISTORY_SAMPLES - 1);
    let thermal = smoothstep(60.0, 86.0, processor.temperature);
    let frame_color = vec3(0.025, 0.09, 0.15)
        .lerp(usage_color, 0.18 + load * 0.24)
        .lerp(heat_color(processor.temperature), thermal * 0.9);
    background
        .lerp(vec3(0.004, 0.012, 0.026), glass)
        .lerp(frame_color, hardware * 0.92)
        + Vec3::splat(grid)
        + usage
        + memory
        + thermal_wisps(point, time, processor.temperature)
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
    let level = 12.0 - battery_level.clamp(0.0, 1.0) * 24.0;
    let wave = (point.x * 0.62 + time * (1.4 + charging * 1.2)).sin() * 1.15
        + (point.x * 0.27 - time * 0.8).sin() * 0.45;
    let liquid = inside * smoothstep(level + wave - 0.7, level + wave + 0.7, point.y - 1.0);
    let low = smoothstep(0.28, 0.08, battery_level);
    let full = smoothstep(0.18, 0.72, battery_level);
    let liquid_color = vec3(1.0, 0.18, 0.10)
        .lerp(vec3(1.0, 0.72, 0.12), 1.0 - low)
        .lerp(vec3(0.22, 0.95, 0.55), full);

    let mut bubbles: f32 = 0.0;
    #[allow(clippy::needless_range_loop)]
    for index in 0..3 {
        let seed = index as f32 * 0.31;
        let rise = (time * 0.32 + seed).fract();
        let bubble_point = point - vec2(-5.0 + index as f32 * 5.0, 11.0 - rise * 20.0);
        bubbles = bubbles.max(ring(bubble_point, 0.8 + seed, 0.35));
    }
    Vec3::splat(shell * 0.43 + terminal * 0.38)
        + liquid_color * liquid * 0.78
        + Vec3::splat(bubbles * charging * 0.32)
}

fn audio_icon(point: Vec2, time: f32, pill: &StatusPill) -> Vec3 {
    let volume = pill.volume;
    let muted = pill.muted;
    let active = pill.audio_activity * (1.0 - muted);
    let mut bars: f32 = 0.0;
    let mut glow: f32 = 0.0;
    #[allow(clippy::needless_range_loop)]
    for index in 0..7 {
        let seed = index as f32;
        let x = -12.0 + seed * 4.0;
        let envelope = 1.0 - (seed - 3.0).abs() * 0.16;
        let pulse = ((time * (3.2 + seed * 0.17) + seed * 1.71).sin() * 0.5 + 0.5)
            * ((time * 1.37 - seed * 0.83).sin() * 0.18 + 0.82);
        let height = 1.2 + volume * (2.2 + envelope * 5.5) * (0.46 + pulse * 0.54) * active;
        let bar_point = point - vec2(x, -1.5);
        let distance = sd_rounded_box(bar_point, vec2(1.25, height), 1.25);
        bars = bars.max(smoothstep(0.7, -0.7, distance));
        glow = glow.max(smoothstep(3.2, 0.0, distance));
    }

    let rail_point = point - vec2(0.0, 11.5);
    let rail = fill_box(rail_point, vec2(14.0, 1.25), 1.25);
    let level_edge = -14.0 + volume.clamp(0.0, 1.0) * 28.0;
    let level = rail * smoothstep(level_edge + 0.8, level_edge - 0.8, rail_point.x);
    let idle = rail * (1.0 - level) * 0.22;

    let diagonal = vec2(point.x + point.y, point.y - point.x) * 0.707;
    let cross_a = fill_box(diagonal, vec2(8.0, 0.85), 0.85);
    let cross_b = fill_box(vec2(-diagonal.y, diagonal.x), vec2(8.0, 0.85), 0.85);
    let audio_color = vec3(0.08, 0.88, 1.0).lerp(vec3(0.65, 0.34, 1.0), volume * 0.65);
    audio_color * (bars * (0.58 + active * 0.35) + glow * active * 0.12 + level)
        + Vec3::splat(idle)
        + vec3(1.0, 0.24, 0.3) * cross_a.max(cross_b) * muted * 0.85
}

fn arrow_segment(point: Vec2, end: Vec2) -> f32 {
    let along = (point.dot(end) / end.dot(end)).clamp(0.0, 1.0);
    smoothstep(0.7, -0.7, (point - end * along).length() - 1.0)
}

fn power_icon(point: Vec2, time: f32, charge: f32) -> f32 {
    let ease = charge * charge * (3.0 - 2.0 * charge);
    let pulse = (time * 8.0).sin() * charge * (1.0 - charge) * 0.16;
    let radius = 7.5 - charge * 4.6 + pulse;
    let ring_shape = ring(
        point - vec2(0.0, 0.65 * (1.0 - charge)),
        radius,
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
    const SWEEP_FRACTION: f32 = 0.82;
    const SWEEP: f32 = TAU * SWEEP_FRACTION;
    const ARROW_CLEARANCE: f32 = 0.045;

    let phase = ((point.y.atan2(point.x) - START) / TAU + 1.0).fract();
    let arc_end = (progress * SWEEP_FRACTION - ARROW_CLEARANCE).max(0.0);
    let revealed =
        smoothstep(arc_end + 0.008, arc_end - 0.008, phase) * smoothstep(0.0, 0.02, progress);
    let arc = ring(point, 7.1, 1.05) * revealed;

    let angle = START + SWEEP * progress;
    let direction = vec2(angle.cos(), angle.sin());
    let tangent = vec2(-direction.y, direction.x);
    let arrow_point = point - direction * 7.1;
    let arrow_point = vec2(arrow_point.dot(tangent), arrow_point.dot(direction));
    let arrow = arrow_segment(arrow_point, vec2(-3.2, -2.1))
        .max(arrow_segment(arrow_point, vec2(-3.2, 2.1)));
    arc.max(arrow)
}

fn action_icon(point: Vec2, time: f32, action: f32, hover: f32, pill: &StatusPill) -> Vec3 {
    let selected = smoothstep(0.4, 0.05, (pill.power_action - action - 1.0).abs());
    let charge = pill.power_progress * selected;
    let icon = if action < 0.5 {
        let motion = charge + hover * (1.0 - charge) * 0.1;
        power_icon(point, time, motion)
    } else {
        reboot_icon(point, 1.0 - selected + charge)
    };
    let color = vec3(1.0, 0.24, 0.2).lerp(vec3(0.24, 0.78, 1.0), action);
    let shimmer = 0.9 + (time * 8.0 + charge * 19.0).sin() * selected * 0.1;
    Vec3::splat(icon * 0.48)
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
    let scroll = ((time - pill.sample_time) / 0.5).clamp(0.0, 1.0);
    let layout = StatusLayout::new(pill.battery_present > 0.5);
    let section = layout.section(local.x);
    let center = vec2(layout.center(section), size.y * 0.5);
    let point = local - center;
    match section {
        0 => processor_monitor(point, time, &pill.cpu, scroll, background),
        1 => processor_monitor(point, time, &pill.gpu, scroll, background),
        2 => background + battery_icon(point, time, pill),
        3 => background + audio_icon(point, time, pill),
        _ => {
            let mouse = global.mouse_pos - vec2(pill.x, global.bar_height.x);
            let hover = smoothstep(20.0, 4.0, (mouse - center).length())
                * global.mouse_pressure.clamp(0.0, 1.0);
            background + action_icon(point, time, (5 - section) as f32, hover, pill)
        }
    }
}

#[spirv(vertex)]
pub fn vs_status(
    #[spirv(vertex_index)] vertex: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
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
    #[spirv(uniform, descriptor_set = 0, binding = 0)] global: &GlobalUniforms,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] status: &[StatusPill],
    #[spirv(location = 0)] out_color: &mut Vec4,
) {
    let pill = status[0];
    let (interaction, local, size, dist) = pill_fragment(pixel, global, pill.x, pill.width);
    let (dist, mask, alpha) = interaction.surface(dist);
    if alpha <= 0.0 {
        kill();
    }
    let refracted = interaction.refract(local, size, dist);
    let edge = (local.x / pill.width - 0.5).abs() * 2.0;
    let background = crate::weather::scene(
        refracted * size,
        size,
        size.y,
        pill.sun,
        crate::weather::forecast(pill.conditions, edge),
        global.time,
        0.0,
    ) + pill_sheen(refracted.y, dist, interaction);
    let color = status_sections(refracted * size, size, &pill, background, global)
        .lerp(Vec3::splat(0.95), interaction.ripple_flash * 0.35);
    *out_color = (color * mask).extend(alpha);
}
