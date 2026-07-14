use crate::{
    interaction::InteractionState,
    model::{AppUpdate, AppUpdater, PlaybackState},
    render::RenderState,
};
use std::{
    io,
    sync::mpsc::{self, Receiver},
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
mod text_render;

const PANEL_START: f32 = 6.0;
const PANEL_EXTENSION: f32 = 44.0;
const PARTICLE_COUNT: usize = 64;
const MAX_RENDER_INSTANCES: usize = 256;
const MAX_HISTORY_TRACKS: usize = 6;
const TRACK_SPACING_MS: f32 = 4000.0;

struct CantusApp {
    render: RenderState,
    interaction: InteractionState,
    playback: PlaybackState,
    app_updates: Receiver<AppUpdate>,
    config: config::Config,
    spotify: spotify::SpotifyBackend,
}

impl Default for CantusApp {
    fn default() -> Self {
        let (update_tx, app_updates) = mpsc::channel();
        let updater = AppUpdater(update_tx);
        let config = config::load();
        let spotify = spotify::SpotifyBackend::new(&config, updater);
        Self {
            render: RenderState::default(),
            interaction: InteractionState::default(),
            playback: PlaybackState::default(),
            app_updates,
            spotify,
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
