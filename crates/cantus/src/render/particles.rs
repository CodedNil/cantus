use crate::render::{
    shader::{direction_and_length, pixel_to_ndc, quad_coord},
    shared::{FrameData, smoothstep},
};
use isthmus::glam::{Vec2, Vec3, Vec4, vec2, vec3};
use spirv_std::arch::kill;

#[cfg(feature = "cpu")]
use {
    crate::{
        PANEL_START, PARTICLE_COUNT,
        render::{Frame, Passes, track},
    },
    core::f32::consts::TAU,
};

#[isthmus::pass]
pub struct ParticlePass {
    pass: isthmus::Pass<Self>,
    accumulator: f32,
}

#[isthmus::data]
#[derive(Default)]
pub struct Particle {
    pub spawn_pos: Vec2,
    pub spawn_vel: Vec2,
    pub end_time: f32,
    pub duration: f32,
    pub rgb: isthmus::Unorm8x4,
}

#[derive(isthmus::Varyings)]
pub struct Varyings {
    pub color: Vec4,
    pub uv: Vec2,
}

#[isthmus::pass]
impl ParticlePass {
    pub fn new(passes: &Passes<'_>) -> Self {
        Self {
            pass: passes.with_instances((), [Particle::default(); PARTICLE_COUNT]),
            accumulator: 0.0,
        }
    }

    pub fn update(&mut self, track: &track::TrackPass, frame: &mut Frame<'_>) {
        const EMISSION: f32 = 20.0;
        const VELOCITY_Y: f32 = 5.0;
        const LIFETIME_START: f32 = 1.2;
        const LIFETIME_END: f32 = 1.5;

        let time = frame.shared.time;
        if let Some(palette) = track.current_track_palette {
            let movement_speed = track.movement_speed;
            self.accumulator = if movement_speed.abs() > 0.00001 {
                self.accumulator + frame.delta_time * EMISSION
            } else {
                0.0
            };
            let emit_count = self.accumulator.floor() as u8;
            self.accumulator -= f32::from(emit_count);
            let horizontal_bias =
                (movement_speed.abs().powf(0.2) * movement_speed.signum()).clamp(-3.0, 3.0);

            for particle in self.expired(time).take(emit_count as usize) {
                let y_fraction = fastrand::f32();

                particle.spawn_pos = vec2(
                    frame.shared.playhead_x,
                    PANEL_START + frame.config.height * (0.1 + y_fraction * 0.85),
                );
                particle.spawn_vel = vec2(
                    fastrand::usize(40..60) as f32 * horizontal_bias,
                    (y_fraction - 0.5) * 2.0 * VELOCITY_Y,
                );
                particle.duration = LIFETIME_START + (LIFETIME_END - LIFETIME_START) * fastrand::f32();
                particle.rgb = palette[fastrand::usize(0..palette.len())].rgb;
                particle.end_time = time + particle.duration;
            }
        }
        if let Some(pointer) = frame.interaction.take_rate_burst() {
            for particle in self.expired(time).take(20) {
                particle.duration = 0.5 + fastrand::f32();
                particle.spawn_pos = pointer;
                particle.spawn_vel =
                    Vec2::from_angle(fastrand::f32() * TAU) * (30.0 + fastrand::f32() * 20.0);
                particle.rgb = isthmus::Unorm8x4::from_rgb(vec3(1.0, 0.843, 0.196));
                particle.end_time = time + particle.duration;
            }
        }
    }

    fn expired(&mut self, time: f32) -> impl Iterator<Item = &mut Particle> {
        self.pass
            .instances
            .iter_mut()
            .filter(move |particle| time > particle.end_time)
    }

    #[gpu]
    pub fn vertex(
        #[gpu(vertex_index)] vertex: u32,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] particle: Particle,
    ) -> isthmus::Vertex<Varyings> {
        let dt = frame.time - (particle.end_time - particle.duration);

        if dt < 0.0 || dt > particle.duration {
            return isthmus::Vertex {
                position: Vec4::ZERO,
                varyings: Varyings {
                    color: Vec4::ZERO,
                    uv: Vec2::ZERO,
                },
            };
        }

        let p_life = dt / particle.duration;
        let (dir, _) = direction_and_length(particle.spawn_vel);
        let uv = quad_coord(vertex) * 2.0 - 1.0;
        let extent = uv * vec2(5.0, 2.5) * (p_life + 0.5);
        let world_pos =
            particle.spawn_pos + particle.spawn_vel * dt + dir * extent.x + dir.perp() * extent.y;
        let rgb = particle.rgb.rgb();
        let luma = rgb.dot(vec3(0.299, 0.587, 0.114));
        let spark_color = Vec3::splat(luma).lerp(rgb, 2.0).lerp(Vec3::ONE, 0.2) * 2.0;

        isthmus::Vertex {
            position: pixel_to_ndc(world_pos, frame.screen_size),
            varyings: Varyings {
                color: spark_color.extend((1.0 - p_life) * smoothstep(0.0, 0.15, dt) * 0.3),
                uv,
            },
        }
    }

    #[gpu]
    pub fn fragment(input: Varyings) -> Vec4 {
        let alpha = input.color.w * smoothstep(1.0, 0.2, (input.uv * vec2(0.8, 1.0)).length());
        if alpha <= 0.0 {
            kill();
        }
        (input.color.truncate() * alpha).extend(alpha)
    }
}
