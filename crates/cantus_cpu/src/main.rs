use crate::{
    interaction::InteractionState, model::PlaybackState, render::RenderState, status::Status,
    weather::Weather,
};
use cantus_shared::WEATHER_CALENDAR_EXTENSION;
use std::{
    io,
    sync::mpsc::{self, Sender},
};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod art;
mod config;
mod interaction;
mod layer_shell;
mod model;
mod pipelines;
mod render;
mod spotify;
mod status;
mod text_render;
mod weather;

const PANEL_START: f32 = 6.0;
const PANEL_EXTENSION: f32 = WEATHER_CALENDAR_EXTENSION + 16.0;
const PARTICLE_COUNT: usize = 64;
const MAX_RENDER_INSTANCES: usize = 256;
const TRACK_SPACING_MS: f32 = 4000.0;

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

    layer_shell::run();
}
