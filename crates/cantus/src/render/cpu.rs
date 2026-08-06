use super::{
    frame::Frame,
    particles, playhead,
    shared::{FrameData, GAP},
    status, tempo, text, track,
};
use crate::{CantusApp, PANEL_OVERFLOW, PANEL_START, spotify::PlaybackState};
use isthmus::{
    Present, Program,
    glam::vec2,
    wgpu::{Color, Instance, PowerPreference, Surface},
};
use std::{sync::Arc, time::Instant};

pub type Passes<'a> = isthmus::PassBuilder<'a, FrameData>;

#[derive(isthmus::Render)]
pub struct Systems {
    pub tempo: Option<tempo::TempoPass>,
    pub status: Option<status::StatusPass>,
    pub track: track::TrackPass,
    pub particles: particles::ParticlePass,
    pub playhead: playhead::PlayheadPass,
    #[render(skip)]
    pub text_font: text::Font,
}

type RenderProgram = Program<FrameData, Systems>;

pub struct RenderState {
    pub instance: Instance,
    pub program: Option<RenderProgram>,
    pub start_time: Instant,
    pub last_toggle_time: f32,
    /// Physical buffer pixels per logical Wayland surface pixel.
    pub scale: f32,
    pub surface_width: Option<f32>,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            instance: Instance::default(),
            program: None,
            start_time: Instant::now(),
            last_toggle_time: 0.0,
            scale: 1.0,
            surface_width: None,
        }
    }
}

impl RenderState {
    /// The GPU device and its dependent resources, valid once the Wayland surface is configured.
    pub const fn program(&mut self) -> &mut RenderProgram {
        self.program
            .as_mut()
            .expect("render called before GPU configured")
    }
}

impl Systems {
    fn new(passes: &Passes<'_>, app: &CantusApp) -> Self {
        let sampler = passes.filtering_sampler("Linear Sampler");
        let text_font = text::Font::new(passes);
        Self {
            tempo: app.config.tempo_enabled.then(|| {
                tempo::TempoPass::new(passes, app.config.location, app.updater.clone(), &text_font)
            }),
            status: app
                .config
                .status_enabled
                .then(|| status::StatusPass::new(passes, app.updater.clone(), &text_font)),
            track: track::TrackPass::new(passes, &sampler, &text_font),
            particles: particles::ParticlePass::new(passes),
            playhead: playhead::PlayheadPass::new(passes),
            text_font,
        }
    }

    fn update(
        &mut self,
        frame: &mut Frame<'_>,
        playback: &mut PlaybackState,
        last_toggle_time: &mut f32,
    ) {
        let status_width = self
            .status
            .as_ref()
            .map_or(0.0, |status| status.pill.width() + GAP);
        frame.shared.status_width = status_width;
        frame.shared.px_per_ms = frame.config.timeline_px_per_ms(frame.screen_width, status_width);
        frame.shared.playhead_x = frame.config.playhead_x(frame.shared.px_per_ms);
        if let Some(tempo) = self.tempo.as_mut() {
            tempo.update(&self.text_font, self.status.as_mut(), frame);
        }
        if let Some(status) = self.status.as_mut() {
            status.update(&self.text_font, frame);
        }
        self.playhead.update(frame, playback, last_toggle_time);
        self.track.update(&self.text_font, playback, frame);
        self.particles.update(&self.track, frame);
    }
}

impl CantusApp {
    pub fn initialize_gpu(&mut self, surface: Surface<'static>, width: u32, height: u32) {
        assert!(self.render.program.is_none(), "GPU initialized twice");
        let (program, info) = pollster::block_on(Program::surface(
            &self.render.instance,
            surface,
            width,
            height,
            PowerPreference::LowPower,
            isthmus::shader_module!(),
            |passes| Systems::new(passes, self),
        ))
        .expect("failed to initialize renderer");
        tracing::info!("Using adapter: {} ({:?})", info.name, info.device_type);
        program
            .device()
            .on_uncaptured_error(Arc::new(|error| tracing::error!(%error, "uncaptured wgpu error")));
        self.render.program = Some(program);
    }

    pub fn replace_render_surface(&mut self, surface: Surface<'static>) {
        self.render
            .program()
            .replace_surface(surface)
            .expect("replacement surface is incompatible");
    }

    pub fn logical_surface_size(&self) -> (f32, f32) {
        let extension = if self.config.tempo_enabled {
            tempo::EXTENSION + PANEL_OVERFLOW
        } else {
            PANEL_OVERFLOW
        };
        (
            self.render.surface_width.unwrap_or(1920.0),
            self.config.height + PANEL_START + extension,
        )
    }

    pub fn buffer_size(&self) -> (u32, u32) {
        let (width, height) = self.logical_surface_size();
        (
            (width * self.render.scale).round() as u32,
            (height * self.render.scale).round() as u32,
        )
    }

    pub fn render(&mut self) -> bool {
        self.start_missing_art_downloads();
        if self.render.program.is_none() {
            return false;
        }
        while let Ok(update) = self.app_updates.try_recv() {
            update(self);
        }

        let (screen_width, screen_height) = self.logical_surface_size();
        let Some(program) = self.render.program.as_mut() else {
            return false;
        };
        let elapsed = self.render.start_time.elapsed().as_secs_f32();
        let present = program.render(Color::TRANSPARENT, |shared, systems| {
            let mut frame = Frame::begin(
                shared,
                &mut self.interaction,
                &self.config,
                elapsed,
                vec2(screen_width, screen_height),
                self.render.scale,
            );
            systems.update(&mut frame, &mut self.playback, &mut self.render.last_toggle_time);
            frame.finish();
        });
        if matches!(present, Present::Validation) {
            tracing::error!("surface texture acquisition failed validation");
        }
        matches!(present, Present::Lost)
    }
}
