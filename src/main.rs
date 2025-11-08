use crate::{
    background::WarpBackground,
    interaction::InteractionState,
    render::{FontEngine, ParticlesState, RenderState},
};
use anyhow::Result;
use std::{
    collections::{HashMap, hash_map},
    time::Instant,
};
use tracing_subscriber::EnvFilter;
use vello::{
    AaConfig, Renderer, RendererOptions, Scene,
    peniko::color::palette,
    util::{DeviceHandle, RenderContext, RenderSurface},
    wgpu::{
        BlendComponent, BlendFactor, BlendOperation, BlendState, CommandEncoderDescriptor,
        CompositeAlphaMode, InstanceDescriptor, PollType, PresentMode, TextureViewDescriptor,
        util::TextureBlitterBuilder,
    },
};

#[cfg(not(any(feature = "wayland", feature = "winit")))]
compile_error!("Enable at least one of the `wayland` or `winit` features.");

#[cfg(all(feature = "wayland", feature = "winit"))]
compile_error!("`wayland` and `winit` features cannot be enabled at the same time.");

mod background;
mod interaction;
mod render;
mod spotify;

#[cfg(feature = "wayland")]
mod layer_shell;

#[cfg(feature = "winit")]
mod winit_app;

/// Target width of the panel in logical pixels.
const PANEL_WIDTH: f64 = 1050.0;
/// The width on the left where previous tracks are shown
const HISTORY_WIDTH: f64 = 100.0;
/// Base height of the panel in logical pixels.
const PANEL_HEIGHT_BASE: f64 = 40.0;
/// Additional height allocated for extended content.
const PANEL_HEIGHT_EXTENSION: f64 = 10.0;
/// Total height of the panel in logical pixels.
const PANEL_HEIGHT: f64 = PANEL_HEIGHT_BASE + PANEL_HEIGHT_EXTENSION;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().unwrap();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(
            ["warn", "cantus=info", "wgpu_hal=error"].join(","),
        ))
        .init();

    spotify::init().await;

    #[cfg(feature = "wayland")]
    layer_shell::run();

    #[cfg(feature = "winit")]
    winit_app::run();
}

struct CantusApp {
    render_context: RenderContext,
    render_surface: Option<RenderSurface<'static>>,
    render_devices: HashMap<usize, RenderDevice>,
    scene: Scene,
    font: FontEngine,
    scale_factor: f64,
    #[cfg(feature = "wayland")]
    is_configured: bool,
    #[cfg(feature = "wayland")]
    should_exit: bool,
    time_origin: Instant,
    frame_index: u64,
    render_state: RenderState,
    interaction: InteractionState,
    particles: ParticlesState,
}

impl Default for CantusApp {
    fn default() -> Self {
        let mut render_context = RenderContext::new();
        render_context.instance = vello::wgpu::Instance::new(&InstanceDescriptor {
            backends: vello::wgpu::Backends::VULKAN,
            ..Default::default()
        });

        Self {
            render_context,
            render_surface: None,
            render_devices: HashMap::new(),
            scene: Scene::new(),
            font: FontEngine::default(),
            scale_factor: 1.0,
            #[cfg(feature = "wayland")]
            is_configured: false,
            #[cfg(feature = "wayland")]
            should_exit: false,
            time_origin: Instant::now(),
            frame_index: 0,
            render_state: RenderState::default(),
            interaction: InteractionState::default(),
            particles: ParticlesState::default(),
        }
    }
}
impl CantusApp {
    fn configure_render_surface(
        &mut self,
        surface: vello::wgpu::Surface<'static>,
        width: u32,
        height: u32,
        present_mode: PresentMode,
    ) -> Result<()> {
        let mut rs = pollster::block_on(self.render_context.create_render_surface(
            surface,
            width,
            height,
            present_mode,
        ))?;
        let device_handle = &self.render_context.devices[rs.dev_id];
        let alpha_modes = rs
            .surface
            .get_capabilities(device_handle.adapter())
            .alpha_modes;
        let alpha_mode = [
            CompositeAlphaMode::PreMultiplied,
            CompositeAlphaMode::PostMultiplied,
        ]
        .into_iter()
        .find(|mode| alpha_modes.contains(mode))
        .or_else(|| alpha_modes.first().copied())
        .unwrap_or(CompositeAlphaMode::Auto);
        rs.config.alpha_mode = alpha_mode;
        if alpha_mode != CompositeAlphaMode::PostMultiplied {
            rs.blitter = TextureBlitterBuilder::new(&device_handle.device, rs.config.format)
                .blend_state(BlendState {
                    color: BlendComponent {
                        src_factor: BlendFactor::SrcAlpha,
                        dst_factor: BlendFactor::Zero,
                        operation: BlendOperation::Add,
                    },
                    alpha: BlendComponent {
                        src_factor: BlendFactor::One,
                        dst_factor: BlendFactor::Zero,
                        operation: BlendOperation::Add,
                    },
                })
                .build();
        }
        rs.surface.configure(&device_handle.device, &rs.config);
        self.render_surface = Some(rs);
        Ok(())
    }

    fn render<G>(&mut self, on_surface_lost: G) -> Result<bool>
    where
        G: FnOnce(),
    {
        self.frame_index = self.frame_index.wrapping_add(1);

        let Some(render_surface) = self.render_surface.take() else {
            return Ok(false);
        };

        let dev_id = render_surface.dev_id;
        if let hash_map::Entry::Vacant(entry) = self.render_devices.entry(dev_id) {
            entry.insert(RenderDevice::new(&self.render_context.devices[dev_id])?);
        }

        self.scene.reset();
        self.create_scene(dev_id);

        let handle = &self.render_context.devices[dev_id];
        let bundle = self
            .render_devices
            .get_mut(&dev_id)
            .expect("render device must exist");
        bundle.renderer.render_to_texture(
            &handle.device,
            &handle.queue,
            &self.scene,
            &render_surface.target_view,
            &vello::RenderParams {
                base_color: palette::css::TRANSPARENT,
                width: render_surface.config.width,
                height: render_surface.config.height,
                antialiasing_method: AaConfig::Area,
            },
        )?;

        let Ok(acquired) = render_surface.surface.get_current_texture() else {
            self.render_surface = None;
            on_surface_lost();
            return Ok(false);
        };

        let mut encoder = handle
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Cantus blit"),
            });
        render_surface.blitter.copy(
            &handle.device,
            &mut encoder,
            &render_surface.target_view,
            &acquired
                .texture
                .create_view(&TextureViewDescriptor::default()),
        );

        handle.queue.submit([encoder.finish()]);
        acquired.present();
        handle.device.poll(PollType::Poll)?;

        self.render_surface = Some(render_surface);
        Ok(true)
    }
}

struct RenderDevice {
    renderer: Renderer,
    background: WarpBackground,
}

impl RenderDevice {
    fn new(handle: &DeviceHandle) -> Result<Self> {
        Ok(Self {
            renderer: Renderer::new(&handle.device, RendererOptions::default())?,
            background: WarpBackground::new(&handle.device),
        })
    }
}
