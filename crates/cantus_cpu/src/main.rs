use crate::{
    interaction::InteractionState,
    model::{AppUpdate, AppUpdater, PlaybackState},
    render::{GpuResources, RenderState},
};
use cantus_shared::{BackgroundPill, GlobalUniforms, Particle, PlayheadUniforms};
use std::{
    io,
    sync::mpsc::{self, Receiver},
    time::Instant,
};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use wgpu::Instance;

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
    instance: Instance,
    gpu_resources: Option<GpuResources>,

    start_time: Instant,
    render_state: RenderState,
    interaction: InteractionState,
    playback_state: PlaybackState,
    app_updates: Receiver<AppUpdate>,
    config: config::Config,
    spotify: spotify::SpotifyBackend,
    last_toggle_playing: Instant,
    particles: [Particle; PARTICLE_COUNT],
    particles_accumulator: f32,
    particles_dirty: bool,
    /// Physical buffer pixels per logical Wayland surface pixel.
    render_scale: f32,
    surface_width: Option<f32>,
    global_uniforms: GlobalUniforms,
    background_pills: Vec<BackgroundPill>,
    playhead_info: PlayheadUniforms,
}

impl Default for CantusApp {
    fn default() -> Self {
        let (update_tx, app_updates) = mpsc::channel();
        let updater = AppUpdater(update_tx);
        let config = config::load();
        let spotify = spotify::SpotifyBackend::new(&config, updater);
        Self {
            instance: Instance::default(),
            gpu_resources: None,
            start_time: Instant::now(),
            render_state: RenderState::default(),
            interaction: InteractionState::default(),
            playback_state: PlaybackState::default(),
            app_updates,
            spotify,
            config,
            last_toggle_playing: Instant::now(),
            particles: [Particle::default(); PARTICLE_COUNT],
            particles_accumulator: 0.0,
            particles_dirty: false,
            render_scale: 1.0,
            surface_width: None,
            global_uniforms: GlobalUniforms::default(),
            background_pills: Vec::with_capacity(MAX_RENDER_INSTANCES),
            playhead_info: PlayheadUniforms::default(),
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
