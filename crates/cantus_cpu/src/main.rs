use crate::{
    interaction::InteractionState,
    render::{RenderState, status::Status, weather::Weather},
    spotify::PlaybackState,
};
use cantus_shared::WEATHER_CALENDAR_EXTENSION;
use glam::Vec2;
use std::{
    io,
    sync::mpsc::{self, Sender},
};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod interaction;
mod platform;
mod render;
mod spotify;

const PANEL_START: f32 = 6.0;
const PANEL_EXTENSION: f32 = WEATHER_CALENDAR_EXTENSION + 16.0;
const PARTICLE_COUNT: usize = 64;
const MAX_RENDER_INSTANCES: usize = 64;
const TRACK_SPACING_MS: f32 = 4000.0;

#[derive(Copy, Clone)]
struct Rect {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

impl Rect {
    const fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self { x0, y0, x1, y1 }
    }

    const fn pill(x: f32, width: f32, height: f32) -> Self {
        Self::new(x, PANEL_START, x + width, PANEL_START + height)
    }

    fn from_center(center: Vec2, half_size: Vec2) -> Self {
        let (min, max) = (center - half_size, center + half_size);
        Self::new(min.x, min.y, max.x, max.y)
    }

    fn contains(self, point: Vec2) -> bool {
        point.x >= self.x0 && point.x <= self.x1 && point.y >= self.y0 && point.y <= self.y1
    }
}

type Update<T> = Box<dyn FnOnce(&mut T) + Send>;
type AppUpdater = Sender<Update<CantusApp>>;

fn send_update<T>(
    sender: &Sender<Update<T>>,
    update: impl FnOnce(&mut T) + Send + 'static,
) -> bool {
    sender.send(Box::new(update)).is_ok()
}

struct CantusApp {
    render: RenderState,
    interaction: InteractionState,
    playback: PlaybackState,
    app_updates: mpsc::Receiver<Update<Self>>,
    config: config::Config,
    spotify: spotify::SpotifyBackend,
    status: Status,
    weather: Weather,
}

impl Default for CantusApp {
    fn default() -> Self {
        let (updater, app_updates) = mpsc::channel();
        let mut config = config::load();
        Self {
            render: RenderState::default(),
            interaction: InteractionState::default(),
            playback: PlaybackState::default(),
            app_updates,
            spotify: spotify::SpotifyBackend::new(&mut config, updater.clone()),
            status: Status::new(updater.clone()),
            weather: Weather::new(config.location, updater),
            config,
        }
    }
}

fn main() {
    tracing_subscriber::registry()
        .with(
            Targets::new()
                .with_default(LevelFilter::WARN)
                .with_target("cantus", Level::INFO)
                .with_target("wgpu_hal", Level::ERROR),
        )
        .with(fmt::layer().with_writer(io::stderr))
        .init();

    platform::wayland::run();
}
