use crate::image_manager::ImageManager;
use crate::interaction::InteractionState;
use crate::render::{
    BackgroundPill, FontEngine, IconInstance, ParticlesState, PlayheadUniforms, RenderState,
    ScreenUniforms,
};
use crate::text_render::TextInstance;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use wgpu::{
    Adapter, BindGroup, Buffer, Color, CommandEncoderDescriptor, Device, Instance,
    InstanceDescriptor, LoadOp, Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, StoreOp, Surface, SurfaceConfiguration, TextureViewDescriptor,
};

#[cfg(not(any(feature = "wayland", feature = "winit")))]
compile_error!("Enable at least one of the `wayland` or `winit` features.");
#[cfg(all(feature = "wayland", feature = "winit"))]
compile_error!("`wayland` and `winit` features cannot be enabled at the same time.");

mod config;
mod image_manager;
mod interaction;
mod pipelines;
mod render;
mod spotify;
mod text_render;

#[cfg(feature = "wayland")]
mod layer_shell;

#[cfg(feature = "winit")]
mod winit_app;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

const PANEL_START: f32 = 6.0;
const PANEL_EXTENSION: f32 = 12.0;

struct GpuResources {
    queue: Arc<Queue>,

    playhead_pipeline: RenderPipeline,
    background_pipeline: RenderPipeline,
    icon_pipeline: RenderPipeline,
    text_pipeline: RenderPipeline,

    uniform_buffer: Buffer,
    particles_buffer: Buffer,
    playhead_buffer: Buffer,
    playhead_bind_group: BindGroup,
    background_storage_buffer: Buffer,
    background_bind_group: BindGroup,
    icon_storage_buffer: Buffer,
    icon_bind_group: BindGroup,
    text_storage_buffer: Buffer,
    text_bind_group: BindGroup,

    images: ImageManager,
    requested_textures: HashSet<String>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            ["warn", "cantus=info", "wgpu_hal=error"].join(","),
        ))
        .with_writer(std::io::stderr)
        .init();

    spotify::init();

    #[cfg(feature = "wayland")]
    layer_shell::run();

    #[cfg(feature = "winit")]
    winit_app::run();
}

struct CantusApp {
    render_context: RenderContext,
    render_surface: Option<RenderSurface>,
    font: FontEngine,
    scale_factor: f32,
    render_state: RenderState,
    interaction: InteractionState,
    particles: ParticlesState,
    background_pills: Vec<BackgroundPill>,
    icon_pills: Vec<IconInstance>,
    text_instances: Vec<TextInstance>,
    gpu_resources: Option<GpuResources>,
    playhead_info: PlayheadUniforms,
    gpu_uniforms: ScreenUniforms,
}

impl Default for CantusApp {
    fn default() -> Self {
        Self {
            render_context: RenderContext::default(),
            render_surface: None,
            font: FontEngine::default(),
            scale_factor: 1.0,
            render_state: RenderState::default(),
            interaction: InteractionState::default(),
            particles: ParticlesState::default(),
            background_pills: Vec::new(),
            icon_pills: Vec::new(),
            text_instances: Vec::new(),
            gpu_resources: None,
            playhead_info: PlayheadUniforms::default(),
            gpu_uniforms: ScreenUniforms::default(),
        }
    }
}

impl CantusApp {
    /// Render out the app
    fn render(&mut self) {
        let rs_ptr = self.render_surface.as_ref().unwrap();
        let dev_id = rs_ptr.dev_id;

        self.background_pills.clear();
        self.icon_pills.clear();
        self.text_instances.clear();
        if let Some(gpu) = self.gpu_resources.as_mut() {
            gpu.requested_textures.clear();
        }

        let current_indices = self
            .gpu_resources
            .as_ref()
            .map(|g| g.images.url_to_index.clone())
            .unwrap_or_default();
        self.create_scene(&current_indices);

        let image_updated = if let Some(gpu) = self.gpu_resources.as_mut() {
            gpu.images.update(&gpu.requested_textures)
        } else {
            false
        };

        if image_updated {
            let indices = self
                .gpu_resources
                .as_ref()
                .unwrap()
                .images
                .url_to_index
                .clone();
            self.background_pills.clear();
            self.icon_pills.clear();
            self.text_instances.clear();
            self.create_scene(&indices);
        }

        if let Some(gpu) = self.gpu_resources.as_ref() {
            gpu.queue.write_buffer(
                &gpu.uniform_buffer,
                0,
                bytemuck::bytes_of(&self.gpu_uniforms),
            );
            gpu.queue.write_buffer(
                &gpu.particles_buffer,
                0,
                bytemuck::cast_slice(&self.particles.particles),
            );
            if !self.background_pills.is_empty() {
                gpu.queue.write_buffer(
                    &gpu.background_storage_buffer,
                    0,
                    bytemuck::cast_slice(&self.background_pills),
                );
            }
            if !self.icon_pills.is_empty() {
                gpu.queue.write_buffer(
                    &gpu.icon_storage_buffer,
                    0,
                    bytemuck::cast_slice(&self.icon_pills),
                );
            }
            if !self.text_instances.is_empty() {
                let bytes: &[u8] = bytemuck::cast_slice(&self.text_instances);
                gpu.queue.write_buffer(
                    &gpu.text_storage_buffer,
                    0,
                    &bytes[..bytes.len().min(gpu.text_storage_buffer.size() as usize)],
                );
            }
            gpu.queue.write_buffer(
                &gpu.playhead_buffer,
                0,
                bytemuck::bytes_of(&self.playhead_info),
            );
        }

        let rs = self.render_surface.as_mut().unwrap();
        let handle = &self.render_context.devices[dev_id];

        let Ok(surface_texture) = rs.surface.get_current_texture() else {
            rs.surface.configure(&handle.device, &rs.config);
            return;
        };
        let surface_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        if let Some(gpu_resources) = self.gpu_resources.as_ref() {
            let mut encoder = handle
                .device
                .create_command_encoder(&CommandEncoderDescriptor { label: None });

            {
                let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Background Pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &surface_view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color::TRANSPARENT),
                            store: StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    ..Default::default()
                });
                rpass.set_pipeline(&gpu_resources.background_pipeline);
                rpass.set_bind_group(0, &gpu_resources.background_bind_group, &[]);
                rpass.draw(0..4, 0..self.background_pills.len() as u32);
            }

            {
                let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Text Pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &surface_view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Load,
                            store: StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    ..Default::default()
                });
                rpass.set_pipeline(&gpu_resources.text_pipeline);
                rpass.set_bind_group(0, &gpu_resources.text_bind_group, &[]);
                rpass.draw(0..4, 0..self.text_instances.len() as u32);
            }

            {
                let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Icon Pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &surface_view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Load,
                            store: StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    ..Default::default()
                });
                rpass.set_pipeline(&gpu_resources.icon_pipeline);
                rpass.set_bind_group(0, &gpu_resources.icon_bind_group, &[]);
                rpass.draw(0..4, 0..self.icon_pills.len() as u32);
            }

            {
                let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Playhead Pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &surface_view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Load,
                            store: StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    ..Default::default()
                });
                rpass.set_pipeline(&gpu_resources.playhead_pipeline);
                rpass.set_bind_group(0, &gpu_resources.playhead_bind_group, &[]);
                rpass.draw(0..4, 0..1);
            }

            handle.queue.submit([encoder.finish()]);
        }
        surface_texture.present();
    }

    fn get_image_index(&mut self, url: &str, image_map: &HashMap<String, i32>) -> i32 {
        if let Some(gpu) = self.gpu_resources.as_mut() {
            gpu.requested_textures.insert(url.to_owned());
        }
        image_map.get(url).copied().unwrap_or(-1)
    }
}

struct RenderContext {
    instance: Instance,
    devices: Vec<DeviceHandle>,
}

struct DeviceHandle {
    adapter: Adapter,
    device: Arc<Device>,
    queue: Arc<Queue>,
}

struct RenderSurface {
    surface: Surface<'static>,
    dev_id: usize,
    config: SurfaceConfiguration,
}

impl Default for RenderContext {
    fn default() -> Self {
        let instance = Instance::new(&InstanceDescriptor::default());
        Self {
            instance,
            devices: Vec::new(),
        }
    }
}
