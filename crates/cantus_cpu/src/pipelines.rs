use crate::{
    CantusApp, MAX_RENDER_INSTANCES, PARTICLE_COUNT,
    render::{GpuPass, GpuResources, ImageAtlas},
    text_render::TextRenderer,
};
use cantus_shared::{
    GlobalUniforms, GlyphInstance, MAX_GLYPH_INSTANCES, Particle, PlayheadUniforms, StatusPill,
    TrackPill, WeatherPill,
};
use std::{
    mem::size_of,
    sync::{Arc, Weak},
};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindingResource, BlendState, BufferDescriptor,
    BufferUsages, ColorTargetState, ColorWrites, CompositeAlphaMode, Device, DeviceDescriptor,
    Extent3d, FilterMode, FragmentState, Limits, MemoryHints, MultisampleState,
    PipelineCompilationOptions, PowerPreference, PrimitiveState, PrimitiveTopology,
    RenderPipelineDescriptor, RequestAdapterOptions, SamplerDescriptor, ShaderModule, Surface,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
    TextureViewDimension, VertexState,
};

pub const MAX_TEXTURE_IMAGES: u32 = 32;
pub const IMAGE_SIZE: u32 = 64;

const fn buffer_size<T>(len: usize) -> u64 {
    (size_of::<T>() * len) as u64
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
    let name = label.to_ascii_lowercase();
    let vertex_entry = format!("{name}::vs_{name}");
    let fragment_entry = format!("{name}::fs_{name}");
    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
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
    });
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
        let adapter = pollster::block_on(self.render.instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                ..Default::default()
            },
        ))
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
            size: buffer_size::<GlobalUniforms>(1),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let texture_array = device.create_texture(&TextureDescriptor {
            label: Some("Images"),
            size: Extent3d {
                width: IMAGE_SIZE,
                height: IMAGE_SIZE,
                depth_or_array_layers: MAX_TEXTURE_IMAGES,
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
        macro_rules! pass {
            ($label:literal, $type:ty, $count:expr, $usage:ident $(, $resource:expr)*) => {
                create_pass(
                    $label,
                    buffer_size::<$type>($count),
                    BufferUsages::$usage,
                    &[$($resource),*],
                )
            };
        }
        let playhead = pass!("Playhead", PlayheadUniforms, 1, UNIFORM);
        let particles = pass!("Particles", Particle, PARTICLE_COUNT, STORAGE);
        let track = pass!(
            "Track",
            TrackPill,
            MAX_RENDER_INSTANCES,
            STORAGE,
            BindingResource::TextureView(&image_view),
            BindingResource::Sampler(&sampler)
        );
        let status = pass!("Status", StatusPill, 1, STORAGE);
        let weather = pass!("Weather", WeatherPill, 1, STORAGE);
        let text = pass!(
            "Text",
            GlyphInstance,
            MAX_GLYPH_INSTANCES,
            STORAGE,
            BindingResource::TextureView(&text_atlas_view),
            BindingResource::Sampler(&sampler)
        );

        self.render.gpu = Some(GpuResources {
            device,
            queue,
            surface,
            surface_config,
            uniform_buffer,
            playhead,
            track,
            weather,
            status,
            text,
            particles,
            images: ImageAtlas {
                texture: texture_array,
                slots: [const { Weak::new() }; MAX_TEXTURE_IMAGES as usize],
                used: 0,
            },
            text_renderer,
        });
    }
}
