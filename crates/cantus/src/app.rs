use crate::render::{Globals, Interface, Services, TEXT_COLOR};
use isthmus::{Renderer, SurfaceHandle, glam::vec2};
use music::{Enrichment, MusicBackend, PlaybackState};
use platform::{Linux as Platform, Platform as _};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
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
pub(crate) mod config;
#[path = "music/mod.rs"]
pub(crate) mod music;
#[path = "platform/linux.rs"]
pub(crate) mod platform;

pub fn trigger_launcher() -> ! {
    Platform::trigger_launcher()
}

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

    pub(crate) fn spawn_blocking(&self, job: impl FnOnce() + Send + 'static) {
        self.runtime.spawn_blocking(job);
    }

    pub(crate) fn spawn_update(&self, task: impl Future<Output = Option<Update<CantusApp>>> + Send + 'static) {
        let updater = self.updater.clone();
        self.runtime.spawn(async move {
            if let Some(event) = task.await {
                let _ = updater.send(event);
            }
        });
    }

    pub(crate) fn spawn(&self, task: impl Future<Output = ()> + Send + 'static) {
        self.runtime.spawn(task);
    }
}

pub struct CantusApp {
    pub(crate) gpu: Option<Renderer<Globals>>,
    pub(crate) bar_surface: Option<SurfaceHandle>,
    pub(crate) playback: PlaybackState,
    pub(crate) ui: Interface,
    pub(crate) app_updates: mpsc::Receiver<Update<Self>>,
    pub(crate) config: config::Config,
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
        let ui = Interface::new(&config, &background, &enrichment);
        Self {
            gpu: None,
            bar_surface: None,
            playback: PlaybackState::default(),
            ui,
            app_updates,
            enrichment,
            music,
            config,
            _runtime: runtime,
        }
    }
}

impl CantusApp {
    /// Initializes the renderer for the first configured surface.
    ///
    /// # Panics
    /// Panics if initialized twice or Vulkan setup fails.
    pub fn initialize_renderer(&mut self, surface: &(impl HasDisplayHandle + HasWindowHandle), width: u32, height: u32) {
        assert!(self.gpu.is_none(), "GPU initialized twice");
        let (gpu, bar_surface) = Renderer::new(
            Globals::program(),
            surface,
            [width, height],
            include_bytes!("../../../assets/NotoSans-Variable.ttf"),
            TEXT_COLOR,
        )
        .expect("failed to initialize renderer");
        tracing::info!("Using Vulkan device: {}", gpu.device_name());
        self.gpu = Some(gpu);
        self.bar_surface = Some(bar_surface);
    }

    pub fn apply_pending_updates(&mut self) {
        while let Ok(update) = self.app_updates.try_recv() {
            update(self);
        }
    }

    pub fn render(&mut self, screen_size: [f32; 2], launcher: Option<(SurfaceHandle, [f32; 2])>) {
        let Some(gpu) = &mut self.gpu else {
            return;
        };
        let Some(bar_surface) = self.bar_surface else {
            return;
        };
        let screen_size = vec2(screen_size[0], screen_size[1]);
        let services = Services {
            config: &self.config,
            music: &self.music,
        };
        gpu.render(|render| {
            if let Some((surface, size)) = launcher {
                render.surface(surface, services, |frame| {
                    frame.globals.panel_height = self.config.height;
                    frame.globals.info.screen_size = vec2(size[0], size[1]);
                    if let Some(position) = frame.press_position() {
                        frame.globals.ripple(position);
                    }
                    self.ui.launcher.show(frame);
                });
            } else {
                render.surface(bar_surface, services, |frame| {
                    frame.globals.panel_height = self.config.height;
                    frame.globals.info.screen_size = screen_size;
                    self.ui.show(frame, &mut self.playback);
                });
            }
        });
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
        .with(Targets::new().with_default(LevelFilter::WARN).with_target("cantus", Level::INFO))
        .with(fmt::layer().with_writer(io::stderr))
        .init();

    platform::run();
}
