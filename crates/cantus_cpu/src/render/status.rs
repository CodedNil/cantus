use crate::{
    AppUpdater,
    interaction::{InteractionState, Rect},
    platform::linux as platform,
};
use cantus_gpu::{
    GAP,
    status::{AUDIO_SPECTRUM_BANDS, Data, StatusSection, WIDTH, pill_x},
};
use glam::{FloatExt, vec2};
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

#[derive(Default)]
pub struct StatusRuntime {
    audio_spectrum: Arc<[AtomicU32; AUDIO_SPECTRUM_BANDS]>,
}

impl StatusRuntime {
    pub fn new(updater: AppUpdater) -> Self {
        let status = Self::default();
        platform::start_status_monitor(updater, Arc::clone(&status.audio_spectrum));
        status
    }

    pub fn update_data(&self, data: &mut Data, dt: f32) {
        for (damped, level) in data.audio_spectrum.iter_mut().zip(self.audio_spectrum.iter()) {
            let target = f32::from_bits(level.load(Ordering::Relaxed));
            let response = if target > *damped { 18.0 } else { 6.0 };
            *damped += (target - *damped) * (1.0 - (-response * dt).exp());
        }
        if data.battery_level >= 0.995 {
            data.battery_level = -1.0;
        }
        data.history_scroll =
            (data.history_scroll + dt / platform::STATUS_SAMPLE_INTERVAL.as_secs_f32()).saturate();
        data.power_hover = 0.0;
        data.sun_height = 0.0;
        data.conditions = [0.0; 6];
    }

    pub fn interact(
        pill: &mut Data,
        screen_width: f32,
        height: f32,
        ui: &mut InteractionState,
        dt: f32,
    ) {
        let x = pill_x(screen_width);
        let scroll = ui.scroll(section_rect(pill, x, height, StatusSection::Audio));
        if scroll != 0 {
            let sign = pill.volume.signum();
            pill.volume = (pill.volume.abs() - scroll as f32 * 0.05).saturate() * sign;
            platform::set_volume(pill.volume.abs());
        }

        let buttons = StatusSection::POWER_ACTIONS
            .map(|section| ui.surface(section_rect(pill, x, height, section)));
        pill.power_hover = buttons
            .iter()
            .position(|response| response.hovered)
            .map_or(0.0, |action| action as f32 + 1.0);
        if let Some(action) = buttons.iter().position(|response| response.pressed) {
            pill.power_state = action as f32 + 1.0;
        }
        if pill.power_state > 0.0 {
            let action = pill.power_state.floor() as usize - 1;
            let progress = pill.power_state.fract() + dt / 1.5;
            if !ui.down() || !buttons[action].hovered {
                pill.power_state = 0.0;
            } else if progress >= 1.0 {
                pill.power_state = 0.0;
                platform::run_power_action(action);
            } else {
                pill.power_state = action as f32 + 1.0 + progress;
            }
        }
        ui.surface(Rect::pill(x, WIDTH, height));
    }
}

fn section_rect(pill: &Data, x: f32, height: f32, section: StatusSection) -> Rect {
    Rect::from_center(
        vec2(
            x + pill.section_center(section),
            crate::PANEL_START + height * 0.5,
        ),
        vec2((pill.section_width(section) + GAP) * 0.5, height * 0.5),
    )
}
