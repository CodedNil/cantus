use bytemuck::{Pod, Zeroable};
use std::{borrow::Cow, collections::HashMap};
use vello::{Renderer, peniko::ImageData, util::DeviceHandle, wgpu};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferDescriptor, BufferUsages, Color,
    ColorTargetState, ColorWrites, CommandEncoderDescriptor, Extent3d, FragmentState, LoadOp,
    MultisampleState, Operations, PipelineLayoutDescriptor, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler, SamplerBindingType,
    SamplerDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp,
    TexelCopyBufferLayout, Texture, TextureDescriptor, TextureDimension, TextureFormat,
    TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension,
    VertexState,
};

const WARP_SHADER_SRC: &str = include_str!("warp_background.wgsl");

const BACKGROUND_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct WarpUniforms {
    params: [f32; 4],
}

struct BackgroundSlot {
    texture: Texture,
    texture_size: (u32, u32),
    bind_group: BindGroup,
    output_view: TextureView,
    output_size: (u32, u32),
    output_image: ImageData,
    last_frame: u64,
}

pub struct WarpBackground {
    pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    sampler: Sampler,
    uniform_buffer: wgpu::Buffer,
    slots: HashMap<String, BackgroundSlot>,
}

impl WarpBackground {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("cantus_warp_shader"),
            source: ShaderSource::Wgsl(Cow::Borrowed(WARP_SHADER_SRC)),
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("cantus_warp_bind_group_layout"),
            entries: &[
                // Album Texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                // Uniform Buffer
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("cantus_warp_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("cantus_warp_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: BACKGROUND_FORMAT,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("cantus_warp_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("cantus_warp_uniform_buffer"),
            size: std::mem::size_of::<WarpUniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            bind_group_layout,
            sampler,
            uniform_buffer,
            slots: HashMap::new(),
        }
    }

    pub fn render(
        &mut self,
        key: &str,
        device_handle: &DeviceHandle,
        renderer: &mut Renderer,
        width: u32,
        height: u32,
        image: &ImageData,
        elapsed_seconds: f32,
        frame_index: u64,
    ) -> Option<ImageData> {
        if width == 0 || height == 0 || image.width == 0 || image.height == 0 {
            return None;
        }

        let slot = self
            .slots
            .entry(key.to_string())
            .and_modify(|slot| {
                if slot.texture_size != (image.width, image.height)
                    || slot.output_size != (width, height)
                {
                    // Dimensions changed, recreate the slot
                    renderer.unregister_texture(slot.output_image.clone()); // Unregister old texture
                    *slot = BackgroundSlot::new(
                        &device_handle.device,
                        renderer,
                        &self.bind_group_layout,
                        &self.sampler,
                        &self.uniform_buffer,
                        width,
                        height,
                        image,
                    );
                }
            })
            .or_insert_with(|| {
                BackgroundSlot::new(
                    &device_handle.device,
                    renderer,
                    &self.bind_group_layout,
                    &self.sampler,
                    &self.uniform_buffer,
                    width,
                    height,
                    image,
                )
            });

        device_handle.queue.write_texture(
            slot.texture.as_image_copy(),
            image.data.data(),
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * image.width),
                rows_per_image: Some(image.height),
            },
            Extent3d {
                width: image.width,
                height: image.height,
                depth_or_array_layers: 1,
            },
        );

        slot.last_frame = frame_index;

        let uniforms = WarpUniforms {
            params: [elapsed_seconds, 0.0, 0.0, 0.0],
        };
        device_handle
            .queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

        let mut encoder = device_handle
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("cantus_warp_encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("cantus_warp_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &slot.output_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::TRANSPARENT),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &slot.bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        device_handle.queue.submit(Some(encoder.finish()));
        Some(slot.output_image.clone())
    }

    pub fn purge_stale(&mut self, renderer: &mut Renderer, frame_index: u64) {
        const STALE_FRAME_BUDGET: u64 = 600;
        self.slots.retain(|_, slot| {
            let keep = frame_index.saturating_sub(slot.last_frame) <= STALE_FRAME_BUDGET;
            if !keep {
                renderer.unregister_texture(slot.output_image.clone());
            }
            keep
        });
    }
}

impl BackgroundSlot {
    fn new(
        device: &wgpu::Device,
        renderer: &mut Renderer,
        bind_group_layout: &BindGroupLayout,
        sampler: &Sampler,
        uniform_buffer: &wgpu::Buffer,
        width: u32,
        height: u32,
        image: &ImageData,
    ) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("cantus_warp_texture"),
            size: Extent3d {
                width: image.width,
                height: image.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("cantus_warp_bind_group"),
            layout: bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        });

        let output_texture = device.create_texture(&TextureDescriptor {
            label: Some("cantus_warp_output_texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: BACKGROUND_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let output_view = output_texture.create_view(&TextureViewDescriptor::default());
        let output_image = renderer.register_texture(output_texture);

        Self {
            texture,
            bind_group,
            texture_size: (image.width, image.height),
            output_view,
            output_size: (width, height),
            output_image,
            last_frame: 0,
        }
    }
}
