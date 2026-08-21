use crate::render::{
    FrameData, PANEL_START,
    shader::{pixel_to_ndc, quad_coord},
    smoothstep, track,
};
use core::f32::consts::TAU;
use isthmus::{
    Unorm8x4,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv_std::arch::kill,
};

pub const PARTICLE_COUNT: usize = 64;
const EMISSION: f32 = 20.0;
const VELOCITY_Y: f32 = 5.0;
const LIFETIME_START: f32 = 1.2;
const LIFETIME_END: f32 = 1.5;

#[isthmus::system]
pub struct Particles {
    instances: isthmus::Instances<Particle>,
    accumulator: f32,
}

#[isthmus::data]
#[derive(Default)]
pub struct Particle {
    pub spawn_pos: Vec2,
    pub spawn_vel: Vec2,
    pub end_time: f32,
    pub duration: f32,
    pub rgb: Unorm8x4,
}

#[isthmus::host]
mod host {
    use super::{EMISSION, LIFETIME_END, LIFETIME_START, PANEL_START, PARTICLE_COUNT, Particle, Particles, TAU, Unorm8x4, VELOCITY_Y, Vec2, particle_shader, track, vec2, vec3};
    use crate::{
        app::music::PlaybackState,
        render::cpu::{Frame, Passes},
    };

    impl Particles {
        pub fn new(passes: &Passes<'_>) -> Self {
            Self {
                instances: particle_shader::instances(passes, (), [Particle::default(); PARTICLE_COUNT]),
                accumulator: 0.0,
            }
        }

        pub fn update(&mut self, track: &track::TrackRenderer, playback: &PlaybackState, frame: &mut Frame<'_>) {
            let time = frame.shared.time;
            if let Some(palette) = track.current_track_palette {
                let movement = playback.timeline.movement;
                self.accumulator = if movement.abs() > 0.00001 {
                    self.accumulator + frame.delta_time * EMISSION
                } else {
                    0.0
                };
                let emit_count = self.accumulator.floor() as u8;
                self.accumulator -= f32::from(emit_count);
                let horizontal_bias = (movement.abs().powf(0.2) * movement.signum()).clamp(-3.0, 3.0);

                for particle in self.expired(time).take(emit_count as usize) {
                    let y_fraction = fastrand::f32();

                    particle.spawn_pos = vec2(frame.shared.playhead_x, PANEL_START + frame.config.height * (0.1 + y_fraction * 0.85));
                    particle.spawn_vel = vec2(fastrand::usize(40..60) as f32 * horizontal_bias, (y_fraction - 0.5) * 2.0 * VELOCITY_Y);
                    particle.duration = LIFETIME_START + (LIFETIME_END - LIFETIME_START) * fastrand::f32();
                    particle.rgb = palette[fastrand::usize(0..palette.len())];
                    particle.end_time = time + particle.duration;
                }
            }
            if let Some(pointer) = frame.interaction.take_rate_burst() {
                for particle in self.expired(time).take(20) {
                    particle.duration = 0.5 + fastrand::f32();
                    particle.spawn_pos = pointer;
                    particle.spawn_vel = Vec2::from_angle(fastrand::f32() * TAU) * (30.0 + fastrand::f32() * 20.0);
                    particle.rgb = Unorm8x4::from_vec3(vec3(1.0, 0.843, 0.196));
                    particle.end_time = time + particle.duration;
                }
            }
        }

        fn expired(&mut self, time: f32) -> impl Iterator<Item = &mut Particle> {
            self.instances.iter_mut().filter(move |particle| time > particle.end_time)
        }
    }
}

#[isthmus::shader]
pub mod particle_shader {
    pub fn vertex(vertex: u32, frame: FrameData, particle: Particle) -> Raster {
        let dt = frame.time - (particle.end_time - particle.duration);

        if dt < 0.0 || dt > particle.duration {
            return Raster {
                position: Vec4::ZERO,
                color: Vec4::ZERO,
                uv: Vec2::ZERO,
            };
        }

        let p_life = dt / particle.duration;
        let dir = particle.spawn_vel.normalize_or_zero();
        let uv = quad_coord(vertex) * 2.0 - 1.0;
        let extent = uv * vec2(5.0, 2.5) * (p_life + 0.5);
        let world_pos = particle.spawn_pos + particle.spawn_vel * dt + dir * extent.x + dir.perp() * extent.y;
        let rgb = particle.rgb.to_vec3();
        let luma = rgb.dot(vec3(0.299, 0.587, 0.114));
        let spark_color = Vec3::splat(luma).lerp(rgb, 2.0).lerp(Vec3::ONE, 0.2) * 2.0;

        Raster {
            position: pixel_to_ndc(world_pos, frame.screen_size),
            color: spark_color.extend((1.0 - p_life) * smoothstep(0.0, 0.15, dt) * 0.3),
            uv,
        }
    }

    pub fn fragment(color: Vec4, uv: Vec2) -> Vec4 {
        let alpha = color.w * smoothstep(1.0, 0.2, (uv * vec2(0.8, 1.0)).length());
        if alpha <= 0.0 {
            kill();
        }
        (color.truncate() * alpha).extend(alpha)
    }
}
