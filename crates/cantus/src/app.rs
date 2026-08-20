use crate::render::{cpu::RenderState, launcher::LauncherState};
use interaction::InteractionState;
use music::{Enrichment, MusicBackend, PlaybackState};
use parking_lot::Mutex;
use platform::{Current as Platform, Platform as _};
use std::{
    io,
    sync::{
        Arc,
        mpsc::{self, Sender},
    },
    thread,
};
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
pub(crate) type AppUpdater = Sender<Update<CantusApp>>;
type Job = Box<dyn FnOnce(&AppUpdater) + Send>;

#[derive(Clone)]
pub(crate) struct Background(Sender<Job>);

impl Background {
    fn new(updater: &AppUpdater) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        for _ in 0..8 {
            let receiver = Arc::clone(&receiver);
            let updater = updater.clone();
            thread::spawn(move || {
                loop {
                    let Ok(job) = receiver.lock().recv() else {
                        break;
                    };
                    job(&updater);
                }
            });
        }
        Self(sender)
    }

    pub(crate) fn submit(&self, job: impl FnOnce() -> Update<CantusApp> + Send + 'static) -> bool {
        self.0
            .send(Box::new(move |updater| {
                let _ = updater.send(job());
            }))
            .is_ok()
    }

    pub(crate) fn run(&self, job: impl FnOnce() + Send + 'static) -> bool {
        self.0.send(Box::new(move |_| job())).is_ok()
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
}

impl Default for CantusApp {
    fn default() -> Self {
        let (updater, app_updates) = mpsc::channel();
        let background = Background::new(&updater);
        let enrichment = Enrichment::new(background.clone());
        let config = config::load();
        let music = MusicBackend::spotify(&config, &updater, enrichment.http.clone());
        Platform::start_launcher_listener(&updater);
        Self {
            render: RenderState::default(),
            interaction: InteractionState::new(music.clone()),
            playback: PlaybackState::default(),
            launcher: LauncherState::new(&background, enrichment.http.clone()),
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

    platform::run();
}
