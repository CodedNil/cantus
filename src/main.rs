use crate::image_manager::ImageManager;
use crate::interaction::InteractionState;
use crate::render::{
    BackgroundPill, FontEngine, IconInstance, Particle, ParticlesState, PlayheadUniforms,
    RenderState, ScreenUniforms, Shaders,
};
use crate::text_render::TextInstance;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use wgpu::{
    Adapter, AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, Buffer,
    BufferDescriptor, BufferUsages, Color, CommandEncoderDescriptor, CompositeAlphaMode, Device,
    DeviceDescriptor, ExperimentalFeatures, Extent3d, Features, FilterMode, Instance,
    InstanceDescriptor, Limits, LoadOp, MemoryHints, Operations, PowerPreference, PresentMode,
    Queue, RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, Sampler,
    SamplerDescriptor, StoreOp, Surface, SurfaceConfiguration, TexelCopyBufferLayout,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor, Trace,
};

#[cfg(not(any(feature = "wayland", feature = "winit")))]
compile_error!("Enable at least one of the `wayland` or `winit` features.");
#[cfg(all(feature = "wayland", feature = "winit"))]
compile_error!("`wayland` and `winit` features cannot be enabled at the same time.");

mod config;
mod image_manager;
mod interaction;
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
    shaders: Shaders,
    uniform_buffer: Buffer,
    particles_buffer: Buffer,
    playhead_buffer: Buffer,
    playhead_bind_group: BindGroup,
    bg_storage_buffer: Buffer,
    bg_bind_group: Option<BindGroup>,
    icon_storage_buffer: Buffer,
    icon_bind_group: Option<BindGroup>,
    atlas_view: TextureView,
    image_view: TextureView,
    text_storage_buffer: Buffer,
    text_bind_group: Option<BindGroup>,
    bg_sampler: Sampler,
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
    fn configure_render_surface(&mut self, surface: Surface<'static>, width: u32, height: u32) {
        let adapter = pollster::block_on(self.render_context.instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ))
        .expect("No compatible adapter found");
        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::default(),
            required_limits: Limits::defaults(),
            memory_hints: MemoryHints::MemoryUsage,
            trace: Trace::Off,
            experimental_features: ExperimentalFeatures::disabled(),
        }))
        .expect("No compatible device found");
        self.render_context.devices.push(DeviceHandle {
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
        });
        let dev_id = self.render_context.devices.len() - 1;
        let device_handle = &self.render_context.devices[dev_id];
        let capabilities = surface.get_capabilities(&device_handle.adapter);

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

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode,
            view_formats: vec![],
        };
        surface.configure(&device_handle.device, &config);

        let render_surface = RenderSurface {
            surface,
            dev_id,
            config,
        };

        let shaders = Shaders::new(&device_handle.device, format);
        let uniform_buffer = device_handle.device.create_buffer(&BufferDescriptor {
            label: Some("Uniforms"),
            size: std::mem::size_of::<ScreenUniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let particles_buffer = device_handle.device.create_buffer(&BufferDescriptor {
            label: Some("Particles"),
            size: (std::mem::size_of::<Particle>() * 64) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
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
                        resource: particles_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 2,
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

        let atlas_texture = device_handle.device.create_texture(&TextureDescriptor {
            label: Some("MSDF Atlas"),
            size: Extent3d {
                width: self.font.atlas.width,
                height: self.font.atlas.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        device_handle.queue.write_texture(
            atlas_texture.as_image_copy(),
            &self.font.atlas.texture_data,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.font.atlas.width * 4),
                rows_per_image: None,
            },
            Extent3d {
                width: self.font.atlas.width,
                height: self.font.atlas.height,
                depth_or_array_layers: 1,
            },
        );

        let atlas_view = atlas_texture.create_view(&TextureViewDescriptor::default());

        let text_storage_buffer = device_handle.device.create_buffer(&BufferDescriptor {
            label: Some("Text Instances"),
            size: (std::mem::size_of::<TextInstance>() * 512) as u64,
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

        let device = device_handle.device.clone();
        let queue = device_handle.queue.clone();
        let images = ImageManager::new(&device_handle.device, device_handle.queue.clone());
        let image_view = images.create_view();

        let mut gpu_resources = GpuResources {
            device,
            queue,
            shaders,
            uniform_buffer,
            particles_buffer,
            bg_storage_buffer,
            playhead_buffer,
            playhead_bind_group,
            bg_bind_group: None,
            icon_storage_buffer,
            icon_bind_group: None,
            atlas_view,
            image_view,
            text_storage_buffer,
            text_bind_group: None,
            bg_sampler,
            images,
            requested_textures: HashSet::new(),
        };

        let make_standard_bg = |gpu: &GpuResources, label, layout, storage: BindingResource| {
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
                        resource: BindingResource::TextureView(&gpu.image_view),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: BindingResource::Sampler(&gpu.bg_sampler),
                    },
                ],
            })
        };

        gpu_resources.bg_bind_group = Some(make_standard_bg(
            &gpu_resources,
            "BG BG",
            &gpu_resources.shaders.bg_bind_group_layout,
            gpu_resources.bg_storage_buffer.as_entire_binding(),
        ));
        gpu_resources.icon_bind_group = Some(make_standard_bg(
            &gpu_resources,
            "Icon BG",
            &gpu_resources.shaders.icon_bind_group_layout,
            gpu_resources.icon_storage_buffer.as_entire_binding(),
        ));
        gpu_resources.text_bind_group = Some(gpu_resources.device.create_bind_group(
            &BindGroupDescriptor {
                label: Some("Text BG"),
                layout: &gpu_resources.shaders.text_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: gpu_resources.uniform_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: gpu_resources.text_storage_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::TextureView(&gpu_resources.atlas_view),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: BindingResource::Sampler(&gpu_resources.bg_sampler),
                    },
                ],
            },
        ));

        self.gpu_resources = Some(gpu_resources);
        self.render_surface = Some(render_surface);
    }

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

            if let Some(text_bind_group) = &gpu.text_bind_group {
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
                rpass.set_pipeline(&gpu.shaders.text_pipeline);
                rpass.set_bind_group(0, text_bind_group, &[]);
                rpass.draw(0..4, 0..self.text_instances.len() as u32);
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
                rpass.set_pipeline(&gpu.shaders.playhead_pipeline);
                rpass.set_bind_group(0, &gpu.playhead_bind_group, &[]);
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
