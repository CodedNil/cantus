use crate::render::{
    Fragment, GAP, PADDING, PANEL_START, TextFragment,
    sdf::{VISIBLE_ALPHA, fill, fill_rounded_box, hash, pill_margin, sample_pill, sd_capsule_box, sd_chevron, sd_rounded_box, segment_distance, smooth_union, stroke},
    tempestas::{StatusSky, scene, sky_phase},
};
use core::f32::consts::TAU;
use isthmus::{
    FloatExt, Quad, ShaderData,
    glam::{Vec2, Vec3, vec2, vec3},
    rgba,
    spirv_std::arch::kill,
    text::TextStyle,
};

const STATUS_HISTORY_SAMPLES: usize = 32;
pub const AUDIO_SPECTRUM_BANDS: usize = 7;
const DATA_WIDTH: f32 = 32.0;
const ACTION_WIDTH: f32 = 24.0;
/// CPU/GPU graphs stay this wide regardless of whether the battery slot is present.
const GRAPH_WIDTH: f32 = 60.0 + f32::midpoint(DATA_WIDTH, GAP);
const CHART_LINE_WIDTH: f32 = 0.85;
const USAGE_COLOR: Vec3 = Vec3::new(0.32, 0.68, 1.0);
const MEMORY_COLOR: Vec3 = Vec3::new(0.78, 0.3, 1.0);
const MUTED_COLOR: Vec3 = Vec3::new(1.0, 0.24, 0.3);
const LABEL_STYLE: TextStyle = TextStyle::new(11.0, 700.0);
const HISTORY_END: usize = STATUS_HISTORY_SAMPLES - 1;
const BASE_WIDTH: f32 = PADDING * 2.0 + GRAPH_WIDTH * 2.0 + DATA_WIDTH + ACTION_WIDTH * 2.0 + GAP * 4.0;

#[repr(C)]
#[derive(Clone, Copy, Default, ShaderData)]
pub struct ProcessorStatus {
    pub temperature: f32,
    pub usage: [f32; STATUS_HISTORY_SAMPLES],
    pub memory: [f32; STATUS_HISTORY_SAMPLES],
}

#[isthmus::paint]
mod host {
    use super::*;
    use crate::{
        app::{
            Background,
            platform::{Linux as Platform, Platform as _},
        },
        interaction::Rect,
        render::RenderContext,
    };
    use arrayvec::ArrayString;
    use std::{
        fmt::Write,
        sync::{
            Arc,
            atomic::{AtomicU32, Ordering},
            mpsc::{self, Receiver},
        },
    };

    impl ProcessorStatus {
        fn record(&mut self, sample: ProcessorSample) {
            for (history, value) in [(&mut self.usage, sample.usage), (&mut self.memory, sample.memory)] {
                history.copy_within(1.., 0);
                history[HISTORY_END] = value.saturate();
            }
        }
    }

    #[derive(Default)]
    pub struct AudioMonitor {
        pub volume: AtomicU32,
        pub spectrum: [AtomicU32; AUDIO_SPECTRUM_BANDS],
    }

    pub struct StatusPanel {
        /// Battery charge magnitude, negated while charging.
        battery_level: Option<f32>,
        /// Logarithmic frequency-band levels sampled from the system audio monitor stream.
        audio_spectrum: [f32; AUDIO_SPECTRUM_BANDS],
        /// Fractional scroll between the two newest history samples.
        history_scroll: f32,
        /// CPU temperature and usage history.
        cpu: ProcessorStatus,
        /// GPU temperature and usage history.
        gpu: ProcessorStatus,
        target_temperature: [f32; 2],
        action_hover: [f32; 2],
        audio_monitor: Arc<AudioMonitor>,
        background: Background,
        updates: Receiver<SystemSample>,
    }

    #[derive(Clone, Copy)]
    pub struct ProcessorSample {
        pub temperature: f32,
        pub usage: f32,
        pub memory: f32,
    }

    pub struct SystemSample {
        pub cpu: ProcessorSample,
        pub gpu: Option<ProcessorSample>,
        pub battery_level: Option<f32>,
    }

    impl StatusPanel {
        /// Total on-screen width of the pill; grows to include the battery slot when shown.
        pub const fn width(&self) -> f32 {
            BASE_WIDTH + if self.battery_level.is_some() { DATA_WIDTH + GAP } else { 0.0 }
        }

        pub(crate) fn new(background: &Background) -> Self {
            let audio_monitor = Arc::<AudioMonitor>::default();
            let (updates, inbox) = mpsc::channel();
            Platform::start_status_monitor(background, updates, Arc::clone(&audio_monitor));
            Self {
                battery_level: None,
                audio_spectrum: Default::default(),
                history_scroll: 0.0,
                cpu: ProcessorStatus::default(),
                gpu: ProcessorStatus::default(),
                target_temperature: [0.0; 2],
                action_hover: [0.0; 2],
                audio_monitor,
                background: background.clone(),
                updates: inbox,
            }
        }

        pub fn show(&mut self, frame: &mut RenderContext, sky: StatusSky) {
            while let Ok(update) = self.updates.try_recv() {
                self.target_temperature[0] = update.cpu.temperature;
                self.cpu.record(update.cpu);
                if let Some(gpu) = update.gpu {
                    self.target_temperature[1] = gpu.temperature;
                    self.gpu.record(gpu);
                }
                self.battery_level = update.battery_level;
                self.history_scroll = 0.0;
            }
            let mut volume = f32::from_bits(self.audio_monitor.volume.load(Ordering::Relaxed));
            let height = frame.config.height;
            let width = self.width();
            let x = frame.paint.screen_size.x - width - GAP;
            let pill = Rect::new(x, PANEL_START, x + width, PANEL_START + height);
            let pill_quad: Quad = pill.into();
            let mut cursor = PADDING;
            let section = |center: f32, section_width: f32| Rect::from_center(vec2(x + center, PANEL_START + height * 0.5), vec2(f32::midpoint(section_width, GAP), height * 0.5));

            // GPU: Status background.
            let margin = pill_margin(frame.globals(), frame.paint.time);
            frame.paint.paint_quad(pill_quad.expanded(margin), |fragment: Fragment, pill_quad: Quad, sky: StatusSky| {
                let sample = sample_pill(pill_quad, fragment.pixel, fragment.globals, fragment.time);
                if sample.alpha <= VISIBLE_ALPHA {
                    kill();
                }

                // Weather-reactive glass body and interaction flash.
                let color = scene(
                    fragment.time,
                    fragment.globals.bar_height,
                    sample.refracted,
                    sample.size.x,
                    sample.distance,
                    sky_phase(sky.sun_height),
                    sky.conditions,
                )
                .lerp(Vec3::splat(0.95), sample.ripple_flash * 0.35);
                sample.color(color)
            });

            let temperature_blend = 1.0 - (-5.0 * frame.paint.delta_time).exp();
            for (processor, target) in [&mut self.cpu, &mut self.gpu].into_iter().zip(self.target_temperature) {
                processor.temperature += (target - processor.temperature) * temperature_blend;
            }
            self.history_scroll = (self.history_scroll + frame.paint.delta_time / Platform::STATUS_SAMPLE_INTERVAL.as_secs_f32()).saturate();
            for (processor, show_pins) in [(self.cpu, true), (self.gpu, false)] {
                let center = cursor + GRAPH_WIDTH * 0.5;
                let mut label = ArrayString::<16>::new();
                write!(
                    label,
                    "{:.0}% {:.0}% {:.0}\u{b0}C",
                    processor.usage[HISTORY_END] * 100.0,
                    processor.memory[HISTORY_END] * 100.0,
                    processor.temperature,
                )
                .unwrap();
                let half_width = GRAPH_WIDTH * 0.5 - GAP * 0.5;
                let line = frame.paint.text().line(&label, LABEL_STYLE).fit(GAP + 5.0, center - half_width..center + half_width);
                // GPU: Processor monitor.
                frame.paint.paint_quad(
                    section(center, GRAPH_WIDTH),
                    |fragment: Fragment, processor: ProcessorStatus, history_scroll: f32, show_pins: bool| {
                        let half_width = GRAPH_WIDTH * 0.5 - GAP * 0.5;
                        let radius = fragment.globals.bar_height * 0.5 - GAP;
                        let capsule = sd_capsule_box(fragment.local, half_width - radius, radius);

                        // CPU package pins follow the capsule boundary; the GPU monitor omits them.
                        let pins = if show_pins {
                            let absolute = fragment.local.abs();
                            let half_span = half_width - radius;
                            let pin = |boundary: Vec2, normal: Vec2| {
                                let offset = absolute - boundary - normal * 0.9;
                                let tangent = vec2(-normal.y, normal.x);
                                sd_rounded_box(vec2(offset.dot(tangent), offset.dot(normal)), vec2(1.55, 2.05), 0.65)
                            };
                            let x = ((absolute.x / 9.0).round() * 9.0).min(half_width);
                            let curve_x = (x - half_span).max(0.0);
                            let curve_y = (radius * radius - curve_x * curve_x).sqrt();
                            let long_edge = pin(vec2(x, curve_y), vec2(curve_x, curve_y) / radius);
                            let y = (absolute.y / 8.0).round().min(1.0) * 8.0;
                            let curve_x = (radius * radius - y * y).sqrt();
                            long_edge.min(pin(vec2(half_span + curve_x, y), vec2(curve_x, y) / radius))
                        } else {
                            1_000.0
                        };

                        // Scrolling usage and memory histories.
                        let chart = fill(capsule);
                        let history_step = half_width * 2.0 / HISTORY_END as f32;
                        let sample = ((fragment.local.x + half_width) / history_step + history_scroll).clamp(0.0, HISTORY_END as f32);
                        let index = sample.floor() as usize;
                        let graph_height = radius - 2.0;
                        let curve = |history: &[f32; STATUS_HISTORY_SAMPLES], color: Vec3, fill_strength: f32| {
                            let height = |i: usize| graph_height * (1.0 - history[i.min(HISTORY_END)] * 2.0);
                            let at = |i: usize| vec2((i as f32 - history_scroll) * history_step - half_width, height(i));
                            let start = at(index);
                            let end = at(index + 1);
                            let line = stroke(segment_distance(fragment.local, start, end), CHART_LINE_WIDTH);
                            let graph_y = start.y.lerp(end.y, sample.fract().smoothstep(0.0, 1.0));
                            color * chart * (fill(graph_y - fragment.local.y) * fill_strength + line)
                        };
                        let graphs = curve(&processor.usage, USAGE_COLOR, 0.156) + curve(&processor.memory, MEMORY_COLOR, 0.084);

                        // Grid and smoothly temperature-tinted package frame.
                        let cell = (((fragment.local + vec2(half_width, radius)) / vec2(7.0, 6.1)).fract() - 0.5).abs();
                        let grid = chart * cell.x.smoothstep(0.49, 0.46).max(cell.y.smoothstep(0.49, 0.45)) * 0.045;
                        let heat = vec3(0.22, 0.62, 1.0)
                            .lerp(vec3(1.0, 0.38, 0.08), processor.temperature.smoothstep(60.0, 72.0))
                            .lerp(vec3(1.0, 0.08, 0.035), processor.temperature.smoothstep(72.0, 88.0));
                        let frame_color = vec3(0.025, 0.09, 0.15)
                            .lerp(USAGE_COLOR, 0.18 + processor.usage[HISTORY_END] * 0.24)
                            .lerp(heat, processor.temperature.smoothstep(60.0, 86.0) * 0.9);
                        let pin_visibility = if show_pins { 1.0 } else { 0.0 };
                        let coverage = fill(smooth_union(capsule, pins, 1.6, pin_visibility));
                        let hardware = stroke(capsule, 1.55).max(fill(pins) * pin_visibility);
                        let color = vec3(0.004, 0.012, 0.026).lerp(frame_color, hardware) * coverage + Vec3::splat(grid) + graphs;
                        color.extend(coverage.max(graphs.max_element().saturate()))
                    },
                );
                // GPU: Processor label.
                frame.paint.paint_text(line.translated(vec2(x, PANEL_START)), |text: TextFragment| text.color(text.alpha()));
                cursor += GRAPH_WIDTH + GAP;
            }

            if let Some(battery_level) = self.battery_level {
                let center = cursor + DATA_WIDTH * 0.5;
                // GPU: Battery indicator.
                frame.paint.paint_quad(section(center, DATA_WIDTH), |fragment: Fragment, battery_level: f32| {
                    let time = fragment.time;
                    let point = fragment.local / 0.8;

                    // Battery shell and fill boundary.
                    let charging = if battery_level < 0.0 { 1.0 } else { 0.0 };
                    let level = battery_level.abs();
                    let shell = stroke(sd_rounded_box(point - vec2(0.0, 1.0), vec2(11.5, 15.0), 3.2), 1.875);
                    let terminal = fill_rounded_box(point - vec2(0.0, -15.6), vec2(4.0, 1.8), 0.8);
                    let inside = fill_rounded_box(point - vec2(0.0, 1.0), vec2(8.5, 12.0), 1.7);
                    let surface = 12.0 - level.saturate() * 24.0;
                    let wave = (point.x * 0.62 + time * (1.4 + charging * 1.2)).sin() * 1.15 + (point.x * 0.27 - time * 0.8).sin() * 0.45;
                    let liquid = inside * (point.y - 1.0).smoothstep(surface + wave - 0.7, surface + wave + 0.7);

                    // Charge-dependent liquid color.
                    let liquid_color = vec3(1.0, 0.18, 0.10)
                        .lerp(vec3(1.0, 0.72, 0.12), level.smoothstep(0.08, 0.28))
                        .lerp(vec3(0.22, 0.95, 0.55), level.smoothstep(0.18, 0.72));

                    // Rising bubbles only while charging.
                    let column = (point.x / 3.0).floor();
                    let seed = hash(vec2(column, 0.0));
                    let cycle = (time * (0.35 + seed.y * 0.5) + seed.x * 7.0).fract();
                    let center = vec2((column + 0.2 + seed.x * 0.6) * 3.0, 13.0 - cycle * 24.0);
                    let distance = (point - center).length() - (0.4 + seed.y * 0.5);
                    let fade = cycle.smoothstep(0.0, 0.25) * cycle.smoothstep(1.0, 0.7);
                    let bubble = stroke(distance, 0.45) * fade * inside * charging;
                    let color = Vec3::splat(shell * 0.43 + terminal * 0.38) + liquid_color * liquid * 0.78 + liquid_color.lerp(Vec3::ONE, 0.72) * bubble * 0.9;
                    color.extend(shell.max(terminal).max(liquid).max(bubble))
                });
                cursor += DATA_WIDTH + GAP;
            }

            for (damped, level) in self.audio_spectrum.iter_mut().zip(&self.audio_monitor.spectrum) {
                let target = f32::from_bits(level.load(Ordering::Relaxed));
                let response = if target > *damped { 18.0 } else { 6.0 };
                *damped += (target - *damped) * (1.0 - (-response * frame.paint.delta_time).exp());
            }
            let center = cursor + DATA_WIDTH * 0.5;
            let scroll = frame.interaction.interact(section(center, DATA_WIDTH)).scroll;
            if scroll != 0 {
                let sign = volume.signum();
                volume = (volume.abs() - scroll as f32 * 0.05).saturate() * sign;
                self.audio_monitor.volume.store(volume.to_bits(), Ordering::Relaxed);
                Platform::set_volume(volume.abs());
            }
            // GPU: Audio spectrum and volume.
            frame.paint.paint_quad(
                section(center, DATA_WIDTH),
                |fragment: Fragment, audio_spectrum: [f32; AUDIO_SPECTRUM_BANDS], volume: f32| {
                    // Seven-band spectrum.
                    let muted = if volume < 0.0 { 1.0 } else { 0.0 };
                    let volume = volume.abs();
                    let bar = ((fragment.local.x + 12.0) / 4.0).round().clamp(0.0, 6.0);
                    let active = audio_spectrum[bar as usize] * (1.0 - muted);
                    let height = 1.2 + 7.7 * active;
                    let bars = sd_rounded_box(fragment.local - vec2(-12.0 + bar * 4.0, -1.5), vec2(1.25, height), 1.25);

                    // Volume rail and filled level.
                    let rail_point = fragment.local - vec2(0.0, 11.5);
                    let rail = fill_rounded_box(rail_point, vec2(14.0, 1.25), 1.25);
                    let level_x = -14.0 + volume.saturate() * 28.0;
                    let level = rail * rail_point.x.smoothstep(level_x + 0.8, level_x - 0.8);

                    let color = vec3(0.18, 0.96, 1.0);
                    let spectrum_color = color * bars.smoothstep(0.7, -0.7);
                    let level_color = color.lerp(MUTED_COLOR, muted) * (level + rail * (1.0 - level) * 0.22);
                    (spectrum_color + level_color).extend(bars.smoothstep(0.7, -0.7).max(rail))
                },
            );
            cursor += DATA_WIDTH + GAP;

            for action in [1usize, 0] {
                let center = cursor + ACTION_WIDTH * 0.5;
                let response = frame.interaction.interact(section(center, ACTION_WIDTH));
                self.action_hover[action] = self.action_hover[action].move_towards(f32::from(response.hovered()), frame.paint.delta_time / 0.12);
                if response.hovered() && response.held_for(1.5) {
                    Platform::run_power_action(&self.background, action);
                }
                let hover = self.action_hover[action];
                let selected = f32::from(response.held() && response.hovered());
                let power_progress = (response.held_seconds / 1.5).saturate();
                let reboot = action == 1;
                // GPU: Power action icon.
                frame.paint.paint_quad(
                    section(center, ACTION_WIDTH),
                    |fragment: Fragment, reboot: bool, hover: f32, selected: f32, power_progress: f32| {
                        let time = fragment.time;
                        let point = fragment.local / (1.0 + hover * 0.07);
                        let charge = power_progress * selected;

                        // Power icon morphs inward as the hold completes.
                        let icon = if reboot {
                            // Reboot icon draws a progressing circular arrow.
                            const START: f32 = TAU * 0.08;
                            const SWEEP: f32 = TAU * 0.82;
                            let progress = 1.0 - selected + charge;
                            let phase = ((point.y.atan2(point.x) - START) / TAU + 1.0).fract();
                            let arc_end = (progress * 0.82 - 0.045).max(0.0);
                            let arc = stroke(point.length() - 7.1, 1.05) * phase.smoothstep(arc_end + 0.008, arc_end - 0.008) * progress.smoothstep(0.0, 0.02);
                            let angle = START + SWEEP * progress;
                            let direction = vec2(angle.cos(), angle.sin());
                            let tangent = vec2(-direction.y, direction.x);
                            let arrow = point - direction * 7.1;
                            let arrow = vec2(arrow.dot(tangent), arrow.dot(direction));
                            arc.max((sd_chevron(arrow, vec2(-3.2, 2.1)) - 1.0).smoothstep(0.7, -0.7))
                        } else {
                            let ease = charge.smoothstep(0.0, 1.0);
                            let radius = 7.5 - charge * 4.6 + (time * 8.0).sin() * charge * (1.0 - charge) * 0.16;
                            let ring = stroke(point.length() - radius, 1.05 + ease * 0.7);
                            let gap = fill_rounded_box(point - vec2(0.0, -7.0), vec2(3.0 * (1.0 - charge), 3.0), 0.5);
                            let stem = fill_rounded_box(point - vec2(0.0, -5.0 + charge * 3.5), vec2(1.05 + ease * 0.45, 4.6 - charge * 3.0), 0.7);
                            (ring * (1.0 - gap)).max(stem)
                        };
                        let color = Vec3::splat(0.76).lerp(vec3(0.95, 0.42, 0.4), hover.max(selected * (0.5 + charge * 0.5)));
                        rgba(color * (1.0 + charge * 0.45), icon)
                    },
                );
                cursor += ACTION_WIDTH + GAP;
            }
            frame.interaction.input_region(pill);
        }
    }
}
