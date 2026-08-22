use crate::render::{Globals, PANEL_START};
use core::f32::consts::TAU;
use isthmus::{
    Quad,
    float::FloatExt,
    glam::{Vec2, Vec3, vec2, vec3},
};

#[cfg(not(target_arch = "spirv"))]
use crate::{app::music::PlaybackState, render::Frame};

pub const PARTICLE_COUNT: usize = 32;
const EMISSION: f32 = 20.0;
const VELOCITY_Y: f32 = 5.0;
const LIFETIME_START: f32 = 1.2;
const LIFETIME_END: f32 = 1.5;

#[derive(Default)]
pub struct Particles {
    particles: [Particle; PARTICLE_COUNT],
    emission_debt: f32,
}

#[isthmus::data]
#[derive(Default)]
pub struct Particle {
    pub spawn_pos: Vec2,
    pub spawn_vel: Vec2,
    pub end_time: f32,
    pub duration: f32,
    pub color: Vec3,
}

#[isthmus::paint]
impl Particles {
    pub fn show(&mut self, frame: &mut Frame, playback: &PlaybackState) {
        self.emit_playhead(playback, frame);

        let time = frame.globals.info.time;
        for &particle in self.particles.iter().filter(|particle| particle.end_time > time) {
            let dt = time - (particle.end_time - particle.duration);
            let half_size = vec2(5.0, 2.5) * (dt / particle.duration + 0.5);
            let quad = Quad::new(particle.spawn_pos + particle.spawn_vel * dt, half_size * 2.0, particle.spawn_vel.normalize_or_zero());
            // GPU: Particle sprite.
            frame.paint_quad(quad, |frame: Globals, fragment: isthmus::Fragment, particle: Particle| {
                let dt = frame.info.time - (particle.end_time - particle.duration);
                let life = dt / particle.duration;
                let centered = fragment.uv * 2.0 - 1.0;
                let shape = (centered * vec2(0.8, 1.0)).length().smoothstep(1.0, 0.2);
                let fade = (1.0 - life) * dt.smoothstep(0.0, 0.15);
                let alpha = fade * shape * 0.3;
                (particle.color * alpha).extend(alpha)
            });
        }
    }

    fn emit_playhead(&mut self, playback: &PlaybackState, frame: &Frame) {
        if let Some((index, _)) = playback.timeline.track_at_playhead(&playback.queue) {
            let palette = playback.queue[index].runtime.art.palette();
            let movement = playback.timeline.movement;
            self.emission_debt = if movement.abs() > 0.00001 {
                self.emission_debt + frame.delta_time * EMISSION
            } else {
                0.0
            };
            let emit_count = self.emission_debt.floor() as usize;
            self.emission_debt -= emit_count as f32;
            let horizontal_bias = (movement.abs().powf(0.2) * movement.signum()).clamp(-3.0, 3.0);
            for particle in self.particles.iter_mut().filter(|particle| particle.end_time <= frame.globals.info.time).take(emit_count) {
                let y_fraction = fastrand::f32();
                particle.spawn_pos = vec2(frame.globals.playhead_x, PANEL_START + frame.services.config.height * (0.1 + y_fraction * 0.85));
                particle.spawn_vel = vec2(fastrand::usize(40..60) as f32 * horizontal_bias, (y_fraction - 0.5) * 2.0 * VELOCITY_Y);
                particle.duration = LIFETIME_START + (LIFETIME_END - LIFETIME_START) * fastrand::f32();
                particle.color = spark_color(palette[fastrand::usize(0..palette.len())].to_vec3());
                particle.end_time = frame.globals.info.time + particle.duration;
            }
        }
    }

    pub fn burst(&mut self, frame: &Frame) {
        for particle in self.particles.iter_mut().filter(|particle| particle.end_time <= frame.globals.info.time).take(20) {
            particle.duration = 0.5 + fastrand::f32();
            particle.spawn_pos = frame.pointer();
            particle.spawn_vel = Vec2::from_angle(fastrand::f32() * TAU) * (30.0 + fastrand::f32() * 20.0);
            particle.color = spark_color(vec3(1.0, 0.843, 0.196));
            particle.end_time = frame.globals.info.time + particle.duration;
        }
    }
}

fn spark_color(rgb: Vec3) -> Vec3 {
    let luma = rgb.dot(vec3(0.299, 0.587, 0.114));
    Vec3::splat(luma).lerp(rgb, 2.0).lerp(Vec3::ONE, 0.2) * 2.0
}
