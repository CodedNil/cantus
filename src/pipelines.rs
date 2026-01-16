use crate::render::{BackgroundPill, GlobalUniforms, IconInstance, Particle, PlayheadUniforms};
use crate::text_render::TextRenderer;
use crate::{CantusApp, GpuResources};
use std::collections::HashMap;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendState, BufferBindingType,
    BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, CompositeAlphaMode,
    DeviceDescriptor, ExperimentalFeatures, Extent3d, Features, FilterMode, FragmentState, Limits,
    MemoryHints, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PowerPreference, PresentMode, PrimitiveState, PrimitiveTopology, RenderPipelineDescriptor,
    RequestAdapterOptions, SamplerBindingType, SamplerDescriptor, ShaderModule,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, Surface, SurfaceConfiguration,
    TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages,
    TextureViewDescriptor, TextureViewDimension, Trace, VertexState,
};

pub const MAX_TEXTURE_LAYERS: u32 = 48;
pub const IMAGE_SIZE: u32 = 64;

impl CantusApp {
    pub fn configure_render_surface(&mut self, surface: Surface<'static>, width: u32, height: u32) {
        let adapter = pollster::block_on(self.instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("No adapter");

        let info = adapter.get_info();
        tracing::info!("Using adapter: {} ({:?})", info.name, info.device_type);

        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::downlevel_defaults(),
            experimental_features: ExperimentalFeatures::disabled(),
            memory_hints: MemoryHints::MemoryUsage,
            trace: Trace::Off,
        }))
        .expect("No device");

        let capabilities = surface.get_capabilities(&adapter);
        let alpha_mode = [
            CompositeAlphaMode::PreMultiplied,
            CompositeAlphaMode::PostMultiplied,
        ]
        .into_iter()
        .find(|m| capabilities.alpha_modes.contains(m))
        .unwrap_or(CompositeAlphaMode::Auto);

        let format = TextureFormat::Rgba8Unorm;
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: PresentMode::AutoVsync,
            desired_maximum_frame_latency: 1,
            alpha_mode,
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        self.text_renderer = Some(TextRenderer::new(&device, format));

        let create_shader = |label, source: &str| {
            device.create_shader_module(ShaderModuleDescriptor {
                label: Some(label),
                source: ShaderSource::Wgsl(source.into()),
            })
        };
        let playhead_shader = create_shader("Playhead", include_str!("../assets/playhead.wgsl"));
        let particle_shader = create_shader("Particles", include_str!("../assets/particles.wgsl"));
        let background_shader =
            create_shader("Background", include_str!("../assets/background.wgsl"));
        let icon_shader = create_shader("Icons", include_str!("../assets/icons.wgsl"));

        let bgl = |label, entries: &[(u32, ShaderStages, BindingType)]| {
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some(label),
                entries: &entries
                    .iter()
                    .map(|&(binding, visibility, ty)| BindGroupLayoutEntry {
                        binding,
                        visibility,
                        ty,
                        count: None,
                    })
                    .collect::<Vec<_>>(),
            })
        };

        let ub = BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        };
        let sb = BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        };
        let tx = |view_dimension| BindingType::Texture {
            multisampled: false,
            view_dimension,
            sample_type: TextureSampleType::Float { filterable: true },
        };
        let sp = BindingType::Sampler(SamplerBindingType::Filtering);
        let vf = ShaderStages::VERTEX | ShaderStages::FRAGMENT;

        let playhead_layout = bgl("Playhead", &[(0, vf, ub), (1, ShaderStages::FRAGMENT, ub)]);
        let particle_layout = bgl(
            "Particles",
            &[(0, ShaderStages::VERTEX, ub), (1, ShaderStages::VERTEX, sb)],
        );
        let std_layout = bgl(
            "Standard",
            &[
                (0, vf, ub),
                (1, vf, sb),
                (2, ShaderStages::FRAGMENT, tx(TextureViewDimension::D2Array)),
                (3, ShaderStages::FRAGMENT, sp),
            ],
        );

        let create_pipe = |label, shader: &ShaderModule, layout: &BindGroupLayout| {
            device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some(label),
                    bind_group_layouts: &[layout],
                    ..Default::default()
                })),
                vertex: VertexState {
                    module: shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                fragment: Some(FragmentState {
                    module: shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(ColorTargetState {
                        format,
                        blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    })],
                    compilation_options: PipelineCompilationOptions::default(),
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: MultisampleState::default(),
                multiview: None,
                cache: None,
            })
        };

        let playhead_pipeline = create_pipe("Playhead", &playhead_shader, &playhead_layout);
        let particle_pipeline = create_pipe("Particles", &particle_shader, &particle_layout);
        let background_pipeline = create_pipe("Background", &background_shader, &std_layout);
        let icon_pipeline = create_pipe("Icons", &icon_shader, &std_layout);

        let mk_buf = |l, s, u| {
            device.create_buffer(&BufferDescriptor {
                label: Some(l),
                size: s,
                usage: u | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };

        let uniform_buffer = mk_buf(
            "Uniforms",
            std::mem::size_of::<GlobalUniforms>() as u64,
            BufferUsages::UNIFORM,
        );
        let particles_buffer = mk_buf(
            "Particles",
            (std::mem::size_of::<Particle>() * 64) as u64,
            BufferUsages::STORAGE,
        );
        let playhead_buffer = mk_buf(
            "Playhead",
            std::mem::size_of::<PlayheadUniforms>() as u64,
            BufferUsages::UNIFORM,
        );
        let background_storage_buffer = mk_buf(
            "BG Pills",
            (std::mem::size_of::<BackgroundPill>() * 256) as u64,
            BufferUsages::STORAGE,
        );
        let icon_storage_buffer = mk_buf(
            "Icons",
            (std::mem::size_of::<IconInstance>() * 256) as u64,
            BufferUsages::STORAGE,
        );

        let texture_array = device.create_texture(&TextureDescriptor {
            label: Some("Images"),
            size: Extent3d {
                width: IMAGE_SIZE,
                height: IMAGE_SIZE,
                depth_or_array_layers: MAX_TEXTURE_LAYERS,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let image_view = texture_array.create_view(&TextureViewDescriptor {
            dimension: Some(TextureViewDimension::D2Array),
            ..Default::default()
        });

        let sampler = device.create_sampler(&SamplerDescriptor {
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..Default::default()
        });

        let mk_bg = |l, layout, entries: &[BindGroupEntry]| {
            device.create_bind_group(&BindGroupDescriptor {
                label: Some(l),
                layout,
                entries,
            })
        };

        let playhead_bind_group = mk_bg(
            "Playhead",
            &playhead_layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: playhead_buffer.as_entire_binding(),
                },
            ],
        );
        let particle_bind_group = mk_bg(
            "Particles",
            &particle_layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: particles_buffer.as_entire_binding(),
                },
            ],
        );
        let background_bind_group = mk_bg(
            "Background",
            &std_layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: background_storage_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(&image_view),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        );
        let icon_bind_group = mk_bg(
            "Icon",
            &std_layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: icon_storage_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(&image_view),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        );

        self.gpu_resources = Some(GpuResources {
            device,
            queue,
            surface,
            surface_config,
            playhead_pipeline,
            background_pipeline,
            icon_pipeline,
            particle_pipeline,
            uniform_buffer,
            particles_buffer,
            playhead_buffer,
            background_storage_buffer,
            icon_storage_buffer,
            playhead_bind_group,
            background_bind_group,
            icon_bind_group,
            particle_bind_group,
            texture_array,
            url_to_image_index: HashMap::new(),
        });
    }
}
