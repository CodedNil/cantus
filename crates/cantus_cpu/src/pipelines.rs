use crate::render::{BackgroundPill, GlobalUniforms, IconInstance, Particle, PlayheadUniforms};
use crate::text_render::TextRenderer;
use crate::{CantusApp, GpuPass, GpuResources, ImageAtlas, MAX_RENDER_INSTANCES, PARTICLE_COUNT};
use cantus_shared::{GlyphInstance, MAX_GLYPH_INSTANCES};
use std::array;
use std::mem::size_of;
use std::sync::Arc;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendState, BufferBindingType,
    BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, CompositeAlphaMode, Device,
    DeviceDescriptor, Extent3d, FilterMode, FragmentState, Limits, MemoryHints, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PowerPreference, PrimitiveState,
    PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
    SamplerBindingType, SamplerDescriptor, ShaderStages, Surface, TextureDescriptor,
    TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor,
    TextureViewDimension, VertexState,
};

pub const MAX_TEXTURE_IMAGES: u32 = 32;
pub const TEXTURE_LAYER_COUNT: u32 = MAX_TEXTURE_IMAGES * 2;
pub const IMAGE_SIZE: u32 = 64;

fn gpu_pass<const N: usize>(
    device: &Device,
    label: &str,
    layout: &BindGroupLayout,
    pipeline: RenderPipeline,
    uniform_buffer: &wgpu::Buffer,
    size: u64,
    usage: BufferUsages,
    extra_resources: [BindingResource<'_>; N],
) -> GpuPass {
    let buffer_label = format!("{label} Data");
    let buffer = device.create_buffer(&BufferDescriptor {
        label: Some(&buffer_label),
        size,
        usage: usage | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let entries = [
        uniform_buffer.as_entire_binding(),
        buffer.as_entire_binding(),
    ]
    .into_iter()
    .chain(extra_resources)
    .enumerate()
    .map(|(binding, resource)| BindGroupEntry {
        binding: binding as u32,
        resource,
    })
    .collect::<Vec<_>>();
    let bind_group_label = format!("{label} Bind Group");
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some(&bind_group_label),
        layout,
        entries: &entries,
    });
    GpuPass {
        pipeline,
        buffer,
        bind_group,
    }
}

impl CantusApp {
    pub fn configure_render_surface(&mut self, surface: Surface<'static>, width: u32, height: u32) {
        let adapter = pollster::block_on(self.instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .expect("No adapter");

        let info = adapter.get_info();
        tracing::info!("Using adapter: {} ({:?})", info.name, info.device_type);

        let limits = Limits::downlevel_defaults().using_resolution(adapter.limits());
        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
            required_limits: limits,
            memory_hints: MemoryHints::MemoryUsage,
            ..Default::default()
        }))
        .expect("No device");
        device.on_uncaptured_error(Arc::new(|error| {
            tracing::error!(%error, "uncaptured wgpu error");
        }));

        let capabilities = surface.get_capabilities(&adapter);
        let alpha_mode = [
            CompositeAlphaMode::PreMultiplied,
            CompositeAlphaMode::PostMultiplied,
        ]
        .into_iter()
        .find(|m| capabilities.alpha_modes.contains(m))
        .unwrap_or(CompositeAlphaMode::Auto);

        let format = [TextureFormat::Rgba8Unorm, TextureFormat::Bgra8Unorm]
            .into_iter()
            .find(|format| capabilities.formats.contains(format))
            .unwrap_or(capabilities.formats[0]);
        let mut surface_config = surface
            .get_default_config(&adapter, width, height)
            .expect("Surface is unsupported by the selected adapter");
        surface_config.format = format;
        surface_config.alpha_mode = alpha_mode;
        surface.configure(&device, &surface_config);

        let text_renderer = TextRenderer::new(&device, self.config.height);

        let rust_gpu_shader = device.create_shader_module(wgpu::include_spirv!(concat!(
            env!("OUT_DIR"),
            "/cantus.spv"
        )));

        let bgl = |label, entries: &[(ShaderStages, BindingType)]| {
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some(label),
                entries: &entries
                    .iter()
                    .enumerate()
                    .map(|(binding, &(visibility, ty))| BindGroupLayoutEntry {
                        binding: binding as u32,
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

        let playhead_layout = bgl("Playhead", &[(vf, ub), (ShaderStages::FRAGMENT, ub)]);
        let particle_layout = bgl(
            "Particles",
            &[(ShaderStages::VERTEX, ub), (ShaderStages::VERTEX, sb)],
        );
        let std_layout = bgl(
            "Standard",
            &[
                (vf, ub),
                (vf, sb),
                (ShaderStages::FRAGMENT, tx(TextureViewDimension::D2Array)),
                (ShaderStages::FRAGMENT, sp),
            ],
        );
        // Text: uniform(buffer0) + storage(buffer1) + atlas(tex2) + sampler(3)
        let text_layout = bgl(
            "Text",
            &[
                (vf, ub),
                (vf, sb),
                (ShaderStages::FRAGMENT, tx(TextureViewDimension::D2)),
                (ShaderStages::FRAGMENT, sp),
            ],
        );

        let create_pipe = |label, layout: &BindGroupLayout, vertex_entry, fragment_entry| {
            device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some(label),
                    bind_group_layouts: &[Some(layout)],
                    ..Default::default()
                })),
                vertex: VertexState {
                    module: &rust_gpu_shader,
                    entry_point: Some(vertex_entry),
                    buffers: &[],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                fragment: Some(FragmentState {
                    module: &rust_gpu_shader,
                    entry_point: Some(fragment_entry),
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
                multiview_mask: None,
                cache: None,
            })
        };

        let playhead_pipeline = create_pipe(
            "Playhead",
            &playhead_layout,
            "playhead::vs_playhead",
            "playhead::fs_playhead",
        );
        let particle_pipeline = create_pipe(
            "Particles",
            &particle_layout,
            "particles::vs_particles",
            "particles::fs_particles",
        );
        let background_pipeline = create_pipe(
            "Background",
            &std_layout,
            "background::vs_background",
            "background::fs_background",
        );
        let icon_pipeline = create_pipe("Icons", &std_layout, "icons::vs_icons", "icons::fs_icons");
        let text_pipeline = create_pipe("Text", &text_layout, "text::vs_text", "text::fs_text");

        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Uniforms"),
            size: size_of::<GlobalUniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let texture_array = device.create_texture(&TextureDescriptor {
            label: Some("Images"),
            size: Extent3d {
                width: IMAGE_SIZE,
                height: IMAGE_SIZE,
                depth_or_array_layers: TEXTURE_LAYER_COUNT,
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

        let playhead = gpu_pass(
            &device,
            "Playhead",
            &playhead_layout,
            playhead_pipeline,
            &uniform_buffer,
            size_of::<PlayheadUniforms>() as u64,
            BufferUsages::UNIFORM,
            [],
        );
        let particles = gpu_pass(
            &device,
            "Particles",
            &particle_layout,
            particle_pipeline,
            &uniform_buffer,
            (size_of::<Particle>() * PARTICLE_COUNT) as u64,
            BufferUsages::STORAGE,
            [],
        );
        let background = gpu_pass(
            &device,
            "Background",
            &std_layout,
            background_pipeline,
            &uniform_buffer,
            (size_of::<BackgroundPill>() * MAX_RENDER_INSTANCES) as u64,
            BufferUsages::STORAGE,
            [
                BindingResource::TextureView(&image_view),
                BindingResource::Sampler(&sampler),
            ],
        );
        let icons = gpu_pass(
            &device,
            "Icons",
            &std_layout,
            icon_pipeline,
            &uniform_buffer,
            (size_of::<IconInstance>() * MAX_RENDER_INSTANCES) as u64,
            BufferUsages::STORAGE,
            [
                BindingResource::TextureView(&image_view),
                BindingResource::Sampler(&sampler),
            ],
        );
        let text = gpu_pass(
            &device,
            "Text",
            &text_layout,
            text_pipeline,
            &uniform_buffer,
            (size_of::<GlyphInstance>() * MAX_GLYPH_INSTANCES) as u64,
            BufferUsages::STORAGE,
            [
                BindingResource::TextureView(text_renderer.atlas_view()),
                BindingResource::Sampler(&sampler),
            ],
        );

        self.gpu_resources = Some(GpuResources {
            device,
            queue,
            surface,
            surface_config,
            uniform_buffer,
            playhead,
            background,
            icons,
            text,
            particles,
            images: ImageAtlas {
                texture: texture_array,
                slots: array::from_fn(|_| None),
                used: 0,
            },
            text_renderer,
        });
    }
}
