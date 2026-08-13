use crate::{
    app::{
        CantusApp, PANEL_OVERFLOW, config::Config, interaction::InteractionState, spotify::PlaybackState,
    },
    render::{
        lyrics::{self, LyricsPass},
        particles::ParticlePass,
        playhead::PlayheadPass,
        shared::{FrameData, GAP, PANEL_START, RipplePulse},
        status::{self, StatusPass},
        tempestas::{self, EXTENSION, TempestasPass},
        text::Renderer as TextRenderer,
        track::{self, TrackPass},
    },
};
use isthmus::{
    PassBuilder, Present, Program, Render,
    glam::{Vec2, vec2},
    wgpu::{Color, Instance, PowerPreference, RenderPass, Surface},
};
use std::{sync::Arc, time::Instant};

pub type Passes<'a> = PassBuilder<'a, FrameData>;
type RenderProgram = Program<FrameData, Systems>;

/// The values almost every pass needs each frame.
pub struct Frame<'a> {
    pub shared: &'a mut FrameData,
    pub delta_time: f32,
    pub config: &'a Config,
    pub interaction: &'a mut InteractionState,
}

pub struct Systems {
    pub lyrics: LyricsPass,
    pub tempestas: Option<TempestasPass>,
    pub status: Option<StatusPass>,
    pub track: TrackPass,
    pub particles: ParticlePass,
    pub playhead: PlayheadPass,
    pub text: TextRenderer,
}

pub struct RenderState {
    pub instance: Instance,
    pub program: Option<RenderProgram>,
    pub start_time: Instant,
    pub last_toggle_time: f32,
    /// Physical buffer pixels per logical Wayland surface pixel.
    pub scale: f32,
    pub surface_width: Option<f32>,
}

pub fn approach(current: &mut f32, target: f32, speed: f32) {
    *current += (target - *current).clamp(-speed, speed);
}

impl<'a> Frame<'a> {
    pub fn begin(
        shared: &'a mut FrameData,
        interaction: &'a mut InteractionState,
        config: &'a Config,
        elapsed: f32,
        screen_size: Vec2,
    ) -> Self {
        let delta_time = (elapsed - shared.time).min(0.1);
        shared.time = elapsed;
        shared.screen_size = screen_size;
        shared.panel_height = config.height;
        shared.mouse_pos = interaction.pointer;
        interaction.begin_frame();
        Self {
            shared,
            delta_time,
            config,
            interaction,
        }
    }

    pub fn finish(&mut self) {
        approach(
            &mut self.shared.mouse_pressure,
            self.interaction.pressure(),
            5.0 * self.delta_time,
        );
        if let Some(origin) = self.interaction.end_frame()
            && let Some(ripple) = self
                .shared
                .ripples
                .iter_mut()
                .min_by(|a, b| a.start_time.total_cmp(&b.start_time))
        {
            *ripple = RipplePulse {
                origin,
                start_time: self.shared.time,
                strength: 1.0,
            };
        }
    }
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
    ///
    /// # Panics
    ///
    /// Panics if called before GPU initialization.
    pub const fn program(&mut self) -> &mut RenderProgram {
        self.program
            .as_mut()
            .expect("render called before GPU configured")
    }
}

impl Systems {
    fn new(passes: &Passes<'_>, app: &CantusApp) -> Self {
        let text = TextRenderer::new(
            passes,
            lyrics::TEXT_GLYPHS + track::TEXT_GLYPHS + status::TEXT_GLYPHS + tempestas::TEXT_GLYPHS,
        );
        let tempestas = app
            .config
            .tempestas_enabled
            .then(|| TempestasPass::new(passes, &text, &app.config.timezones));
        let status = app
            .config
            .status_enabled
            .then(|| StatusPass::new(passes, &text, app.updater.clone()));
        Self {
            lyrics: LyricsPass::new(passes, &text, app.background.clone()),
            tempestas,
            status,
            track: TrackPass::new(passes, &text),
            particles: ParticlePass::new(passes),
            playhead: PlayheadPass::new(passes),
            text,
        }
    }

    fn update(
        &mut self,
        frame: &mut Frame<'_>,
        playback: &mut PlaybackState,
        last_toggle_time: &mut f32,
    ) {
        self.text.begin();
        let status_width = self
            .status
            .as_ref()
            .map_or(0.0, |status| status.pill.width() + GAP);
        frame.shared.status_width = status_width;
        frame.shared.px_per_ms = frame
            .config
            .timeline_px_per_ms(frame.shared.screen_size.x, status_width);
        frame.shared.playhead_x = frame.config.playhead_x(frame.shared.px_per_ms);
        if let Some(tempestas) = self.tempestas.as_mut() {
            tempestas.update(&mut self.text, self.status.as_mut(), frame);
        }
        if let Some(status) = self.status.as_mut() {
            status.update(&mut self.text, frame);
        }
        self.playhead.update(frame, playback, last_toggle_time);
        self.track.update(&mut self.text, playback, frame);
        self.lyrics.update(&mut self.text, playback, &self.track, frame);
        self.particles.update(&self.track, frame);
        self.text.upload();
    }
}

impl Render for Systems {
    fn prepare(&mut self) {
        Render::prepare(&mut self.lyrics);
        Render::prepare(&mut self.tempestas);
        Render::prepare(&mut self.status);
        Render::prepare(&mut self.track);
        Render::prepare(&mut self.particles);
        Render::prepare(&mut self.playhead);
    }

    fn draw<'pass>(&'pass self, pass: &mut RenderPass<'pass>) {
        Render::draw(&self.lyrics, pass);
        Render::draw(&self.tempestas, pass);
        Render::draw(&self.status, pass);
        Render::draw(&self.track, pass);
        Render::draw(&self.particles, pass);
        Render::draw(&self.playhead, pass);
    }
}

impl CantusApp {
    /// Initializes the GPU and renderer for the first configured surface.
    ///
    /// # Panics
    ///
    /// Panics if the GPU was already initialized or no compatible adapter, device, or surface configuration is available.
    pub fn initialize_gpu(&mut self, surface: Surface<'static>, width: u32, height: u32) {
        assert!(self.render.program.is_none(), "GPU initialized twice");
        let program = pollster::block_on(Program::surface(
            &self.render.instance,
            surface,
            width,
            height,
            PowerPreference::LowPower,
            isthmus::shader_module!(),
            |passes| Systems::new(passes, self),
        ))
        .expect("failed to initialize renderer");
        let info = program.adapter_info();
        tracing::info!("Using adapter: {} ({:?})", info.name, info.device_type);
        program
            .device()
            .on_uncaptured_error(Arc::new(|error| tracing::error!(%error, "uncaptured wgpu error")));
        self.render.program = Some(program);
    }

    /// Replaces a lost or recreated presentation surface without rebuilding renderer services.
    ///
    /// # Panics
    ///
    /// Panics if the renderer is not initialized or the replacement surface is incompatible.
    pub fn replace_render_surface(&mut self, surface: Surface<'static>) {
        self.render
            .program()
            .replace_surface(surface)
            .expect("replacement surface is incompatible");
    }

    pub fn logical_surface_size(&self) -> (f32, f32) {
        let extension = if self.config.tempestas_enabled {
            EXTENSION
        } else {
            lyrics::EXTENSION
        } + PANEL_OVERFLOW;
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
