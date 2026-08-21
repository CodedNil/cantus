use crate::render::{
    FrameData, PANEL_START,
    shader::{pixel_to_ndc, quad_coord, sd_capsule_box, sd_rounded_triangle},
    smoothstep,
};
use isthmus::{
    Vertex,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv_std::arch::kill,
};

#[cfg(feature = "cpu")]
use crate::{
    app::{interaction::Rect, music::PlaybackState},
    render::cpu::{Frame, Passes, approach},
};

#[isthmus::pass]
pub struct PlayheadPass {
    pill: isthmus::Instance<Self>,
}

#[isthmus::data]
#[derive(Default)]
pub struct PlayheadState {
    pub bar_split: f32,
    pub icon_presence: f32,
    pub icon_morph: f32,
}

#[derive(isthmus::Varyings)]
pub struct Varyings {
    pub world_pos: Vec2,
}

#[isthmus::pass]
impl PlayheadPass {
    pub fn new(passes: &Passes<'_>) -> Self {
        Self {
            pill: passes.instance((), PlayheadState::default()),
        }
    }

    pub fn update(
        &mut self,
        frame: &mut Frame,
        playback: &mut PlaybackState,
        last_toggle_time: &mut f32,
    ) {
        const START_DURATION: f32 = 0.7;
        const TRANSITION_SPEED: f32 = 5.5;

        let playhead_x = frame.shared.playhead_x;
        let height = frame.config.height;
        let time = frame.shared.time;
        let playhead = frame.interaction.surface(Rect::from_center(
            vec2(playhead_x, PANEL_START + height * 0.5),
            vec2(height * 0.25, height * 0.5),
        ));
        let speed = TRANSITION_SPEED * frame.delta_time;
        let last_toggle = (time - *last_toggle_time) / START_DURATION;
        if !playhead.hovered && playback.playing && last_toggle < 1.0 {
            self.pill.bar_split = 1.0 - last_toggle;
            self.pill.icon_presence = 1.0 - last_toggle;
            approach(&mut self.pill.icon_morph, 1.0, speed * 1.5);
        } else {
            let show_icon = f32::from(playhead.hovered || !playback.playing);
            let play_icon = f32::from(playhead.hovered && !playback.playing);
            approach(&mut self.pill.bar_split, show_icon, speed);
            self.pill.icon_presence = self.pill.icon_presence.max(show_icon);
            approach(&mut self.pill.icon_presence, show_icon, speed);
            approach(&mut self.pill.icon_morph, play_icon, speed);
        }

        if playhead.clicked {
            frame.interaction.toggle_playing(playback.playing);
        }
    }

    #[gpu]
    pub fn vertex(
        #[gpu(vertex_index)] vertex: u32,
        #[gpu(shared)] frame: FrameData,
    ) -> Vertex<Varyings> {
        let uv = quad_coord(vertex);
        let world_pos = vec2(
            frame.playhead_x + (uv.x * 2.0 - 1.0) * frame.panel_height * 0.4,
            PANEL_START - 5.0 + uv.y * (frame.panel_height + 10.0),
        );
        Vertex {
            position: pixel_to_ndc(world_pos, frame.screen_size),
            varyings: Varyings { world_pos },
        }
    }

    #[gpu]
    pub fn fragment(
        Varyings { world_pos }: Varyings,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] state: PlayheadState,
    ) -> Vec4 {
        let center = vec2(frame.playhead_x, PANEL_START + frame.panel_height * 0.5);
        let pause = (world_pos - center).abs();

        // Bar splits into two capsule segments straddling the center as bar_split grows.
        let bar_len = frame.panel_height * (0.5 - 0.375 * state.bar_split);
        let bar_center = (frame.panel_height - bar_len) * 0.5;
        let dist_bar = sd_capsule_box(vec2(pause.y - bar_center, pause.x), bar_len * 0.5, 4.5);

        let dx = (pause.x - 4.0 * state.bar_split).abs();
        let dy = (pause.y - frame.panel_height * 0.1).max(0.0);
        let dist_pause = vec2(dx, dy).length() - 3.5;
        let play_scale =
            frame.panel_height * 0.18 * (1.0 + state.icon_morph * (1.0 - state.icon_presence));
        let dist_play = sd_rounded_triangle((world_pos - center).perp(), play_scale, play_scale * 0.5);
        let dist_icon = dist_pause + (dist_play - dist_pause) * state.icon_morph;
        let bar_mask = 1.0 - smoothstep(-0.8, 0.2, dist_bar);
        let icon_mask = (1.0 - smoothstep(-0.8, 0.2, dist_icon)) * state.icon_presence;
        let alpha = icon_mask.max(bar_mask);
        if alpha <= 0.0 {
            kill();
        }
        let edge = smoothstep(-2.5, -1.0, dist_bar.min(dist_icon));
        vec3(1.0, 0.878, 0.824)
            .lerp(Vec3::splat(0.15), edge)
            .extend(alpha)
    }
}
