use crate::render::shared::{FrameData, RipplePulse};
use crate::{PANEL_START, config::Config, interaction::InteractionState};
use isthmus::glam::{Vec2, vec2};

pub fn approach(current: &mut f32, target: f32, speed: f32) {
    *current += (target - *current).clamp(-speed, speed);
}

/// The values almost every pass needs each frame.
pub struct Frame<'a> {
    pub shared: &'a mut FrameData,
    pub delta_time: f32,
    pub screen_width: f32,
    pub scale: f32,
    pub config: &'a Config,
    pub interaction: &'a mut InteractionState,
}

impl<'a> Frame<'a> {
    pub fn begin(
        shared: &'a mut FrameData,
        interaction: &'a mut InteractionState,
        config: &'a Config,
        elapsed: f32,
        screen_size: Vec2,
        scale: f32,
    ) -> Self {
        let delta_time = (elapsed - shared.time).min(0.1);
        shared.time = elapsed;
        shared.screen_size = screen_size;
        shared.panel_top = PANEL_START;
        shared.panel_height = config.height;
        shared.mouse_pos = interaction.pointer;
        interaction.begin_frame();
        Self {
            shared,
            delta_time,
            screen_width: screen_size.x,
            scale,
            config,
            interaction,
        }
    }

    pub fn finish(&mut self) {
        approach(
            &mut self.shared.mouse_pressure,
            self.interaction.mouse_pressure(),
            5.0 * self.delta_time,
        );
        if let Some(origin) = self.interaction.end_frame() {
            let ripple = self
                .shared
                .ripples
                .iter_mut()
                .min_by(|a, b| a.animation.x.total_cmp(&b.animation.x))
                .unwrap();
            *ripple = RipplePulse {
                origin,
                animation: vec2(self.shared.time, 1.0),
            };
        }
    }
}
