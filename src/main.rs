use crate::image_manager::ImageManager;
use crate::interaction::InteractionState;
use crate::render::{FontEngine, ParticlesState, RenderState};
use render_types::{
    BackgroundPill, IconInstance, Particle, PlayheadUniforms, ScreenUniforms, Shaders,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use vello::{
    AaConfig, AaSupport, Renderer, RendererOptions, Scene,
    peniko::color::AlphaColor,
    util::{RenderContext, RenderSurface},
};
use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, BlendComponent,
    BlendFactor, BlendOperation, BlendState, Buffer, BufferDescriptor, BufferUsages, Color,
    CommandEncoderDescriptor, CompositeAlphaMode, Device, Extent3d, FilterMode, LoadOp, Operations,
    PresentMode, Queue, RenderPassColorAttachment, RenderPassDescriptor, Sampler,
    SamplerDescriptor, StoreOp, Surface, SurfaceConfiguration, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages, TextureViewDescriptor,
};

#[cfg(not(any(feature = "wayland", feature = "winit")))]
compile_error!("Enable at least one of the `wayland` or `winit` features.");
#[cfg(all(feature = "wayland", feature = "winit"))]
compile_error!("`wayland` and `winit` features cannot be enabled at the same time.");

mod background;
mod config;
mod image_manager;
mod interaction;
mod render;
mod render_types;
mod rspotify;
mod spotify;

#[cfg(feature = "wayland")]
mod layer_shell;

#[cfg(feature = "winit")]
mod winit_app;

/// Additional height allocated for extended content.
const PANEL_START: f64 = 6.0;
const PANEL_EXTENSION: f64 = 12.0;

pub struct GpuResources {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub shaders: Shaders,
    pub uniform_buffer: Buffer,
    pub storage_buffer: Buffer,
    pub particle_bind_group: BindGroup,
    pub bg_storage_buffer: Buffer,
    pub playhead_buffer: Buffer,
    pub playhead_bind_group: BindGroup,
    pub bg_bind_group: Option<BindGroup>,
    pub icon_storage_buffer: Buffer,
    pub icon_bind_group: Option<BindGroup>,
    pub bg_sampler: Sampler,
    pub images: ImageManager,
    pub requested_textures: HashSet<String>,
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

pub struct CantusApp {
    render_context: RenderContext,
    render_surface: Option<RenderSurface<'static>>,
    render_device: Option<Renderer>,
    scene: Scene,
    font: FontEngine,
    scale_factor: f64,
    render_state: RenderState,
    interaction: InteractionState,
    particles: ParticlesState,
    background_pills: Vec<BackgroundPill>,
    icon_pills: Vec<IconInstance>,
    gpu_resources: Option<GpuResources>,
    playhead_info: Option<PlayheadUniforms>,
    gpu_uniforms: Option<ScreenUniforms>,
}

impl Default for CantusApp {
    fn default() -> Self {
        Self {
            render_context: RenderContext::new(),
            render_surface: None,
            render_device: None,
            scene: Scene::new(),
            font: FontEngine::default(),
            scale_factor: 1.0,
            render_state: RenderState::default(),
            interaction: InteractionState::default(),
            particles: ParticlesState::default(),
            background_pills: Vec::new(),
            icon_pills: Vec::new(),
            gpu_resources: None,
            playhead_info: None,
            gpu_uniforms: None,
        }
    }
}

impl CantusApp {
    fn configure_render_surface(&mut self, surface: Surface<'static>, width: u32, height: u32) {
        let dev_id = pollster::block_on(self.render_context.device(Some(&surface)))
            .expect("No compatible device found");
        let device_handle = &self.render_context.devices[dev_id];
        let capabilities = surface.get_capabilities(device_handle.adapter());

        let format = TextureFormat::Rgba8Unorm;
        assert!(
            capabilities.formats.contains(&format),
            "No compatible surface format found"
        );
        let alpha_mode = [
            CompositeAlphaMode::PreMultiplied,
            CompositeAlphaMode::PostMultiplied,
        ]
        .into_iter()
        .find(|mode| capabilities.alpha_modes.contains(mode))
        .unwrap_or(CompositeAlphaMode::Auto);

        let target_texture = device_handle.device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            format,
            view_formats: &[],
        });
        let target_view = target_texture.create_view(&TextureViewDescriptor::default());
        let render_surface = RenderSurface {
            surface,
            dev_id,
            format,
            target_texture,
            target_view,
            config: SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format,
                width,
                height,
                present_mode: PresentMode::AutoVsync,
                desired_maximum_frame_latency: 2,
                alpha_mode,
                view_formats: vec![],
            },
            blitter: wgpu::util::TextureBlitterBuilder::new(&device_handle.device, format)
                .blend_state(BlendState {
                    color: BlendComponent {
                        src_factor: BlendFactor::SrcAlpha,
                        dst_factor: BlendFactor::OneMinusSrcAlpha,
                        operation: BlendOperation::Add,
                    },
                    alpha: BlendComponent {
                        src_factor: BlendFactor::One,
                        dst_factor: BlendFactor::OneMinusSrcAlpha,
                        operation: BlendOperation::Add,
                    },
                })
                .build(),
        };
        render_surface
            .surface
            .configure(&device_handle.device, &render_surface.config);

        self.render_device = Some(
            Renderer::new(
                &device_handle.device,
                RendererOptions {
                    use_cpu: false,
                    antialiasing_support: AaSupport::area_only(),
                    num_init_threads: None,
                    pipeline_cache: None,
                },
            )
            .expect("Failed to create renderer"),
        );

        let shaders = Shaders::new(&device_handle.device, format);
        let uniform_buffer = device_handle.device.create_buffer(&BufferDescriptor {
            label: Some("Uniforms"),
            size: std::mem::size_of::<ScreenUniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let storage_buffer = device_handle.device.create_buffer(&BufferDescriptor {
            label: Some("Particles"),
            size: (std::mem::size_of::<Particle>() * 64) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let particle_bind_group = device_handle
            .device
            .create_bind_group(&BindGroupDescriptor {
                label: Some("Particle BG"),
                layout: &shaders.bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: storage_buffer.as_entire_binding(),
                    },
                ],
            });

        let playhead_buffer = device_handle.device.create_buffer(&BufferDescriptor {
            label: Some("Playhead Info"),
            size: std::mem::size_of::<PlayheadUniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let playhead_bind_group = device_handle
            .device
            .create_bind_group(&BindGroupDescriptor {
                label: Some("Playhead BG"),
                layout: &shaders.playhead_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: playhead_buffer.as_entire_binding(),
                    },
                ],
            });

        let bg_storage_buffer = device_handle.device.create_buffer(&BufferDescriptor {
            label: Some("BG Pills"),
            size: (std::mem::size_of::<BackgroundPill>() * 256) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let icon_storage_buffer = device_handle.device.create_buffer(&BufferDescriptor {
            label: Some("Icons"),
            size: (std::mem::size_of::<IconInstance>() * 256) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bg_sampler = device_handle.device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let device = Arc::new(device_handle.device.clone());
        let queue = Arc::new(device_handle.queue.clone());
        self.gpu_resources = Some(GpuResources {
            device: device.clone(),
            queue: queue.clone(),
            shaders,
            uniform_buffer,
            storage_buffer,
            particle_bind_group,
            bg_storage_buffer,
            playhead_buffer,
            playhead_bind_group,
            bg_bind_group: None,
            icon_storage_buffer,
            icon_bind_group: None,
            bg_sampler,
            images: ImageManager::new(device, queue),
            requested_textures: HashSet::new(),
        });
        self.render_surface = Some(render_surface);
    }

    /// Render out the app
    fn render(&mut self) {
        let rs_ptr = self.render_surface.as_ref().unwrap();
        let dev_id = rs_ptr.dev_id;

        self.scene.reset();
        self.background_pills.clear();
        self.icon_pills.clear();
        self.playhead_info = None;
        if let Some(gpu) = self.gpu_resources.as_mut() {
            gpu.requested_textures.clear();
        }

        let current_indices = self
            .gpu_resources
            .as_ref()
            .map(|g| g.images.url_to_index.clone())
            .unwrap_or_default();
        self.create_scene(&current_indices);

        let next_indices = if let Some(gpu) = self.gpu_resources.as_mut()
            && gpu.images.update(&gpu.requested_textures)
        {
            Some(gpu.images.url_to_index.clone())
        } else {
            None
        };

        if let Some(idx) = next_indices {
            self.scene.reset();
            self.background_pills.clear();
            self.icon_pills.clear();
            self.create_scene(&idx);

            if let Some(gpu) = self.gpu_resources.as_mut()
                && let Some(view) = gpu.images.create_view()
            {
                let make_bg = |label, layout, storage: BindingResource| {
                    gpu.device.create_bind_group(&BindGroupDescriptor {
                        label: Some(label),
                        layout,
                        entries: &[
                            BindGroupEntry {
                                binding: 0,
                                resource: gpu.uniform_buffer.as_entire_binding(),
                            },
                            BindGroupEntry {
                                binding: 1,
                                resource: storage,
                            },
                            BindGroupEntry {
                                binding: 2,
                                resource: BindingResource::TextureView(&view),
                            },
                            BindGroupEntry {
                                binding: 3,
                                resource: BindingResource::Sampler(&gpu.bg_sampler),
                            },
                        ],
                    })
                };
                gpu.bg_bind_group = Some(make_bg(
                    "BG BG",
                    &gpu.shaders.bg_bind_group_layout,
                    gpu.bg_storage_buffer.as_entire_binding(),
                ));
                gpu.icon_bind_group = Some(make_bg(
                    "Icon BG",
                    &gpu.shaders.icon_bind_group_layout,
                    gpu.icon_storage_buffer.as_entire_binding(),
                ));
            }
        }

        if let Some(gpu) = self.gpu_resources.as_ref() {
            if let Some(u) = self.gpu_uniforms.as_ref() {
                gpu.queue
                    .write_buffer(&gpu.uniform_buffer, 0, bytemuck::bytes_of(u));
            }
            gpu.queue.write_buffer(
                &gpu.storage_buffer,
                0,
                bytemuck::cast_slice(&self.particles.particles),
            );
            if !self.background_pills.is_empty() {
                gpu.queue.write_buffer(
                    &gpu.bg_storage_buffer,
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
            if let Some(p) = self.playhead_info.as_ref() {
                gpu.queue
                    .write_buffer(&gpu.playhead_buffer, 0, bytemuck::bytes_of(p));
            }
        }

        let rs = self.render_surface.as_mut().unwrap();
        let handle = &self.render_context.devices[dev_id];
        self.render_device
            .as_mut()
            .unwrap()
            .render_to_texture(
                &handle.device,
                &handle.queue,
                &self.scene,
                &rs.target_view,
                &vello::RenderParams {
                    base_color: AlphaColor::from_rgba8(0, 0, 0, 0),
                    width: rs.config.width,
                    height: rs.config.height,
                    antialiasing_method: AaConfig::Area,
                },
            )
            .unwrap();

        let Ok(surface_texture) = rs.surface.get_current_texture() else {
            rs.surface.configure(&handle.device, &rs.config);
            return;
        };
        let surface_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        if let Some(gpu) = self.gpu_resources.as_ref() {
            let mut encoder = handle
                .device
                .create_command_encoder(&CommandEncoderDescriptor { label: None });
            if let Some(bg_bind_group) = &gpu.bg_bind_group {
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
                rpass.set_pipeline(&gpu.shaders.bg_pipeline);
                rpass.set_bind_group(0, bg_bind_group, &[]);
                rpass.draw(0..4, 0..self.background_pills.len() as u32);
            }
            if let Some(icon_bind_group) = &gpu.icon_bind_group {
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
                rpass.set_pipeline(&gpu.shaders.icon_pipeline);
                rpass.set_bind_group(0, icon_bind_group, &[]);
                rpass.draw(0..4, 0..self.icon_pills.len() as u32);
            }
            rs.blitter
                .copy(&handle.device, &mut encoder, &rs.target_view, &surface_view);
            {
                let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Particle Pass"),
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
                rpass.set_pipeline(&gpu.shaders.pipeline);
                rpass.set_bind_group(0, &gpu.particle_bind_group, &[]);
                rpass.draw(0..4, 0..1);
            }
            if self.playhead_info.is_some() {
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
                rpass.set_pipeline(&gpu.shaders.playhead_pipeline);
                rpass.set_bind_group(0, &gpu.playhead_bind_group, &[]);
                rpass.draw(0..4, 0..1);
            }
            handle.queue.submit([encoder.finish()]);
        }
        surface_texture.present();
    }

    pub fn get_image_index(&mut self, url: &str, image_map: &HashMap<String, i32>) -> i32 {
        if let Some(gpu) = self.gpu_resources.as_mut() {
            gpu.requested_textures.insert(url.to_owned());
        }
        image_map.get(url).copied().unwrap_or(-1)
    }
}

fn lerpf32(t: f32, v0: f32, v1: f32) -> f32 {
    v0 + t * (v1 - v0)
}
