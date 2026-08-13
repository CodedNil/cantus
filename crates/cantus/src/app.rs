use crate::render::cpu::RenderState;
use interaction::InteractionState;
use spotify::PlaybackState;
use std::{
    io,
    sync::{
        Arc, Mutex,
        mpsc::{self, Sender},
    },
    thread,
    time::Instant,
};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use ureq::Agent;

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
type Job = Box<dyn FnOnce() -> Update<CantusApp> + Send>;

#[derive(Clone)]
pub struct Background {
    sender: Sender<Job>,
    pub http: Agent,
}

impl Background {
    fn new(updater: &AppUpdater) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        for _ in 0..8 {
            let receiver = Arc::clone(&receiver);
            let updater = updater.clone();
            thread::spawn(move || {
                loop {
                    let job = receiver.lock().expect("worker queue poisoned").recv();
                    let Ok(job) = job else { break };
                    send_update(&updater, job());
                }
            });
        }
        Self {
            sender,
            http: Agent::config_builder()
                .user_agent(concat!("Cantus/", env!("CARGO_PKG_VERSION")))
                .build()
                .into(),
        }
    }

    pub fn submit<U>(&self, work: impl FnOnce() -> U + Send + 'static) -> bool
    where
        U: FnOnce(&mut CantusApp) + Send + 'static,
    {
        self.sender.send(Box::new(move || Box::new(work()))).is_ok()
    }
}

#[derive(Clone)]
pub enum Fetch<T> {
    Missing(Instant),
    Fetching,
    Ready(T),
}

impl<T> Default for Fetch<T> {
    fn default() -> Self {
        Self::Missing(Instant::now())
    }
}

impl<T> Fetch<T> {
    pub fn request(&mut self, now: Instant) -> bool {
        if !matches!(self, Self::Missing(retry_at) if *retry_at <= now) {
            return false;
        }
        *self = Self::Fetching;
        true
    }

    pub const fn ready(&self) -> Option<&T> {
        match self {
            Self::Ready(value) => Some(value),
            _ => None,
        }
    }

    pub const fn ready_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Ready(value) => Some(value),
            _ => None,
        }
    }
}

pub struct CantusApp {
    pub(crate) render: RenderState,
    pub(crate) interaction: InteractionState,
    pub(crate) playback: PlaybackState,
    pub(crate) app_updates: mpsc::Receiver<Update<Self>>,
    pub(crate) config: config::Config,
    pub(crate) spotify: spotify::SpotifyBackend,
    pub(crate) updater: AppUpdater,
    pub(crate) background: Background,
}

impl Default for CantusApp {
    fn default() -> Self {
        let (updater, app_updates) = mpsc::channel();
        let background = Background::new(&updater);
        let mut config = config::load();
        let spotify = spotify::SpotifyBackend::new(&mut config, &updater, background.clone());
        Self {
            render: RenderState::default(),
            interaction: InteractionState::new(spotify.clone()),
            playback: PlaybackState::default(),
            app_updates,
            spotify,
            updater,
            background,
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
