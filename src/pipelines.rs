use crate::image_manager::ImageManager;
use crate::render::{BackgroundPill, IconInstance, Particle, PlayheadUniforms, ScreenUniforms};
use crate::text_render::TextInstance;
use crate::{CantusApp, DeviceHandle, GpuResources, RenderSurface};
use std::{collections::HashSet, sync::Arc};
use wgpu::{
    AddressMode, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendState, BufferBindingType,
    BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, CompositeAlphaMode,
    DeviceDescriptor, ExperimentalFeatures, Extent3d, Features, FilterMode, FragmentState, Limits,
    MemoryHints, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PowerPreference, PresentMode, PrimitiveState, PrimitiveTopology, RenderPipelineDescriptor,
    RequestAdapterOptions, SamplerBindingType, SamplerDescriptor, ShaderModule,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, Surface, SurfaceConfiguration,
    TexelCopyBufferLayout, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType,
    TextureUsages, TextureViewDescriptor, TextureViewDimension, Trace, VertexState,
};

impl CantusApp {
    pub fn configure_render_surface(&mut self, surface: Surface<'static>, width: u32, height: u32) {
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

        let device = &device_handle.device;

        // Shader Modules
        let playhead_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Playhead Shader"),
            source: ShaderSource::Wgsl(include_str!("../assets/playhead.wgsl").into()),
        });
        let background_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Background Shader"),
            source: ShaderSource::Wgsl(include_str!("../assets/background.wgsl").into()),
        });
        let icon_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Icons Shader"),
            source: ShaderSource::Wgsl(include_str!("../assets/icons.wgsl").into()),
        });
        let text_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: ShaderSource::Wgsl(include_str!("../assets/text.wgsl").into()),
        });

        let ub = |_| BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        };
        let sb = |_| BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        };
        let tx = |d| BindingType::Texture {
            multisampled: false,
            view_dimension: d,
            sample_type: TextureSampleType::Float { filterable: true },
        };
        let sp = BindingType::Sampler(SamplerBindingType::Filtering);

        let bgl = |l, e: &[(u32, ShaderStages, BindingType)]| {
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some(l),
                entries: &e
                    .iter()
                    .map(|&(b, v, ty)| BindGroupLayoutEntry {
                        binding: b,
                        visibility: v,
                        ty,
                        count: None,
                    })
                    .collect::<Vec<_>>(),
            })
        };

        let vf = ShaderStages::VERTEX | ShaderStages::FRAGMENT;
        let playhead_bind_group_layout = bgl(
            "Playhead",
            &[
                (0, ShaderStages::FRAGMENT, ub(0)),
                (1, ShaderStages::FRAGMENT, ub(0)),
                (2, ShaderStages::FRAGMENT, sb(0)),
            ],
        );
        let standard_bind_group_layout = bgl(
            "Standard",
            &[
                (0, vf, ub(0)),
                (1, vf, sb(0)),
                (2, ShaderStages::FRAGMENT, tx(TextureViewDimension::D2Array)),
                (3, ShaderStages::FRAGMENT, sp),
            ],
        );
        let text_bind_group_layout = bgl(
            "Text",
            &[
                (0, vf, ub(0)),
                (1, vf, sb(0)),
                (2, ShaderStages::FRAGMENT, tx(TextureViewDimension::D2)),
                (3, ShaderStages::FRAGMENT, sp),
            ],
        );

        // Pipeline Helper
        let create_pipe = |label: &str, shader: &ShaderModule, layout: &BindGroupLayout| {
            let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some(&format!("{label} Pipeline Layout")),
                bind_group_layouts: &[layout],
                push_constant_ranges: &[],
            });

            device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(&format!("{label} Pipeline")),
                layout: Some(&pipeline_layout),
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
                        blend: Some(BlendState::ALPHA_BLENDING),
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

        let playhead_pipeline =
            create_pipe("Playhead", &playhead_shader, &playhead_bind_group_layout);
        let background_pipeline = create_pipe(
            "Background",
            &background_shader,
            &standard_bind_group_layout,
        );
        let icon_pipeline = create_pipe("Icons", &icon_shader, &standard_bind_group_layout);
        let text_pipeline = create_pipe("Text", &text_shader, &text_bind_group_layout);

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

        let background_storage_buffer = device_handle.device.create_buffer(&BufferDescriptor {
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

        let sampler = device_handle.device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let queue = device_handle.queue.clone();
        let images = ImageManager::new(&device_handle.device, device_handle.queue.clone());
        let image_view = images.create_view();

        let playhead_bind_group = device_handle
            .device
            .create_bind_group(&BindGroupDescriptor {
                label: Some("Playhead BG"),
                layout: &playhead_pipeline.get_bind_group_layout(0),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: playhead_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: particles_buffer.as_entire_binding(),
                    },
                ],
            });
        let background_bind_group = device_handle
            .device
            .create_bind_group(&BindGroupDescriptor {
                label: Some("Background Bind Group"),
                layout: &standard_bind_group_layout,
                entries: &[
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
            });
        let icon_bind_group = device_handle
            .device
            .create_bind_group(&BindGroupDescriptor {
                label: Some("Icon Bind Group"),
                layout: &standard_bind_group_layout,
                entries: &[
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
            });
        let text_bind_group = device_handle
            .device
            .create_bind_group(&BindGroupDescriptor {
                label: Some("Text Bind Group"),
                layout: &text_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: text_storage_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::TextureView(&atlas_view),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: BindingResource::Sampler(&sampler),
                    },
                ],
            });

        let gpu_resources = GpuResources {
            queue,

            playhead_pipeline,
            background_pipeline,
            icon_pipeline,
            text_pipeline,

            uniform_buffer,
            particles_buffer,
            background_storage_buffer,
            playhead_buffer,
            playhead_bind_group,
            background_bind_group,
            icon_storage_buffer,
            icon_bind_group,
            text_storage_buffer,
            text_bind_group,

            images,
            requested_textures: HashSet::new(),
        };

        self.gpu_resources = Some(gpu_resources);
        self.render_surface = Some(render_surface);
    }
}
