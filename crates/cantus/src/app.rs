use crate::render::cpu::RenderState;
use interaction::InteractionState;
use music::{Enrichment, MusicBackend, PlaybackState};
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
#[path = "music/mod.rs"]
pub mod music;
#[path = "platform/mod.rs"]
pub mod platform;

pub type Update<T> = Box<dyn FnOnce(&mut T) + Send>;
pub type AppUpdater = Sender<Update<CantusApp>>;

pub struct CantusApp {
    pub(crate) render: RenderState,
    pub(crate) interaction: InteractionState,
    pub(crate) playback: PlaybackState,
    pub(crate) app_updates: mpsc::Receiver<Update<Self>>,
    pub(crate) config: config::Config,
    pub(crate) updater: AppUpdater,
    pub(crate) enrichment: Enrichment,
    pub(crate) music: MusicBackend,
}

impl Default for CantusApp {
    fn default() -> Self {
        let (updater, app_updates) = mpsc::channel();
        let enrichment = Enrichment::new(&updater);
        let config = config::load();
        let music = MusicBackend::spotify(&config, &updater, enrichment.http.clone());
        Self {
            render: RenderState::default(),
            interaction: InteractionState::new(music.clone()),
            playback: PlaybackState::default(),
            app_updates,
            updater,
            enrichment,
            music,
            config,
        }
    }
}

pub(crate) fn send_update(
    sender: &AppUpdater,
    update: impl FnOnce(&mut CantusApp) + Send + 'static,
) -> bool {
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
