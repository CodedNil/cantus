pub mod launcher;
pub mod lyrics;
pub mod particles;
pub mod playhead;
pub(crate) mod sdf;
pub mod status;
pub mod tempestas;
pub mod track;

pub use isthmus::text;

use isthmus::{
    FrameInfo,
    glam::{Vec2, Vec3},
};

#[cfg(not(target_arch = "spirv"))]
use self::{
    launcher::LauncherState,
    lyrics::LyricsView,
    particles::Particles,
    playhead::Playhead,
    status::StatusPanel,
    tempestas::{StatusSky, WeatherPanel},
    track::TrackView,
};
#[cfg(not(target_arch = "spirv"))]
use crate::app::{
    AppUpdater, Background,
    config::Config,
    music::{Enrichment, MusicBackend, PlaybackState},
};
pub const TEXT_COLOR: Vec3 = Vec3::splat(0.94);

/// Gap between the top of the surface and the top of the bar.
pub const PANEL_START: f32 = 6.0;
/// Base spacing unit. Sizes and gaps should be whole multiples of it.
pub const UNIT: f32 = 4.0;
/// The standard small gap between adjacent elements.
pub const GAP: f32 = UNIT * 2.0;
/// The standard inset between a container edge and its contents.
pub const PADDING: f32 = UNIT * 3.0;

isthmus::shader_module!(pub(crate) SHADER: Globals);

#[isthmus::data]
#[derive(Default)]
pub struct Globals {
    pub info: FrameInfo,
    pub panel_height: f32,
    pub playhead_x: f32,
    pub px_per_ms: f32,
    /// Space reserved for the status pill this frame (its current width plus a gap), or 0 if hidden.
    pub status_width: f32,
    /// Current hour at the detected weather location.
    pub weather_hour: f32,
    pub ripples: [RipplePulse; 4],
}

#[isthmus::data]
#[derive(Default)]
pub struct RipplePulse {
    pub origin: Vec2,
    pub start_time: f32,
    pub strength: f32,
}

impl isthmus::FrameGlobals for Globals {
    fn frame_info(&self) -> &FrameInfo {
        &self.info
    }

    fn frame_info_mut(&mut self) -> &mut FrameInfo {
        &mut self.info
    }
}

#[cfg(not(target_arch = "spirv"))]
#[derive(Clone, Copy)]
pub struct Services<'a> {
    pub config: &'a Config,
    pub music: &'a MusicBackend,
}

#[cfg(not(target_arch = "spirv"))]
pub type Frame<'frame, 'app> = isthmus::Frame<'frame, Globals, Services<'app>>;

#[cfg(not(target_arch = "spirv"))]
pub(crate) fn capture_click_ripple(frame: &mut Frame) {
    let Some(position) = frame.click_position() else {
        return;
    };
    let pulse = RipplePulse {
        origin: position,
        start_time: frame.globals.info.time,
        strength: 1.0,
    };
    if let Some(ripple) = frame.globals.ripples.iter_mut().min_by(|a, b| a.start_time.total_cmp(&b.start_time)) {
        *ripple = pulse;
    }
}

#[cfg(not(target_arch = "spirv"))]
pub(crate) struct Interface {
    pub(crate) launcher: LauncherState,
    pub(crate) lyrics: Option<LyricsView>,
    pub(crate) weather: Option<WeatherPanel>,
    pub(crate) status: Option<StatusPanel>,
    tracks: TrackView,
    particles: Particles,
    playhead: Playhead,
}

#[cfg(not(target_arch = "spirv"))]
impl Interface {
    pub fn new(config: &Config, background: &Background, updater: AppUpdater, enrichment: &Enrichment, music: &MusicBackend) -> Self {
        Self {
            launcher: LauncherState::new(background, &enrichment.http, config.search_providers.clone()),
            lyrics: config.lyrics_enabled.then(|| LyricsView::new(enrichment.clone(), music.clone())),
            weather: config
                .tempestas_enabled
                .then(|| WeatherPanel::new(&config.timezones, background, updater.clone(), enrichment.http.clone())),
            status: config.status_enabled.then(|| StatusPanel::new(background, updater)),
            tracks: TrackView::default(),
            particles: Particles::default(),
            playhead: Playhead::default(),
        }
    }

    pub fn show(&mut self, frame: &mut Frame, playback: &mut PlaybackState) {
        // Globals
        frame.globals.status_width = self.status.as_ref().map_or(0.0, |status| status.width() + GAP);
        frame.globals.px_per_ms = frame.services.config.timeline_px_per_ms(frame.globals.info.screen_size.x, frame.globals.status_width);
        frame.globals.playhead_x = frame.services.config.playhead_x(frame.globals.px_per_ms);
        let drag_offset_ms = if frame.dragging() {
            (frame.globals.info.mouse_pos.x - frame.drag_origin().x) / frame.globals.px_per_ms
        } else {
            0.0
        };
        playback.update_timeline(drag_offset_ms, frame.dragging(), frame.delta_time);

        // Modules
        if let Some(lyrics) = self.lyrics.as_mut() {
            lyrics.show(frame, playback);
        }
        let sky = self.weather.as_mut().map_or_else(StatusSky::default, |weather| weather.show(frame));
        if let Some(status) = self.status.as_mut() {
            status.show(frame, sky);
        }
        self.tracks.show(frame, playback, &mut self.particles);
        self.particles.show(frame, playback);
        self.playhead.show(frame, playback);
        capture_click_ripple(frame);
    }
}
