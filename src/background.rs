use bytemuck::{Pod, Zeroable};
use std::borrow::Cow;
use vello::{Renderer, peniko::ImageData, wgpu};
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

const WARP_STRENGTH: f32 = 1.4;
const SWIRL_STRENGTH: f32 = 0.2;
const WARP_TIME_SCALE: f32 = 0.8;
const BACKGROUND_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct WarpUniforms {
    resolution: [f32; 4],
    params: [f32; 4],
}

struct AlbumSlot {
    texture: Texture,
    bind_group: BindGroup,
    size: (u32, u32),
}

struct OutputSlot {
    view: TextureView,
    size: (u32, u32),
    image: ImageData,
}

pub struct WarpBackground {
    pipeline: RenderPipeline,
    album_layout: BindGroupLayout,
    sampler: Sampler,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: BindGroup,
    album: Option<AlbumSlot>,
    output: Option<OutputSlot>,
}

impl WarpBackground {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("cantus_warp_shader"),
            source: ShaderSource::Wgsl(Cow::Borrowed(WARP_SHADER_SRC)),
        });

        let album_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("cantus_warp_album_layout"),
            entries: &[
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
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let uniform_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("cantus_warp_uniform_layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("cantus_warp_pipeline_layout"),
            bind_group_layouts: &[&album_layout, &uniform_layout],
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

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("cantus_warp_uniform_bind_group"),
            layout: &uniform_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            pipeline,
            album_layout,
            sampler,
            uniform_buffer,
            uniform_bind_group,
            album: None,
            output: None,
        }
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        renderer: &mut Renderer,
        width: u32,
        height: u32,
        album_image: &ImageData,
        elapsed_seconds: f32,
    ) -> Option<ImageData> {
        if width == 0 || height == 0 || album_image.width == 0 || album_image.height == 0 {
            return None;
        }

        if self
            .output
            .as_ref()
            .is_none_or(|slot| slot.size != (width, height))
        {
            if let Some(slot) = self.output.take() {
                renderer.unregister_texture(slot.image);
            }

            let texture = device.create_texture(&TextureDescriptor {
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
            let view = texture.create_view(&TextureViewDescriptor::default());
            let image = renderer.register_texture(texture);

            self.output = Some(OutputSlot {
                view,
                size: (width, height),
                image,
            });
        }

        if self
            .album
            .as_ref()
            .is_none_or(|slot| slot.size != (album_image.width, album_image.height))
        {
            let texture = device.create_texture(&TextureDescriptor {
                label: Some("cantus_warp_album_texture"),
                size: Extent3d {
                    width: album_image.width,
                    height: album_image.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8Unorm,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let view = texture.create_view(&TextureViewDescriptor::default());
            let bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: Some("cantus_warp_album_bind_group"),
                layout: &self.album_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&self.sampler),
                    },
                ],
            });
            self.album = Some(AlbumSlot {
                texture,
                bind_group,
                size: (album_image.width, album_image.height),
            });
        }

        if let Some(slot) = self.album.as_ref() {
            queue.write_texture(
                slot.texture.as_image_copy(),
                album_image.data.data(),
                TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * album_image.width),
                    rows_per_image: Some(album_image.height),
                },
                Extent3d {
                    width: album_image.width,
                    height: album_image.height,
                    depth_or_array_layers: 1,
                },
            );
        }

        let output = self.output.as_ref()?;
        let album = self.album.as_ref()?;

        let uniforms = WarpUniforms {
            resolution: [
                width as f32,
                height as f32,
                1.0 / width as f32,
                1.0 / height as f32,
            ],
            params: [
                elapsed_seconds * WARP_TIME_SCALE,
                WARP_STRENGTH,
                SWIRL_STRENGTH,
                album_image.width as f32 / album_image.height as f32,
            ],
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("cantus_warp_encoder"),
        });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("cantus_warp_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &output.view,
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
            pass.set_bind_group(0, &album.bind_group, &[]);
            pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        Some(output.image.clone())
    }
}
