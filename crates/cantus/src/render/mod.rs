pub mod launcher;
pub mod lyrics;
pub mod music;
pub mod sdf;
pub mod status;
pub mod tempestas;

use isthmus::{
    ShaderData,
    glam::{Vec2, Vec3},
};

#[cfg(not(target_arch = "spirv"))]
use isthmus::Frame;

#[cfg(not(target_arch = "spirv"))]
use crate::{
    app::{
        Background,
        config::Config,
        music::{Enrichment, Music},
    },
    interaction::Interaction,
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

#[repr(C)]
#[derive(Clone, Copy, Default, ShaderData)]
pub struct Globals {
    pub pointer: Vec2,
    pub pressure: f32,
    pub bar_height: f32,
    pub ripples: [RipplePulse; 4],
}

pub type Fragment = isthmus::Fragment<Globals>;
pub type TextFragment<'a> = isthmus::TextFragment<'a, Globals>;

isthmus::program!();

#[repr(C)]
#[derive(Clone, Copy, Default, ShaderData)]
pub struct RipplePulse {
    pub origin: Vec2,
    pub start_time: f32,
}

#[cfg(not(target_arch = "spirv"))]
pub struct RenderContext<'a> {
    pub paint: Frame<'a>,
    pub config: &'a Config,
    pub interaction: &'a mut Interaction,
}

#[cfg(not(target_arch = "spirv"))]
#[derive(Clone, Copy)]
pub struct BarLayout {
    pub playhead_x: f32,
    pub px_per_ms: f32,
}

#[cfg(not(target_arch = "spirv"))]
impl<'a> RenderContext<'a> {
    pub fn new(paint: Frame<'a>, config: &'a Config, interaction: &'a mut Interaction) -> Self {
        interaction.begin_frame(paint.delta_time, paint.time);
        Self { paint, config, interaction }
    }

    pub const fn globals(&self) -> Globals {
        self.interaction.globals(self.config.height)
    }

    pub fn finish(mut self) {
        self.paint.set_globals(self.globals());
        self.interaction.end_frame();
    }
}

#[cfg(not(target_arch = "spirv"))]
pub struct Ui {
    pub(crate) launcher: launcher::LauncherState,
    pub(crate) bar: Bar,
}

#[cfg(not(target_arch = "spirv"))]
pub struct Bar {
    pub(crate) lyrics: Option<lyrics::LyricsView>,
    pub(crate) weather: Option<tempestas::WeatherPanel>,
    pub(crate) status: Option<status::StatusPanel>,
    music_view: music::MusicView,
}

#[cfg(not(target_arch = "spirv"))]
impl Ui {
    pub fn new(config: &Config, background: &Background, enrichment: &Enrichment) -> Self {
        Self {
            launcher: launcher::LauncherState::new(background, &enrichment.http, config.search_providers.clone()),
            bar: Bar::new(config, background, enrichment),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Bar {
    fn new(config: &Config, background: &Background, enrichment: &Enrichment) -> Self {
        Self {
            lyrics: config.lyrics_enabled.then(|| lyrics::LyricsView::new(enrichment.clone())),
            weather: config
                .tempestas_enabled
                .then(|| tempestas::WeatherPanel::new(&config.timezones, background, enrichment.http.clone())),
            status: config.status_enabled.then(|| status::StatusPanel::new(background)),
            music_view: music::MusicView::default(),
        }
    }

    pub fn show(&mut self, frame: &mut RenderContext, music: &mut Music) {
        let status_width = self.status.as_ref().map_or(0.0, |status| status.width() + GAP);
        let reserved = frame.config.history_width + GAP + f32::from(frame.config.tempestas_enabled) * (tempestas::WIDTH + GAP) + status_width;
        let px_per_ms = (frame.paint.screen_size.x - reserved).max(84.0) / (frame.config.timeline_future_minutes * 60_000.0);
        let layout = BarLayout {
            playhead_x: frame.config.history_width + frame.config.timeline_past_minutes * 60_000.0 * px_per_ms,
            px_per_ms,
        };
        if let Some(lyrics) = self.lyrics.as_mut() {
            lyrics.show(frame, music, layout);
        }
        let sky = self
            .weather
            .as_mut()
            .map_or_else(tempestas::StatusSky::default, |weather| weather.show(frame, status_width));
        if let Some(status) = self.status.as_mut() {
            status.show(frame, sky);
        }
        self.music_view.show(frame, music, layout);
    }
}
