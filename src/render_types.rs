use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState,
    BufferBindingType, ColorTargetState, ColorWrites, Device, FragmentState, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology,
    RenderPipeline, RenderPipelineDescriptor, SamplerBindingType, ShaderModule,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureFormat, TextureSampleType,
    TextureViewDimension, VertexState,
};

pub struct Shaders {
    pub playhead_pipeline: RenderPipeline,
    pub playhead_bind_group_layout: BindGroupLayout,
    pub bg_pipeline: RenderPipeline,
    pub bg_bind_group_layout: BindGroupLayout,
    pub icon_pipeline: RenderPipeline,
    pub icon_bind_group_layout: BindGroupLayout,
    pub text_pipeline: RenderPipeline,
    pub text_bind_group_layout: BindGroupLayout,
}

impl Shaders {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        // Shader Modules
        let playhead_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Playhead Shader"),
            source: ShaderSource::Wgsl(include_str!("../assets/playhead.wgsl").into()),
        });
        let bg_shader = device.create_shader_module(ShaderModuleDescriptor {
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

        // Bind Group Layouts
        let playhead_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Playhead Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let standard_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Standard Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2Array,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 3,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let text_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Text Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

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
        let bg_pipeline = create_pipe("Background", &bg_shader, &standard_bind_group_layout);
        let icon_pipeline = create_pipe("Icons", &icon_shader, &standard_bind_group_layout);
        let text_pipeline = create_pipe("Text", &text_shader, &text_bind_group_layout);

        Self {
            playhead_pipeline,
            playhead_bind_group_layout,
            bg_pipeline,
            bg_bind_group_layout: standard_bind_group_layout.clone(),
            icon_pipeline,
            icon_bind_group_layout: standard_bind_group_layout,
            text_pipeline,
            text_bind_group_layout,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl Rect {
    pub const fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self { x0, y0, x1, y1 }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x0 && p.x <= self.x1 && p.y >= self.y0 && p.y <= self.y1
    }

    pub fn inflate(&self, dx: f32, dy: f32) -> Self {
        Self {
            x0: self.x0 - dx,
            y0: self.y0 - dy,
            x1: self.x1 + dx,
            y1: self.y1 + dy,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ScreenUniforms {
    pub screen_size: [f32; 2],
    pub time: f32,
    pub scale_factor: f32,
    pub mouse_pos: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct PlayheadUniforms {
    pub origin_x: f32,
    pub panel_start: f32,
    pub height: f32,
    pub volume: f32,
    pub bar_lerp: f32,
    pub play_lerp: f32,
    pub pause_lerp: f32,
    pub _padding: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct Particle {
    pub spawn_pos: [f32; 2],
    pub spawn_vel: [f32; 2],
    pub spawn_time: f32,
    pub duration: f32,
    pub color: u32,
    pub _padding: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct BackgroundPill {
    pub rect: [f32; 4], // x, y, width, height
    pub dark_width: f32,
    pub alpha: f32,
    pub colors: [u32; 4],
    pub expansion_pos: [f32; 2],
    pub expansion_time: f32,
    pub image_index: i32,
    pub _padding: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct IconInstance {
    pub pos: [f32; 2],
    pub alpha: f32,
    pub variant: f32,
    pub param: f32,
    pub image_index: i32,
    pub _padding: [f32; 2],
}
