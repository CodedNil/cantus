use super::{
    buffer::BufferRange,
    context::{Context, Inner, RenderFrame},
    image::ImageCache,
};
use crate::{
    contract::{DrawRecord, PaintPipeline, PushBlock, Quad, ShaderSpec, SurfaceHandle},
    data::ImageHandle,
};
use ash::vk;
use core::{array::from_fn, mem::size_of, slice::from_ref};
use smallvec::SmallVec;
use std::{collections::HashMap, ffi::CString, format, rc::Rc, sync::Arc, vec::Vec};

const IMAGE_CAPACITY: u32 = 16;

struct GpuPipeline {
    inner: Rc<Inner>,
    pipeline: vk::Pipeline,
}

impl GpuPipeline {
    fn new(context: &Context, shader: &[u32], format: vk::Format, root: &str, name: &str, layout: vk::PipelineLayout) -> Self {
        let inner = context.0.clone();
        let vertex = CString::new(format!("{root}::__isthmus_quad::vertex")).unwrap();
        let fragment = CString::new(format!("{name}::fragment")).unwrap();
        let mut vertex_module = vk::ShaderModuleCreateInfo::default().code(shader);
        let mut fragment_module = vk::ShaderModuleCreateInfo::default().code(shader);
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .name(&vertex)
                .push_next(&mut vertex_module),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .name(&fragment)
                .push_next(&mut fragment_module),
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
            .layout(layout)
            .push_next(&mut rendering);
        let pipeline = unsafe { inner.device.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None) }.unwrap()[0];
        Self { inner, pipeline }
    }

    fn draw(&self, command: vk::CommandBuffer, first: u32, count: u32) {
        unsafe {
            self.inner.device.cmd_bind_pipeline(command, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
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
    inner: Rc<Inner>,
    raw: vk::PipelineLayout,
    descriptors: vk::DescriptorSetLayout,
}

impl PipelineLayout {
    fn new(context: &Context) -> Self {
        let inner = context.0.clone();
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
            storage_binding(6),
        ];
        let flags = [
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::PARTIALLY_BOUND,
            vk::DescriptorBindingFlags::empty(),
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
            .size(size_of::<PushBlock>() as u32)];
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

pub struct Canvas {
    context: Context,
    shader: Vec<u32>,
    format: vk::Format,
    root: &'static str,
    pipelines: HashMap<&'static str, GpuPipeline>,
    layout: PipelineLayout,
    draws: Vec<DrawRecord>,
    payload: Vec<u8>,
    uploaded_draws: BufferRange,
    uploaded_payload: BufferRange,
    paints: Vec<Vec<Paint>>,
    group: Option<(SurfaceHandle, [Vec<Paint>; 2])>,
    images: ImageCache,
    text: [BufferRange; 3],
    globals: Vec<BufferRange>,
}

impl Canvas {
    pub(crate) fn new(context: &Context, shader: Vec<u32>, format: vk::Format, root: &'static str) -> Self {
        Self {
            context: context.clone(),
            shader,
            format,
            root,
            pipelines: HashMap::new(),
            layout: PipelineLayout::new(context),
            draws: Vec::new(),
            payload: Vec::new(),
            uploaded_draws: BufferRange::default(),
            uploaded_payload: BufferRange::default(),
            paints: Vec::new(),
            group: None,
            images: ImageCache::new(IMAGE_CAPACITY),
            text: [BufferRange::default(); 3],
            globals: Vec::new(),
        }
    }

    pub(crate) fn image(&mut self, size: [u32; 2], pixels: &Arc<[u8]>) -> ImageHandle {
        self.images.image(&self.context, size, pixels)
    }

    pub(crate) fn context(&self) -> Context {
        self.context.clone()
    }

    pub(crate) const fn format(&self) -> vk::Format {
        self.format
    }

    pub(crate) const fn register_text(&mut self, glyphs: BufferRange, edges: BufferRange) {
        self.text[1] = glyphs;
        self.text[2] = edges;
    }

    pub(crate) fn set_globals<T: crate::ShaderData>(&mut self, surface: SurfaceHandle, globals: T) {
        let index = surface.index();
        if self.globals.len() <= index {
            self.globals.resize(index + 1, BufferRange::default());
        }
        self.globals[index] = self.context.upload(&[globals]);
    }

    pub(crate) fn ensure_globals(&mut self, surface: SurfaceHandle) {
        if self.globals.get(surface.index()).is_none_or(|globals| globals.size == 0) {
            self.set_globals(surface, 0u32);
        }
    }

    pub(super) fn begin_frame(&mut self) {
        self.draws.clear();
        self.payload.clear();
        self.uploaded_draws = BufferRange::default();
        self.uploaded_payload = BufferRange::default();
        self.globals.clear();
        for paints in &mut self.paints {
            paints.clear();
        }
        self.group = None;
        self.images.begin_frame();
    }

    pub(crate) fn emit<S>(&mut self, surface: SurfaceHandle, quad: Quad, value: S::Instance)
    where
        S: ShaderSpec,
    {
        const {
            assert!(size_of::<S::Instance>() % 4 == 0, "shader payload size must be a multiple of four bytes");
        }
        let paint = self.record(S::PIPELINE, quad, bytemuck::bytes_of(&value));
        self.surface(surface).push(paint);
    }

    pub(crate) fn emit_layer<S>(&mut self, layer: u8, quad: Quad, value: S::Instance)
    where
        S: ShaderSpec,
    {
        const {
            assert!(size_of::<S::Instance>() % 4 == 0, "shader payload size must be a multiple of four bytes");
        }
        let paint = self.record(S::PIPELINE, quad, bytemuck::bytes_of(&value));
        self.group.as_mut().expect("paint layer outside a group").1[layer as usize].push(paint);
    }

    fn record(&mut self, spec: PaintPipeline, quad: Quad, value: &[u8]) -> Paint {
        self.pipelines
            .entry(spec.entry)
            .or_insert_with(|| GpuPipeline::new(&self.context, &self.shader, self.format, self.root, spec.entry, self.layout.raw));
        let payload = u32::try_from(self.payload.len()).expect("frame payload exceeds four gigabytes");
        self.payload.extend_from_slice(value);
        let draw = u32::try_from(self.draws.len()).expect("frame draw count exceeds u32");
        self.draws.push(DrawRecord { quad, payload });
        Paint { pipeline: spec.entry, draw }
    }

    fn surface(&mut self, surface: SurfaceHandle) -> &mut Vec<Paint> {
        let surface = surface.index();
        if self.paints.len() <= surface {
            self.paints.resize_with(surface + 1, Vec::new);
        }
        &mut self.paints[surface]
    }

    pub(crate) fn begin_group(&mut self, surface: SurfaceHandle) {
        assert!(self.group.is_none(), "paint groups cannot be nested");
        self.group = Some((surface, from_fn(|_| Vec::new())));
    }

    pub(crate) fn end_group(&mut self) {
        let (surface, layers) = self.group.take().expect("paint group was not started");
        self.surface(surface).extend(layers.into_iter().flatten());
    }

    pub(crate) fn prepare(&mut self, placed_glyphs: BufferRange) {
        self.text[0] = placed_glyphs;
        self.uploaded_draws = self.context.upload(&self.draws);
        self.uploaded_payload = self.context.upload_bytes(&self.payload);
    }

    pub(super) fn has_draws(&self, surface: SurfaceHandle) -> bool {
        self.paints.get(surface.index()).is_some_and(|paints| !paints.is_empty())
    }

    pub(super) fn draw_surface(&self, frame: &RenderFrame<'_>, surface: SurfaceHandle) {
        let Some(paints) = self.paints.get(surface.index()) else { return };
        if paints.is_empty() {
            return;
        }
        self.bind_frame(frame, surface);
        let mut start = 0;
        while start < paints.len() {
            let first = paints[start];
            let mut end = start + 1;
            while end < paints.len() && paints[end].pipeline == first.pipeline && paints[end].draw == paints[end - 1].draw + 1 {
                end += 1;
            }
            self.pipelines
                .get(first.pipeline)
                .expect("paint pipeline is not initialized")
                .draw(frame.command, first.draw, (end - start) as u32);
            start = end;
        }
    }

    fn bind_frame(&self, frame: &RenderFrame<'_>, surface: SurfaceHandle) {
        let buffers = [self.uploaded_draws, self.uploaded_payload, self.text[0], self.text[1], self.text[2]];
        let buffer_infos = buffers.map(|range| vk::DescriptorBufferInfo::default().buffer(range.raw).offset(range.offset).range(range.size));
        let image_infos = self
            .images
            .images()
            .map(|image| {
                vk::DescriptorImageInfo::default()
                    .sampler(self.context.0.sampler)
                    .image_view(image.view)
                    .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            })
            .collect::<SmallVec<[vk::DescriptorImageInfo; IMAGE_CAPACITY as usize]>>();
        let globals = self.globals[surface.index()];
        let globals = vk::DescriptorBufferInfo::default().buffer(globals.raw).offset(globals.offset).range(globals.size);
        let mut writes = SmallVec::<[vk::WriteDescriptorSet<'_>; 7]>::new();
        for (binding, info) in buffer_infos.iter().enumerate() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_binding(binding as u32)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .buffer_info(from_ref(info)),
            );
        }
        writes.push(
            vk::WriteDescriptorSet::default()
                .dst_binding(6)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(from_ref(&globals)),
        );
        if !image_infos.is_empty() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_binding(5)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(&image_infos),
            );
        }
        unsafe {
            self.context
                .0
                .device
                .cmd_push_constants(frame.command, self.layout.raw, vk::ShaderStageFlags::ALL_GRAPHICS, 0, frame.shared);
            self.context
                .0
                .push_descriptors
                .cmd_push_descriptor_set(frame.command, vk::PipelineBindPoint::GRAPHICS, self.layout.raw, 0, &writes);
        }
    }
}

#[derive(Clone, Copy)]
struct Paint {
    pipeline: &'static str,
    draw: u32,
}
