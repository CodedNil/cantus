use crate::spotify::{IMAGES_CACHE, PLAYBACK_STATE};
use crate::{
    interaction::InteractionState,
    render::{FontEngine, ParticlesState, RenderState},
};
use render_types::{BackgroundPill, Particle, ScreenUniforms, Shaders};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tracing_subscriber::EnvFilter;
use vello::{
    AaConfig, AaSupport, Renderer, RendererOptions, Scene,
    peniko::color::AlphaColor,
    util::{RenderContext, RenderSurface},
};
use wgpu::{
    BlendComponent, BlendFactor, BlendOperation, BlendState, CommandEncoderDescriptor,
    CompositeAlphaMode, Extent3d, PresentMode, Surface, SurfaceConfiguration, TextureDescriptor,
    TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
    util::TextureBlitterBuilder,
};

#[cfg(not(any(feature = "wayland", feature = "winit")))]
compile_error!("Enable at least one of the `wayland` or `winit` features.");

#[cfg(all(feature = "wayland", feature = "winit"))]
compile_error!("`wayland` and `winit` features cannot be enabled at the same time.");

mod background;
mod config;
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
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub shaders: Shaders,
    pub uniform_buffer: wgpu::Buffer,
    pub storage_buffer: wgpu::Buffer,
    pub particle_bind_group: wgpu::BindGroup,
    pub bg_storage_buffer: wgpu::Buffer,
    pub bg_bind_group: Option<wgpu::BindGroup>,
    pub bg_texture_array: Option<wgpu::Texture>,
    pub bg_sampler: wgpu::Sampler,
    pub last_texture_set: HashSet<String>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(
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
    gpu_resources: Option<GpuResources>,
    gpu_uniforms: Option<ScreenUniforms>,
    pub image_map: HashMap<String, i32>,
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
            gpu_resources: None,
            gpu_uniforms: None,
            image_map: HashMap::new(),
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
            dev_id,
            format,
            target_texture,
            target_view,
            blitter: TextureBlitterBuilder::new(&device_handle.device, format)
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
        let uniform_buffer = device_handle.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Particle Uniform Buffer"),
            size: std::mem::size_of::<ScreenUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let storage_buffer = device_handle.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Particle Storage Buffer"),
            size: (std::mem::size_of::<Particle>() * 64) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let particle_bind_group =
            device_handle
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Particle Bind Group"),
                    layout: &shaders.bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: uniform_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: storage_buffer.as_entire_binding(),
                        },
                    ],
                });

        let bg_storage_buffer = device_handle.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Background Storage Buffer"),
            size: (std::mem::size_of::<BackgroundPill>() * 16) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bg_sampler = device_handle
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

        self.gpu_resources = Some(GpuResources {
            device: Arc::new(device_handle.device.clone()),
            queue: Arc::new(device_handle.queue.clone()),
            shaders,
            uniform_buffer,
            storage_buffer,
            particle_bind_group,
            bg_storage_buffer,
            bg_bind_group: None,
            bg_texture_array: None,
            bg_sampler,
            last_texture_set: HashSet::new(),
        });

        self.render_surface = Some(render_surface);
    }

    /// Try to render out the app
    fn render(&mut self) {
        if self.render_surface.is_none() {
            return;
        }
        self.scene.reset();
        self.image_map.clear();

        let mut image_refs = Vec::new();
        let mut current_texture_urls = Vec::new();
        let mut url_to_index = HashMap::new();

        // 1. Identify unique images and check for changes
        for track in &PLAYBACK_STATE.read().queue {
            if !current_texture_urls.contains(&track.album.image) {
                current_texture_urls.push(track.album.image.clone());
            }
        }

        for (i, url) in current_texture_urls.iter().enumerate() {
            if let Some(img_ref) = IMAGES_CACHE.get(url)
                && img_ref.is_some()
            {
                url_to_index.insert(url.clone(), i as i32);
                image_refs.push(img_ref);
            }
        }

        let needs_texture_update = self.gpu_resources.as_ref().is_none_or(|gpu| {
            let current_set: HashSet<String> = current_texture_urls.iter().cloned().collect();
            current_set != gpu.last_texture_set
        });
        self.image_map = url_to_index;

        // Now build the scene (fills self.background_pills using self.image_map index)
        self.create_scene();

        // Collect the raw image pointers safely
        let mut unique_textures = Vec::new();
        for img_ref in &image_refs {
            if let Some(img) = img_ref.as_ref() {
                unique_textures.push(&img.image);
            }
        }

        // --- PREPARE GPU RESOURCES (Texture Array & Bind Group) ---
        if let Some(gpu) = self.gpu_resources.as_mut() {
            // Create or update the texture array if we have images
            if needs_texture_update && !unique_textures.is_empty() {
                let width = unique_textures[0].width;
                let height = unique_textures[0].height;
                let layers = unique_textures.len() as u32;

                let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("Background Texture Array"),
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: layers,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });

                for (i, img) in unique_textures.iter().enumerate() {
                    gpu.queue.write_texture(
                        wgpu::TexelCopyTextureInfo {
                            texture: &texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d {
                                x: 0,
                                y: 0,
                                z: i as u32,
                            },
                            aspect: wgpu::TextureAspect::All,
                        },
                        img.data.as_ref(), // Use .as_ref() to get &[u8] from Blob<u8>
                        wgpu::TexelCopyBufferLayout {
                            offset: 0,
                            bytes_per_row: Some(4 * width),
                            rows_per_image: Some(height),
                        },
                        wgpu::Extent3d {
                            width,
                            height,
                            depth_or_array_layers: 1,
                        },
                    );
                }

                let view = texture.create_view(&wgpu::TextureViewDescriptor {
                    dimension: Some(wgpu::TextureViewDimension::D2Array),
                    ..Default::default()
                });

                gpu.bg_bind_group =
                    Some(gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("Background Bind Group"),
                        layout: &gpu.shaders.bg_bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: gpu.uniform_buffer.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: gpu.bg_storage_buffer.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::TextureView(&view),
                            },
                            wgpu::BindGroupEntry {
                                binding: 3,
                                resource: wgpu::BindingResource::Sampler(&gpu.bg_sampler),
                            },
                        ],
                    }));
                gpu.bg_texture_array = Some(texture);
                gpu.last_texture_set = self.image_map.keys().cloned().collect();
            }

            if let Some(gpu_uniforms) = self.gpu_uniforms.as_ref() {
                gpu.queue
                    .write_buffer(&gpu.uniform_buffer, 0, bytemuck::bytes_of(gpu_uniforms));
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
        }

        // Explicitly drop image_refs after GPU upload to satisfy clippy and release DashMap locks
        drop(image_refs);

        // Now we can safely borrow other parts
        let render_surface_ref = self.render_surface.as_ref().unwrap();
        let dev_id = render_surface_ref.dev_id;
        let handle = &self.render_context.devices[dev_id];

        self.render_device
            .as_mut()
            .unwrap()
            .render_to_texture(
                &handle.device,
                &handle.queue,
                &self.scene,
                &render_surface_ref.target_view,
                &vello::RenderParams {
                    base_color: AlphaColor::from_rgba8(0, 0, 0, 0),
                    width: render_surface_ref.config.width,
                    height: render_surface_ref.config.height,
                    antialiasing_method: AaConfig::Area,
                },
            )
            .expect("failed to render to surface");

        let render_surface = self.render_surface.as_mut().unwrap();
        let Ok(surface_texture) = render_surface.surface.get_current_texture() else {
            let render_surface = self.render_surface.as_mut().unwrap();
            render_surface.surface.configure(
                &self.render_context.devices[render_surface.dev_id].device,
                &render_surface.config,
            );
            return;
        };

        let surface_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        if let Some(gpu) = self.gpu_resources.as_ref() {
            let mut encoder = handle
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("Main Render Encoder"),
                });

            // 1. CLEAR AND RENDER BACKGROUND PILLS
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Background Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &surface_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                if let Some(bind_group) = &gpu.bg_bind_group {
                    rpass.set_pipeline(&gpu.shaders.bg_pipeline);
                    rpass.set_bind_group(0, bind_group, &[]);
                    rpass.draw(0..4, 0..self.background_pills.len() as u32);
                }
            }

            // 2. BLIT VELLO SCENE ON TOP
            render_surface.blitter.copy(
                &handle.device,
                &mut encoder,
                &render_surface.target_view,
                &surface_view,
            );

            // 3. RENDER PARTICLES ON TOP
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Particle Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &surface_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                rpass.set_pipeline(&gpu.shaders.pipeline);
                rpass.set_bind_group(0, &gpu.particle_bind_group, &[]);
                rpass.draw(0..4, 0..1);
            }

            handle.queue.submit([encoder.finish()]);
        }

        surface_texture.present();
    }
}

fn lerpf64(t: f64, v0: f64, v1: f64) -> f64 {
    v0 + t * (v1 - v0)
}

fn lerpf32(t: f32, v0: f32, v1: f32) -> f32 {
    v0 + t * (v1 - v0)
}
