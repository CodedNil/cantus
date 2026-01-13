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
    BindGroup, Buffer, Color, CommandEncoderDescriptor, Device, Instance, LoadOp, Operations,
    Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, StoreOp, Surface,
    SurfaceConfiguration, Texture, TextureViewDescriptor,
};

#[cfg(not(any(feature = "wayland", feature = "winit")))]
compile_error!("Enable at least one of the `wayland` or `winit` features.");
#[cfg(all(feature = "wayland", feature = "winit"))]
compile_error!("`wayland` and `winit` features cannot be enabled at the same time.");

mod config;
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
    device: Arc<Device>,
    queue: Arc<Queue>,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,

    // Pipelines
    playhead_pipeline: RenderPipeline,
    background_pipeline: RenderPipeline,
    icon_pipeline: RenderPipeline,
    text_pipeline: RenderPipeline,

    // Uniform/Storage Buffers
    uniform_buffer: Buffer,
    particles_buffer: Buffer,
    playhead_buffer: Buffer,
    background_storage_buffer: Buffer,
    icon_storage_buffer: Buffer,
    text_storage_buffer: Buffer,

    // Bind Groups
    playhead_bind_group: BindGroup,
    background_bind_group: BindGroup,
    icon_bind_group: BindGroup,
    text_bind_group: BindGroup,

    // Image Management
    texture_array: Texture,
    last_images_set: HashSet<String>,
    url_to_image_index: HashMap<String, i32>,
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
    // Core Graphics
    instance: Instance,
    gpu_resources: Option<GpuResources>,

    // Application State
    render_state: RenderState,
    interaction: InteractionState,
    particles: ParticlesState,
    scale_factor: f32,

    // Scene & Resources
    font: FontEngine,
    screen_uniforms: ScreenUniforms,
    background_pills: Vec<BackgroundPill>,
    icon_pills: Vec<IconInstance>,
    text_instances: Vec<TextInstance>,
    playhead_info: PlayheadUniforms,
}

impl Default for CantusApp {
    fn default() -> Self {
        Self {
            instance: Instance::new(&wgpu::InstanceDescriptor::default()),
            gpu_resources: None,

            render_state: RenderState::default(),
            interaction: InteractionState::default(),
            particles: ParticlesState::default(),
            scale_factor: 1.0,

            font: FontEngine::default(),
            background_pills: Vec::new(),
            icon_pills: Vec::new(),
            text_instances: Vec::new(),

            playhead_info: PlayheadUniforms::default(),
            screen_uniforms: ScreenUniforms::default(),
        }
    }
}

impl CantusApp {
    fn render(&mut self) {
        self.background_pills.clear();
        self.icon_pills.clear();
        self.text_instances.clear();
        if let Some(gpu) = self.gpu_resources.as_mut() {
            gpu.requested_textures.clear();
        }

        let current_indices = self
            .gpu_resources
            .as_ref()
            .map(|g| g.url_to_image_index.clone())
            .unwrap_or_default();
        self.create_scene(&current_indices);

        if self
            .gpu_resources
            .as_mut()
            .is_some_and(GpuResources::update_textures)
        {
            let indices = self
                .gpu_resources
                .as_ref()
                .unwrap()
                .url_to_image_index
                .clone();
            self.background_pills.clear();
            self.icon_pills.clear();
            self.text_instances.clear();
            self.create_scene(&indices);
        }

        if let Some(gpu) = self.gpu_resources.as_ref() {
            let q = &gpu.queue;
            q.write_buffer(
                &gpu.uniform_buffer,
                0,
                bytemuck::bytes_of(&self.screen_uniforms),
            );
            q.write_buffer(
                &gpu.particles_buffer,
                0,
                bytemuck::cast_slice(&self.particles.particles),
            );
            q.write_buffer(
                &gpu.playhead_buffer,
                0,
                bytemuck::bytes_of(&self.playhead_info),
            );
            if !self.background_pills.is_empty() {
                q.write_buffer(
                    &gpu.background_storage_buffer,
                    0,
                    bytemuck::cast_slice(&self.background_pills),
                );
            }
            if !self.icon_pills.is_empty() {
                q.write_buffer(
                    &gpu.icon_storage_buffer,
                    0,
                    bytemuck::cast_slice(&self.icon_pills),
                );
            }
            if !self.text_instances.is_empty() {
                let bytes: &[u8] = bytemuck::cast_slice(&self.text_instances);
                q.write_buffer(
                    &gpu.text_storage_buffer,
                    0,
                    &bytes[..bytes.len().min(gpu.text_storage_buffer.size() as usize)],
                );
            }
        }

        let gpu = self.gpu_resources.as_mut().expect("No gpu resources");
        let Ok(surface_texture) = gpu.surface.get_current_texture() else {
            gpu.surface.configure(&gpu.device, &gpu.surface_config);
            return;
        };
        let surface_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = gpu
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());
        let passes = [
            (
                "Background",
                &gpu.background_pipeline,
                &gpu.background_bind_group,
                self.background_pills.len() as u32,
                LoadOp::Clear(Color::TRANSPARENT),
            ),
            (
                "Text",
                &gpu.text_pipeline,
                &gpu.text_bind_group,
                self.text_instances.len() as u32,
                LoadOp::Load,
            ),
            (
                "Icon",
                &gpu.icon_pipeline,
                &gpu.icon_bind_group,
                self.icon_pills.len() as u32,
                LoadOp::Load,
            ),
            (
                "Play",
                &gpu.playhead_pipeline,
                &gpu.playhead_bind_group,
                1,
                LoadOp::Load,
            ),
        ];

        for (label, pipe, bg, count, load) in passes {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some(label),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: Operations {
                        load,
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                ..Default::default()
            });
            rpass.set_pipeline(pipe);
            rpass.set_bind_group(0, bg, &[]);
            rpass.draw(0..4, 0..count);
        }
        gpu.queue.submit([encoder.finish()]);
        surface_texture.present();
    }

    fn get_image_index(&mut self, url: &str, image_map: &HashMap<String, i32>) -> i32 {
        if let Some(gpu) = self.gpu_resources.as_mut() {
            gpu.requested_textures.insert(url.to_owned());
        }
        image_map.get(url).copied().unwrap_or(-1)
    }
}
