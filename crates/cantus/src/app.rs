#![allow(clippy::missing_panics_doc)]

use crate::render::cpu::RenderState;
use interaction::InteractionState;
use spotify::PlaybackState;
use std::{
    io,
    sync::mpsc::{self, Sender},
};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[path = "config.rs"]
pub mod config;
#[path = "interaction.rs"]
pub mod interaction;
#[path = "platform/mod.rs"]
pub mod platform;
#[path = "spotify.rs"]
pub mod spotify;

pub const PANEL_OVERFLOW: f32 = 16.0;
pub const MAX_RENDER_INSTANCES: usize = 32;
pub const TRACK_SPACING_MS: f32 = 4000.0;

pub type Update<T> = Box<dyn FnOnce(&mut T) + Send>;
pub type AppUpdater = Sender<Update<CantusApp>>;

pub struct CantusApp {
    pub(crate) render: RenderState,
    pub(crate) interaction: InteractionState,
    pub(crate) playback: PlaybackState,
    pub(crate) app_updates: mpsc::Receiver<Update<Self>>,
    pub(crate) config: config::Config,
    pub(crate) spotify: spotify::SpotifyBackend,
    pub(crate) updater: AppUpdater,
}

impl Default for CantusApp {
    fn default() -> Self {
        let (updater, app_updates) = mpsc::channel();
        let mut config = config::load();
        let spotify = spotify::SpotifyBackend::new(&mut config, updater.clone());
        Self {
            render: RenderState::default(),
            interaction: InteractionState::new(spotify.clone()),
            playback: PlaybackState::default(),
            app_updates,
            spotify,
            updater,
            config,
        }
    }
}

pub fn send_update<T>(sender: &Sender<Update<T>>, update: impl FnOnce(&mut T) + Send + 'static) -> bool {
    sender.send(Box::new(update)).is_ok()
}

pub fn run() {
    #[cfg(all(debug_assertions, feature = "generate-nix"))]
    config::nix_options::generate();

    tracing_subscriber::registry()
        .with(
            Targets::new()
                .with_default(LevelFilter::WARN)
                .with_target("cantus", Level::INFO)
                .with_target("wgpu_hal", Level::ERROR),
        )
        .with(fmt::layer().with_writer(io::stderr))
        .init();

    platform::linux::run();
}
