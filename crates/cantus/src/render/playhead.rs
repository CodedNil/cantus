use crate::render::{
    Globals, PANEL_START,
    sdf::{sd_capsule_box, sd_rounded_triangle},
};
use isthmus::{
    Fragment, Quad,
    float::FloatExt as _,
    glam::{Vec3, vec2, vec3},
    spirv_std::arch::kill,
};

#[cfg(not(target_arch = "spirv"))]
use {
    crate::{app::music::PlaybackState, render::Frame},
    isthmus::Rect,
};

const START_DURATION: f32 = 0.7;
const TRANSITION_SPEED: f32 = 5.5;

#[derive(Default)]
pub struct Playhead {
    bar_split: f32,
    icon_presence: f32,
    icon_morph: f32,
}

#[isthmus::paint]
impl Playhead {
    pub fn show(&mut self, frame: &mut Frame, playback: &PlaybackState) {
        let response = frame.interact(Rect::from_center(
            vec2(frame.globals.playhead_x, PANEL_START + frame.services.config.height * 0.5),
            vec2(frame.services.config.height * 0.25, frame.services.config.height * 0.5),
        ));
        let speed = TRANSITION_SPEED * frame.delta_time;
        let last_toggle = playback.last_toggle.elapsed().as_secs_f32() / START_DURATION;
        if !response.hovered() && playback.playing && last_toggle < 1.0 {
            self.bar_split = 1.0 - last_toggle;
            self.icon_presence = 1.0 - last_toggle;
            self.icon_morph.approach(1.0, speed * 1.5);
        } else {
            let show_icon = f32::from(response.hovered() || !playback.playing);
            let play_icon = f32::from(response.hovered() && !playback.playing);
            self.bar_split.approach(show_icon, speed);
            self.icon_presence = self.icon_presence.max(show_icon);
            self.icon_presence.approach(show_icon, speed);
            self.icon_morph.approach(play_icon, speed);
        }

        if response.clicked() {
            frame.services.music.toggle_playing(playback.playing);
        }
        let half_width = frame.globals.panel_height * 0.4;
        let quad = Quad::from_min_max(
            vec2(frame.globals.playhead_x - half_width, PANEL_START - 5.0),
            vec2(frame.globals.playhead_x + half_width, PANEL_START + frame.globals.panel_height + 5.0),
        );
        // GPU: Playhead and playback icon.
        frame.paint_quad(quad, |fragment: Fragment, frame: Globals, bar_split: f32, icon_presence: f32, icon_morph: f32| {
            let mirrored = fragment.local.abs();

            // Vertical timeline bar.
            let bar_len = frame.panel_height * (0.5 - 0.375 * bar_split);
            let bar_center = (frame.panel_height - bar_len) * 0.5;
            let bar_distance = sd_capsule_box(vec2(mirrored.y - bar_center, mirrored.x), bar_len * 0.5, 4.5);

            // Pause icon, whose bars separate as it appears.
            let pause_x = (mirrored.x - 4.0 * bar_split).abs();
            let pause_y = (mirrored.y - frame.panel_height * 0.1).max(0.0);
            let pause_distance = vec2(pause_x, pause_y).length() - 3.5;

            // Play icon and pause-to-play morph.
            let play_scale = frame.panel_height * 0.18 * (1.0 + icon_morph * (1.0 - icon_presence));
            let play_distance = sd_rounded_triangle(fragment.local.perp(), play_scale, play_scale * 0.5);
            let icon_distance = pause_distance + (play_distance - pause_distance) * icon_morph;

            // Coverage and shaded inset.
            let bar_mask = 1.0 - bar_distance.smoothstep(-0.8, 0.2);
            let icon_mask = (1.0 - icon_distance.smoothstep(-0.8, 0.2)) * icon_presence;
            let alpha = icon_mask.max(bar_mask);
            if alpha <= 0.0 {
                kill();
            }
            let edge = bar_distance.min(icon_distance).smoothstep(-2.5, -1.0);
            let color = vec3(1.0, 0.878, 0.824).lerp(Vec3::splat(0.15), edge);
            (color * alpha).extend(alpha)
        });
    }
}
