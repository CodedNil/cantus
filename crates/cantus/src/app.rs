use crate::render::{cpu::RenderState, launcher::LauncherState};
use interaction::InteractionState;
use music::{Enrichment, MusicBackend, PlaybackState};
use platform::{Current as Platform, Platform as _};
use std::{
    future::Future,
    io,
    sync::mpsc::{self, Sender},
    time::Duration,
};
use tokio::runtime::{Builder as RuntimeBuilder, Handle, Runtime};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[path = "config.rs"]
pub mod config;
#[path = "interaction.rs"]
pub mod interaction;
#[path = "music/mod.rs"]
pub mod music;
#[path = "platform/linux.rs"]
pub mod platform;

pub(crate) type Update<T> = Box<dyn FnOnce(&mut T) + Send>;
pub type AppUpdater = Sender<Update<CantusApp>>;

#[derive(Clone)]
pub struct Background {
    runtime: Handle,
    updater: AppUpdater,
}

impl Background {
    fn new(runtime: &Runtime, updater: &AppUpdater) -> Self {
        Self {
            runtime: runtime.handle().clone(),
            updater: updater.clone(),
        }
    }

    pub(crate) fn run(&self, job: impl FnOnce() + Send + 'static) {
        self.runtime.spawn_blocking(job);
    }

    pub(crate) fn spawn(&self, task: impl Future<Output = Option<Update<CantusApp>>> + Send + 'static) {
        let updater = self.updater.clone();
        self.runtime.spawn(async move {
            if let Some(event) = task.await {
                let _ = updater.send(event);
            }
        });
    }
}

pub struct CantusApp {
    pub(crate) render: RenderState,
    pub(crate) interaction: InteractionState,
    pub(crate) playback: PlaybackState,
    pub(crate) launcher: LauncherState,
    pub(crate) app_updates: mpsc::Receiver<Update<Self>>,
    pub(crate) config: config::Config,
    pub(crate) updater: AppUpdater,
    pub(crate) enrichment: Enrichment,
    pub(crate) music: MusicBackend,
    _runtime: Runtime,
}

impl Default for CantusApp {
    fn default() -> Self {
        let (updater, app_updates) = mpsc::channel();
        let runtime = RuntimeBuilder::new_multi_thread()
            .worker_threads(2)
            .max_blocking_threads(8)
            .thread_keep_alive(Duration::from_secs(10))
            .thread_name("cantus-async")
            // Keep renderer worker stacks small; these jobs are shallow async state machines.
            .thread_stack_size(512 * 1024)
            .enable_all()
            .build()
            .expect("failed to start Cantus async runtime");
        let background = Background::new(&runtime, &updater);
        let enrichment = Enrichment::new(background.clone());
        let config = config::load();
        let music = MusicBackend::spotify(&config, &updater, &background);
        Platform::start_launcher_listener(&background, &updater);
        Self {
            render: RenderState::default(),
            interaction: InteractionState::new(music.clone()),
            playback: PlaybackState::default(),
            launcher: LauncherState::new(&background, &enrichment.http, config.search_providers.clone()),
            app_updates,
            updater,
            enrichment,
            music,
            config,
            _runtime: runtime,
        }
    }
}

pub(crate) fn update(work: impl FnOnce(&mut CantusApp) + Send + 'static) -> Update<CantusApp> {
    Box::new(work)
}

pub(crate) fn send_update(sender: &AppUpdater, work: impl FnOnce(&mut CantusApp) + Send + 'static) -> bool {
    sender.send(update(work)).is_ok()
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

    platform::run();
}
