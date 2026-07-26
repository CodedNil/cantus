use crate::{
    interaction::InteractionState,
    render::{RenderState, status::Status, tempo::Weather},
    spotify::PlaybackState,
};
use std::{
    io,
    sync::mpsc::{self, Sender},
};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod interaction;
mod openmeteo;
mod platform;
mod render;
mod spotify;

const PANEL_START: f32 = 6.0;
const PANEL_OVERFLOW: f32 = 16.0;
const PARTICLE_COUNT: usize = 64;
const MAX_RENDER_INSTANCES: usize = 64;
const TRACK_SPACING_MS: f32 = 4000.0;

type Update<T> = Box<dyn FnOnce(&mut T) + Send>;
type AppUpdater = Sender<Update<CantusApp>>;

fn send_update<T>(sender: &Sender<Update<T>>, update: impl FnOnce(&mut T) + Send + 'static) -> bool {
    sender.send(Box::new(update)).is_ok()
}

struct CantusApp {
    render: RenderState,
    interaction: InteractionState,
    playback: PlaybackState,
    app_updates: mpsc::Receiver<Update<Self>>,
    config: config::Config,
    spotify: spotify::SpotifyBackend,
    status: Option<Status>,
    weather: Option<Weather>,
}

impl Default for CantusApp {
    fn default() -> Self {
        let (updater, app_updates) = mpsc::channel();
        let mut config = config::load();
        let status = config.status_enabled.then(|| Status::new(updater.clone()));
        let weather = config
            .weather_enabled
            .then(|| Weather::new(config.location, updater.clone()));
        Self {
            render: RenderState::default(),
            interaction: InteractionState::default(),
            playback: PlaybackState::default(),
            app_updates,
            spotify: spotify::SpotifyBackend::new(&mut config, updater.clone()),
            status,
            weather,
            config,
        }
    }
}

fn main() {
    #[cfg(all(debug_assertions, feature = "generate-nix"))]
    config::generate_nix_options();

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
