use crate::{
    interaction::InteractionState,
    render::{FontEngine, ParticlesState, RenderState},
};
use anyhow::Result;
use std::collections::HashMap;
use tracing_subscriber::EnvFilter;
use vello::{
    AaConfig, AaSupport, Renderer, RendererOptions, Scene,
    peniko::color::palette,
    util::{RenderContext, RenderSurface},
    wgpu::{
        BlendComponent, BlendFactor, BlendOperation, BlendState, CommandEncoderDescriptor,
        CompositeAlphaMode, Instance, InstanceDescriptor, PresentMode, Surface,
        TextureViewDescriptor, util::TextureBlitterBuilder,
    },
};
use wgpu::Backends;

#[cfg(not(any(feature = "wayland", feature = "winit")))]
compile_error!("Enable at least one of the `wayland` or `winit` features.");

#[cfg(all(feature = "wayland", feature = "winit"))]
compile_error!("`wayland` and `winit` features cannot be enabled at the same time.");

mod background;
mod config;
mod interaction;
mod render;
mod spotify;

#[cfg(feature = "wayland")]
mod layer_shell;

#[cfg(feature = "winit")]
mod winit_app;

/// Additional height allocated for extended content.
const PANEL_HEIGHT_EXTENSION: f64 = 10.0;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(
            ["warn", "cantus=info", "wgpu_hal=error"].join(","),
        ))
        .init();

    spotify::init();

    #[cfg(feature = "wayland")]
    layer_shell::run();

    #[cfg(feature = "winit")]
    winit_app::run();
}

struct CantusApp {
    render_context: RenderContext,
    render_surface: Option<RenderSurface<'static>>,
    render_devices: HashMap<usize, Renderer>,
    scene: Scene,
    font: FontEngine,
    scale_factor: f64,
    render_state: RenderState,
    interaction: InteractionState,
    particles: ParticlesState,
}

impl Default for CantusApp {
    fn default() -> Self {
        let mut render_context = RenderContext::new();
        render_context.instance = Instance::new(&InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        Self {
            render_context,
            render_surface: None,
            render_devices: HashMap::new(),
            scene: Scene::new(),
            font: FontEngine::default(),
            scale_factor: 1.0,
            render_state: RenderState::default(),
            interaction: InteractionState::default(),
            particles: ParticlesState::default(),
        }
    }
}
impl CantusApp {
    fn configure_render_surface(
        &mut self,
        surface: Surface<'static>,
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
        if !self.render_devices.contains_key(&rs.dev_id) {
            self.render_devices.insert(
                rs.dev_id,
                Renderer::new(
                    &self.render_context.devices[rs.dev_id].device,
                    RendererOptions {
                        use_cpu: false,
                        antialiasing_support: AaSupport::area_only(),
                        num_init_threads: None,
                        pipeline_cache: None,
                    },
                )?,
            );
        }
        self.render_surface = Some(rs);
        Ok(())
    }

    /// Try to render out the app
    fn render(&mut self) {
        if self.render_surface.is_none() {
            return;
        }

        self.scene.reset();
        self.create_scene();

        let dev_id = self.render_surface.as_ref().unwrap().dev_id;
        let handle = &self.render_context.devices[dev_id];
        let render_surface = self.render_surface.as_mut().unwrap();
        self.render_devices
            .get_mut(&dev_id)
            .unwrap()
            .render_to_texture(
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
            )
            .expect("failed to render to surface");

        let Ok(surface_texture) = render_surface.surface.get_current_texture() else {
            render_surface.surface.configure(
                &self.render_context.devices[render_surface.dev_id].device,
                &render_surface.config,
            );
            return;
        };

        let mut encoder = handle
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Surface Blit"),
            });
        render_surface.blitter.copy(
            &handle.device,
            &mut encoder,
            &render_surface.target_view,
            &surface_texture
                .texture
                .create_view(&TextureViewDescriptor::default()),
        );

        handle.queue.submit([encoder.finish()]);
        surface_texture.present();
    }
}

fn lerpf64(t: f64, v0: f64, v1: f64) -> f64 {
    (1.0 - t) * v0 + t * v1
}
