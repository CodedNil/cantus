use crate::{
    CantusApp, GpuPass, GpuResources, ImageAtlas, MAX_RENDER_INSTANCES, PARTICLE_COUNT,
    text_render::TextRenderer,
};
use cantus_shared::{
    BackgroundPill, GlobalUniforms, GlyphInstance, MAX_GLYPH_INSTANCES, Particle, PlayheadUniforms,
};
use std::{mem::size_of, sync::Arc};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindingResource, BlendState, BufferDescriptor,
    BufferUsages, ColorTargetState, ColorWrites, CompositeAlphaMode, Device, DeviceDescriptor,
    Extent3d, FilterMode, FragmentState, Limits, MemoryHints, MultisampleState,
    PipelineCompilationOptions, PowerPreference, PrimitiveState, PrimitiveTopology, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptions, SamplerDescriptor, ShaderModule, Surface,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
    TextureViewDimension, VertexState,
};

pub const MAX_TEXTURE_IMAGES: u32 = 32;
pub const TEXTURE_LAYER_COUNT: u32 = MAX_TEXTURE_IMAGES * 2;
pub const IMAGE_SIZE: u32 = 64;

fn render_pipeline(
    device: &Device,
    shader: &ShaderModule,
    format: TextureFormat,
    label: &str,
) -> RenderPipeline {
    let name = label.to_ascii_lowercase();
    let vertex_entry = format!("{name}::vs_{name}");
    let fragment_entry = format!("{name}::fs_{name}");
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some(label),
        layout: None,
        vertex: VertexState {
            module: shader,
            entry_point: Some(&vertex_entry),
            buffers: &[],
            compilation_options: PipelineCompilationOptions::default(),
        },
        fragment: Some(FragmentState {
            module: shader,
            entry_point: Some(&fragment_entry),
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
}

fn gpu_pass(
    device: &Device,
    shader: &ShaderModule,
    format: TextureFormat,
    label: &str,
    uniform_buffer: &wgpu::Buffer,
    size: u64,
    usage: BufferUsages,
    extra_resources: &[BindingResource<'_>],
) -> GpuPass {
    let pipeline = render_pipeline(device, shader, format, label);
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
    .chain(extra_resources.iter().cloned())
    .enumerate()
    .map(|(binding, resource)| BindGroupEntry {
        binding: binding as u32,
        resource,
    })
    .collect::<Vec<_>>();
    let bind_group_label = format!("{label} Bind Group");
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some(&bind_group_label),
        layout: &pipeline.get_bind_group_layout(0),
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
        device.on_uncaptured_error(Arc::new(
            |error| tracing::error!(%error, "uncaptured wgpu error"),
        ));

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
        let text_atlas_view = text_renderer.atlas_view();

        let rust_gpu_shader =
            device.create_shader_module(wgpu::include_spirv!("../../../assets/cantus.spv"));

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

        let create_pass = |label, size, usage, resources: &[BindingResource<'_>]| {
            gpu_pass(
                &device,
                &rust_gpu_shader,
                format,
                label,
                &uniform_buffer,
                size,
                usage,
                resources,
            )
        };
        let playhead = create_pass(
            "Playhead",
            size_of::<PlayheadUniforms>() as u64,
            BufferUsages::UNIFORM,
            &[],
        );
        let particles = create_pass(
            "Particles",
            (size_of::<Particle>() * PARTICLE_COUNT) as u64,
            BufferUsages::STORAGE,
            &[],
        );
        let background = create_pass(
            "Background",
            (size_of::<BackgroundPill>() * MAX_RENDER_INSTANCES) as u64,
            BufferUsages::STORAGE,
            &[
                BindingResource::TextureView(&image_view),
                BindingResource::Sampler(&sampler),
            ],
        );
        let text = create_pass(
            "Text",
            (size_of::<GlyphInstance>() * MAX_GLYPH_INSTANCES) as u64,
            BufferUsages::STORAGE,
            &[
                BindingResource::TextureView(&text_atlas_view),
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
            text,
            particles,
            images: ImageAtlas {
                texture: texture_array,
                slots: [const { None }; MAX_TEXTURE_IMAGES as usize],
                used: 0,
            },
            text_renderer,
        });
    }
}
