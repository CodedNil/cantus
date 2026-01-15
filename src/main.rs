use crate::interaction::InteractionState;
use crate::pipelines::{IMAGE_SIZE, MAX_TEXTURE_LAYERS};
use crate::render::{
    BackgroundPill, IconInstance, Particle, PlayheadUniforms, RenderState, ScreenUniforms,
};
use crate::spotify::IMAGES_CACHE;
use crate::text_render::TextRenderer;
use std::collections::HashMap;
use wgpu::{
    BindGroup, Buffer, Color, CommandEncoderDescriptor, Device, Instance, LoadOp, Operations,
    Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, StoreOp, Surface,
    SurfaceConfiguration, Texture, TextureViewDescriptor,
};

mod config;
mod interaction;
mod layer_shell;
mod pipelines;
mod render;
mod spotify;
mod text_render;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

const PANEL_START: f32 = 6.0;
const PANEL_EXTENSION: f32 = 12.0;

struct GpuResources {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,

    // Pipelines
    playhead_pipeline: RenderPipeline,
    background_pipeline: RenderPipeline,
    icon_pipeline: RenderPipeline,
    particle_pipeline: RenderPipeline,

    // Uniform/Storage Buffers
    uniform_buffer: Buffer,
    particles_buffer: Buffer,
    playhead_buffer: Buffer,
    background_storage_buffer: Buffer,
    icon_storage_buffer: Buffer,

    // Bind Groups
    playhead_bind_group: BindGroup,
    background_bind_group: BindGroup,
    icon_bind_group: BindGroup,
    particle_bind_group: BindGroup,

    // Image Management
    texture_array: Texture,
    url_to_image_index: HashMap<String, (i32, bool)>, // (index, used_this_frame)
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            ["warn", "cantus=info", "wgpu_hal=error"].join(","),
        ))
        .with_writer(std::io::stderr)
        .init();

    spotify::init();

    layer_shell::run();
}

struct CantusApp {
    // Core Graphics
    instance: Instance,
    gpu_resources: Option<GpuResources>,

    // Application State
    render_state: RenderState,
    interaction: InteractionState,
    particles: [Particle; 64],
    particles_accumulator: f32,
    scale_factor: f32,

    // Scene & Resources
    text_renderer: Option<TextRenderer>,
    screen_uniforms: ScreenUniforms,
    background_pills: Vec<BackgroundPill>,
    icon_pills: Vec<IconInstance>,
    playhead_info: PlayheadUniforms,
}

impl Default for CantusApp {
    fn default() -> Self {
        Self {
            instance: Instance::new(&wgpu::InstanceDescriptor::default()),
            gpu_resources: None,

            render_state: RenderState::default(),
            interaction: InteractionState::default(),
            particles: [Particle::default(); 64],
            particles_accumulator: 0.0,
            scale_factor: 1.0,

            text_renderer: None,
            screen_uniforms: ScreenUniforms::default(),
            background_pills: Vec::new(),
            icon_pills: Vec::new(),
            playhead_info: PlayheadUniforms::default(),
        }
    }
}

impl CantusApp {
    fn render(&mut self) {
        if self.gpu_resources.is_none() {
            return;
        }

        self.background_pills.clear();
        self.icon_pills.clear();

        // Reset image usage
        if let Some(gpu) = self.gpu_resources.as_mut() {
            for (_, used) in gpu.url_to_image_index.values_mut() {
                *used = false;
            }
        }

        self.create_scene();

        // Prune unused images
        if let Some(gpu) = self.gpu_resources.as_mut() {
            gpu.url_to_image_index.retain(|_, (_, used)| *used);
        }

        // Write the buffers
        let gpu = self.gpu_resources.as_mut().unwrap();
        gpu.queue.write_buffer(
            &gpu.uniform_buffer,
            0,
            bytemuck::bytes_of(&self.screen_uniforms),
        );
        gpu.queue.write_buffer(
            &gpu.particles_buffer,
            0,
            bytemuck::cast_slice(&self.particles),
        );
        gpu.queue.write_buffer(
            &gpu.playhead_buffer,
            0,
            bytemuck::bytes_of(&self.playhead_info),
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

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::TRANSPARENT),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            if !self.background_pills.is_empty() {
                rpass.set_pipeline(&gpu.background_pipeline);
                rpass.set_bind_group(0, &gpu.background_bind_group, &[]);
                rpass.draw(0..4, 0..self.background_pills.len() as u32);
            }

            if let Some(text_renderer) = &mut self.text_renderer {
                text_renderer.draw(
                    &gpu.device,
                    &gpu.queue,
                    &mut rpass,
                    gpu.surface_config.width,
                    gpu.surface_config.height,
                    self.scale_factor,
                );
            }

            if !self.icon_pills.is_empty() {
                rpass.set_pipeline(&gpu.icon_pipeline);
                rpass.set_bind_group(0, &gpu.icon_bind_group, &[]);
                rpass.draw(0..4, 0..self.icon_pills.len() as u32);
            }

            rpass.set_pipeline(&gpu.particle_pipeline);
            rpass.set_bind_group(0, &gpu.particle_bind_group, &[]);
            rpass.draw(0..4, 0..64);

            rpass.set_pipeline(&gpu.playhead_pipeline);
            rpass.set_bind_group(0, &gpu.playhead_bind_group, &[]);
            rpass.draw(0..4, 0..1);
        }

        gpu.queue.submit([encoder.finish()]);
        surface_texture.present();
    }

    fn get_image_index(&mut self, url: &str) -> i32 {
        let Some(gpu) = self.gpu_resources.as_mut() else {
            return -1;
        };

        if let Some(entry) = gpu.url_to_image_index.get_mut(url) {
            entry.1 = true;
            return entry.0;
        }

        if let Some(img_ref) = IMAGES_CACHE.get(url)
            && let Some(image) = img_ref.as_ref()
        {
            let mut used_slots = vec![false; MAX_TEXTURE_LAYERS as usize];
            for (idx, _) in gpu.url_to_image_index.values() {
                used_slots[*idx as usize] = true;
            }

            if let Some(slot) = used_slots.iter().position(|&used| !used) {
                gpu.queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &gpu.texture_array,
                        mip_level: 0,
                        aspect: wgpu::TextureAspect::All,
                        origin: wgpu::Origin3d {
                            x: 0,
                            y: 0,
                            z: slot as u32,
                        },
                    },
                    image.as_raw(),
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * IMAGE_SIZE),
                        rows_per_image: Some(IMAGE_SIZE),
                    },
                    wgpu::Extent3d {
                        width: IMAGE_SIZE,
                        height: IMAGE_SIZE,
                        depth_or_array_layers: 1,
                    },
                );

                gpu.url_to_image_index
                    .insert(url.to_owned(), (slot as i32, true));
                return slot as i32;
            }
        }
        -1
    }
}
