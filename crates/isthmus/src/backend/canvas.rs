use super::{
    Buffer, BufferLifetime,
    buffer::{BufferBinding, BufferRange},
    context::{Context, Inner, RenderFrame},
    image::{ImageArray, TextureHeap},
};
use crate::{
    contract::{DrawRecord, PipelineIdentity, PipelineKey, Quad, ShaderSpec, SurfaceHandle},
    data::ShaderData,
    text::{Edge, Glyph, Line, PlacedGlyph},
};
use ash::vk;
use core::{marker::PhantomData, mem::size_of, slice::from_ref};
use smallvec::SmallVec;
use std::{ffi::CString, format, rc::Rc, sync::Arc, vec::Vec};

const IMAGE_CAPACITY: u32 = 16;

struct GpuPipeline {
    inner: Arc<Inner>,
    pipeline: vk::Pipeline,
    layout: Rc<PipelineLayout>,
}

impl GpuPipeline {
    fn new(context: &Context, shader: vk::ShaderModule, format: vk::Format, root: &str, name: &str, layout: Rc<PipelineLayout>) -> Self {
        let inner = context.inner();
        let vertex = CString::new(format!("{root}::__isthmus_quad::vertex")).unwrap();
        let fragment = CString::new(format!("{name}::fragment")).unwrap();
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(shader)
                .name(&vertex),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(shader)
                .name(&fragment),
        ];
        let assembly = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_STRIP);
        let viewport = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);
        let multisample = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let blend_attachment = blend();
        let color_blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(from_ref(&blend_attachment));
        let dynamic = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic);
        let mut rendering = vk::PipelineRenderingCreateInfo::default().color_attachment_formats(from_ref(&format));
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&assembly)
            .viewport_state(&viewport)
            .rasterization_state(&raster)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            .dynamic_state(&dynamic_state)
            .layout(layout.raw)
            .push_next(&mut rendering);
        let pipeline = unsafe { inner.device.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None) }.unwrap()[0];
        Self { inner, pipeline, layout }
    }

    fn draw(&self, frame: &RenderFrame<'_>, draws: BufferRange, payload: BufferRange, text: &[Rc<dyn BufferBinding>; 3], images: &TextureHeap, first: u32, count: u32) {
        let command = frame.command();
        let text = text.each_ref().map(|buffer| buffer.range());
        let buffers = [draws, payload, text[0], text[1], text[2]];
        let buffer_infos = buffers.map(|range| vk::DescriptorBufferInfo::default().buffer(range.raw).offset(range.offset).range(range.size));
        let image_infos = images
            .images()
            .iter()
            .map(|image| {
                vk::DescriptorImageInfo::default()
                    .sampler(self.inner.sampler)
                    .image_view(image.view)
                    .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            })
            .collect::<SmallVec<[vk::DescriptorImageInfo; IMAGE_CAPACITY as usize]>>();
        let mut writes = SmallVec::<[vk::WriteDescriptorSet<'_>; 6]>::new();
        for (binding, info) in buffer_infos.iter().enumerate() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_binding(binding as u32)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .buffer_info(from_ref(info)),
            );
        }
        if !image_infos.is_empty() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_binding(5)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(&image_infos),
            );
        }
        unsafe {
            self.inner.device.cmd_bind_pipeline(command, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
            self.inner
                .device
                .cmd_push_constants(command, self.layout.raw, vk::ShaderStageFlags::ALL_GRAPHICS, 0, frame.shared());
            self.inner
                .push_descriptors
                .cmd_push_descriptor_set(command, vk::PipelineBindPoint::GRAPHICS, self.layout.raw, 0, &writes);
            self.inner.device.cmd_draw(command, 4, count, 0, first);
        }
    }
}

impl Drop for GpuPipeline {
    fn drop(&mut self) {
        unsafe { self.inner.device.destroy_pipeline(self.pipeline, None) };
    }
}

struct PipelineLayout {
    inner: Arc<Inner>,
    raw: vk::PipelineLayout,
    descriptors: vk::DescriptorSetLayout,
}

impl PipelineLayout {
    fn new<Shared: ShaderData>(context: &Context) -> Self {
        const { assert!(size_of::<Shared>() <= 128, "shared push constants exceed Vulkan 1.3's guaranteed 128 bytes") };
        let inner = context.inner();
        let bindings = [
            storage_binding(0),
            storage_binding(1),
            storage_binding(2),
            storage_binding(3),
            storage_binding(4),
            vk::DescriptorSetLayoutBinding::default()
                .binding(5)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(IMAGE_CAPACITY)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT),
        ];
        let flags = [
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::PARTIALLY_BOUND,
        ];
        let mut flags = vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&flags);
        let descriptors = unsafe {
            inner.device.create_descriptor_set_layout(
                &vk::DescriptorSetLayoutCreateInfo::default()
                    .flags(vk::DescriptorSetLayoutCreateFlags::PUSH_DESCRIPTOR_KHR)
                    .bindings(&bindings)
                    .push_next(&mut flags),
                None,
            )
        }
        .expect("descriptor layout");
        let ranges = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS)
            .size(size_of::<Shared>() as u32)];
        let raw = unsafe {
            inner.device.create_pipeline_layout(
                &vk::PipelineLayoutCreateInfo::default().set_layouts(from_ref(&descriptors)).push_constant_ranges(&ranges),
                None,
            )
        }
        .expect("pipeline layout");
        Self { inner, raw, descriptors }
    }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe {
            self.inner.device.destroy_pipeline_layout(self.raw, None);
            self.inner.device.destroy_descriptor_set_layout(self.descriptors, None);
        }
    }
}

fn storage_binding(binding: u32) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS)
}

fn blend() -> vk::PipelineColorBlendAttachmentState {
    vk::PipelineColorBlendAttachmentState::default()
        .blend_enable(true)
        .src_color_blend_factor(vk::BlendFactor::ONE)
        .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
        .alpha_blend_op(vk::BlendOp::ADD)
        .color_write_mask(vk::ColorComponentFlags::RGBA)
}

pub struct Canvas<Shared> {
    context: Context,
    shader: vk::ShaderModule,
    format: vk::Format,
    root: &'static str,
    layout: Rc<PipelineLayout>,
    pipelines: Vec<RegisteredPipeline>,
    default_text: Option<PipelineKey>,
    draws: Vec<DrawRecord>,
    payload: Vec<u8>,
    uploaded_draws: BufferRange,
    uploaded_payload: BufferRange,
    paints: Vec<Vec<Paint>>,
    surface: Option<SurfaceHandle>,
    images: TextureHeap,
    text: Option<[Rc<dyn BufferBinding>; 3]>,
    marker: PhantomData<Shared>,
}

impl<Shared: ShaderData> Canvas<Shared> {
    pub(crate) fn new(context: &Context, shader: vk::ShaderModule, format: vk::Format, root: &'static str) -> Self {
        Self {
            context: context.clone(),
            shader,
            format,
            root,
            layout: Rc::new(PipelineLayout::new::<Shared>(context)),
            pipelines: Vec::new(),
            default_text: None,
            draws: Vec::new(),
            payload: Vec::new(),
            uploaded_draws: BufferRange::default(),
            uploaded_payload: BufferRange::default(),
            paints: Vec::new(),
            surface: None,
            images: TextureHeap::new(IMAGE_CAPACITY),
            text: None,
            marker: PhantomData,
        }
    }

    pub fn image_array(&self, size: [u32; 2], layers: usize) -> ImageArray {
        ImageArray::new(&self.context, &self.images, size, layers)
    }

    pub fn storage<T: ShaderData>(&self, values: impl IntoIterator<Item = T>) -> Buffer<T> {
        Buffer::with_data(&self.context, &values.into_iter().collect::<Vec<_>>())
    }

    pub fn stream<T: ShaderData>(&self) -> Buffer<T> {
        Buffer::new(&self.context, BufferLifetime::Frame)
    }

    pub(crate) fn register_text(&mut self, placed: &Buffer<PlacedGlyph>, glyphs: &Buffer<Glyph>, edges: &Buffer<Edge>) {
        self.text = Some([placed.binding(), glyphs.binding(), edges.binding()]);
    }

    pub fn begin_frame(&mut self) {
        self.draws.clear();
        self.payload.clear();
        self.uploaded_draws = BufferRange::default();
        self.uploaded_payload = BufferRange::default();
        for paints in &mut self.paints {
            paints.clear();
        }
        self.images.begin_frame();
        self.surface = Some(SurfaceHandle::PRIMARY);
    }

    pub(crate) const fn replace_surface(&mut self, surface: SurfaceHandle) -> SurfaceHandle {
        self.surface.replace(surface).expect("frame has no active surface")
    }

    #[doc(hidden)]
    pub fn register<S>(&mut self)
    where
        S: ShaderSpec<Shared = Shared> + PipelineIdentity,
    {
        const {
            assert!(size_of::<S::Instance>() > 0, "shader payloads cannot be zero-sized");
            assert!(size_of::<S::Instance>() % 4 == 0, "shader payload size must be a multiple of four bytes");
        }
        self.pipelines.push(RegisteredPipeline { name: S::NAME, gpu: None });
    }

    #[doc(hidden)]
    pub fn register_default_text<S>(&mut self)
    where
        S: ShaderSpec<Shared = Shared, Instance = Line> + PipelineIdentity,
    {
        self.register::<S>();
        self.default_text = Some(S::KEY);
    }

    #[doc(hidden)]
    pub fn emit<S>(&mut self, quad: Quad, value: S::Instance)
    where
        S: ShaderSpec<Shared = Shared> + PipelineIdentity,
    {
        self.emit_raw(S::KEY, quad, bytemuck::bytes_of(&value));
    }

    pub(crate) fn emit_text(&mut self, line: Line) {
        if line.is_empty() {
            return;
        }
        self.emit_raw(self.default_text.expect("default text pipeline is not registered"), line.quad(), bytemuck::bytes_of(&line));
    }

    fn emit_raw(&mut self, key: PipelineKey, quad: Quad, value: &[u8]) {
        let pipeline = key.index();
        if self.pipelines[pipeline].gpu.is_none() {
            let name = self.pipelines[pipeline].name;
            self.pipelines[pipeline].gpu = Some(GpuPipeline::new(&self.context, self.shader, self.format, self.root, name, Rc::clone(&self.layout)));
        }
        let payload = u32::try_from(self.payload.len()).expect("frame payload exceeds four gigabytes");
        self.payload.extend_from_slice(value);
        let draw = u32::try_from(self.draws.len()).expect("frame draw count exceeds u32");
        self.draws.push(DrawRecord { quad, payload });
        let surface = self.surface.expect("paint emitted outside a surface scope").index();
        if self.paints.len() <= surface {
            self.paints.resize_with(surface + 1, Vec::new);
        }
        self.paints[surface].push(Paint { pipeline: key, draw });
    }

    pub(crate) fn prepare(&mut self) {
        self.uploaded_draws = self.context.upload(&self.draws);
        self.uploaded_payload = self.context.upload_bytes(&self.payload);
    }

    pub(crate) fn draw_surface(&self, frame: &RenderFrame<'_>, surface: SurfaceHandle) {
        let Some(paints) = self.paints.get(surface.index()) else { return };
        let text = self.text.as_ref().expect("text renderer is not initialized");
        let mut start = 0;
        while start < paints.len() {
            let first = paints[start];
            let mut end = start + 1;
            while end < paints.len() && paints[end].pipeline == first.pipeline && paints[end].draw == paints[end - 1].draw + 1 {
                end += 1;
            }
            self.pipelines[first.pipeline.index()].gpu.as_ref().expect("paint pipeline is not initialized").draw(
                frame,
                self.uploaded_draws,
                self.uploaded_payload,
                text,
                &self.images,
                first.draw,
                (end - start) as u32,
            );
            start = end;
        }
    }
}

#[derive(Clone, Copy)]
struct Paint {
    pipeline: PipelineKey,
    draw: u32,
}

struct RegisteredPipeline {
    name: &'static str,
    gpu: Option<GpuPipeline>,
}
