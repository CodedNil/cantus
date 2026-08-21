use super::{
    Binding, Storage,
    buffer::DataBuffer,
    context::{Context, Inner, RenderFrame},
    texture::{Extent3d, FilterableFloatFormat, SampledTexture, SampledTextureDimension},
};
use crate::{
    contract::{Blend, PassContract, PassShared, Pipeline},
    data::ShaderData,
};
use ash::vk;
use core::{
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut},
    slice::from_ref,
};
use std::{boxed::Box, ffi::CString, format, ops::Range, sync::Arc, vec, vec::Vec};

pub struct PassBuilder<'a, Shared> {
    context: &'a Context,
    shader: vk::ShaderModule,
    format: vk::Format,
    marker: PhantomData<Shared>,
}
impl<'a, Shared: ShaderData> PassBuilder<'a, Shared> {
    pub(super) const fn new(context: &'a Context, shader: vk::ShaderModule, format: vk::Format) -> Self {
        Self {
            context,
            shader,
            format,
            marker: PhantomData,
        }
    }
    pub const fn context(&self) -> &Context {
        self.context
    }
    pub fn sampled_texture<D: SampledTextureDimension>(&self, label: &str, size: Extent3d, format: FilterableFloatFormat) -> SampledTexture<D> {
        SampledTexture::new(self.context, label, size, format)
    }
    pub fn storage<T: ShaderData>(&self, label: &str, values: impl IntoIterator<Item = T>) -> Storage<T> {
        let values = values.into_iter().collect::<Vec<_>>();
        Storage::new(self.context, label, &values)
    }
    pub fn storage_with_capacity<T: ShaderData>(&self, label: &str, capacity: usize) -> Storage<T> {
        Storage::with_capacity(self.context, label, capacity)
    }
    pub fn instances<S: PassContract + PassShared<Shared>>(&self, resources: S::Resources<'_>, instances: impl IntoIterator<Item = S::Instance>) -> Instances<S::Instance> {
        self.build::<S, false>(S::bindings(resources), instances.into_iter().collect())
    }
    pub fn instances_with_capacity<S: PassContract + PassShared<Shared>>(&self, resources: S::Resources<'_>, capacity: usize) -> Instances<S::Instance> {
        self.build::<S, false>(S::bindings(resources), Vec::with_capacity(capacity))
    }
    pub fn instance<S: PassContract + PassShared<Shared>>(&self, resources: S::Resources<'_>, instance: S::Instance) -> Instance<S::Instance> {
        self.build::<S, true>(S::bindings(resources), vec![instance])
    }
    fn build<S: PassContract + PassShared<Shared>, const ONE: bool>(&self, resources: Box<[Binding]>, values: Vec<S::Instance>) -> Instances<S::Instance, ONE> {
        let buffer = DataBuffer::new(self.context, S::NAME, values.capacity().max(values.len()).max(1));
        let gpu = GpuPass::new::<Shared>(self.context, self.shader, self.format, S::NAME, S::PIPELINE, buffer.raw().raw(), &resources);
        Instances {
            gpu,
            buffer,
            resources,
            values,
            dirty: true,
        }
    }
}

struct GpuPass {
    inner: Arc<Inner>,
    pipeline: vk::Pipeline,
    layout: vk::PipelineLayout,
    descriptor_layout: vk::DescriptorSetLayout,
    set: vk::DescriptorSet,
    vertices: u32,
}
impl GpuPass {
    fn new<Shared: ShaderData>(context: &Context, shader: vk::ShaderModule, format: vk::Format, name: &str, state: Pipeline, instances: vk::Buffer, resources: &[Binding]) -> Self {
        let inner = context.inner();
        let mut bindings = Vec::new();
        bindings.push(
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS),
        );
        bindings.extend(resources.iter().enumerate().map(|(index, resource)| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(index as u32 + 1)
                .descriptor_type(match resource {
                    Binding::Texture(_) => vk::DescriptorType::SAMPLED_IMAGE,
                    Binding::Sampler => vk::DescriptorType::SAMPLER,
                    Binding::Storage(_) => vk::DescriptorType::STORAGE_BUFFER,
                })
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS)
        }));
        let descriptor_layout = unsafe {
            inner
                .device
                .create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings), None)
        }
        .expect("descriptor layout");
        let layouts = [descriptor_layout];
        const { assert!(size_of::<Shared>() <= 128, "shared push constants exceed Vulkan 1.3's guaranteed 128 bytes") };
        let push_ranges = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS)
            .size(size_of::<Shared>() as u32)];
        let layout = unsafe {
            inner
                .device
                .create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().set_layouts(&layouts).push_constant_ranges(&push_ranges), None)
        }
        .expect("pipeline layout");
        let vertex = CString::new(format!("{name}::vertex")).unwrap();
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
        let assembly = vk::PipelineInputAssemblyStateCreateInfo::default().topology(state.topology);
        let viewport = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);
        let multisample = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let blend_attachment = blend(state.blend);
        let blend_attachments = [blend_attachment];
        let color_blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend_attachments);
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
        let set = unsafe {
            inner
                .device
                .allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(inner.descriptor_pool).set_layouts(&layouts))
        }
        .unwrap()[0];
        let result = Self {
            inner,
            pipeline,
            layout,
            descriptor_layout,
            set,
            vertices: state.vertices,
        };
        result.write(instances, resources);
        result
    }
    fn write(&self, instances: vk::Buffer, resources: &[Binding]) {
        let mut buffers = Vec::new();
        let mut images = Vec::new();
        buffers.push((0, instances));
        for (index, resource) in resources.iter().enumerate() {
            match resource {
                Binding::Storage(buffer) => buffers.push((index as u32 + 1, buffer.raw())),
                Binding::Texture(image) => images.push((index as u32 + 1, Some(image.view), None)),
                Binding::Sampler => images.push((index as u32 + 1, None, Some(self.inner.sampler))),
            }
        }
        let buffer_infos = buffers
            .iter()
            .map(|(_, raw)| vk::DescriptorBufferInfo::default().buffer(*raw).range(vk::WHOLE_SIZE))
            .collect::<Vec<_>>();
        let image_infos = images
            .iter()
            .map(|(_, image, sampler)| {
                vk::DescriptorImageInfo::default()
                    .image_view(image.unwrap_or_default())
                    .sampler(sampler.unwrap_or_default())
                    .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            })
            .collect::<Vec<_>>();
        let mut writes = Vec::new();
        for (i, (binding, _)) in buffers.iter().enumerate() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_set(self.set)
                    .dst_binding(*binding)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .buffer_info(from_ref(&buffer_infos[i])),
            );
        }
        for (i, (binding, image, _)) in images.iter().enumerate() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_set(self.set)
                    .dst_binding(*binding)
                    .descriptor_type(if image.is_some() {
                        vk::DescriptorType::SAMPLED_IMAGE
                    } else {
                        vk::DescriptorType::SAMPLER
                    })
                    .image_info(from_ref(&image_infos[i])),
            );
        }
        unsafe {
            self.inner.device.update_descriptor_sets(&writes, &[]);
        }
    }
}
impl Drop for GpuPass {
    fn drop(&mut self) {
        unsafe {
            self.inner.device.destroy_pipeline(self.pipeline, None);
            self.inner.device.destroy_pipeline_layout(self.layout, None);
            let _ = self.inner.device.free_descriptor_sets(self.inner.descriptor_pool, &[self.set]);
            self.inner.device.destroy_descriptor_set_layout(self.descriptor_layout, None);
        }
    }
}
fn blend(mode: Blend) -> vk::PipelineColorBlendAttachmentState {
    let (enable, src, dst, src_alpha, dst_alpha) = match mode {
        Blend::Replace => (false, vk::BlendFactor::ONE, vk::BlendFactor::ZERO, vk::BlendFactor::ONE, vk::BlendFactor::ZERO),
        Blend::Alpha => (
            true,
            vk::BlendFactor::SRC_ALPHA,
            vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            vk::BlendFactor::ONE,
            vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        ),
        Blend::PremultipliedAlpha => (
            true,
            vk::BlendFactor::ONE,
            vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            vk::BlendFactor::ONE,
            vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        ),
        Blend::Add => (true, vk::BlendFactor::ONE, vk::BlendFactor::ONE, vk::BlendFactor::ONE, vk::BlendFactor::ONE),
    };
    vk::PipelineColorBlendAttachmentState::default()
        .blend_enable(enable)
        .src_color_blend_factor(src)
        .dst_color_blend_factor(dst)
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(src_alpha)
        .dst_alpha_blend_factor(dst_alpha)
        .alpha_blend_op(vk::BlendOp::ADD)
        .color_write_mask(vk::ColorComponentFlags::RGBA)
}

pub struct Instances<T: ShaderData, const ONE: bool = false> {
    gpu: GpuPass,
    buffer: DataBuffer<T>,
    resources: Box<[Binding]>,
    values: Vec<T>,
    dirty: bool,
}
impl<T: ShaderData, const ONE: bool> super::context::Render for Instances<T, ONE> {
    fn prepare(&mut self) {
        if !self.dirty {
            return;
        }
        if self.buffer.grow(self.values.len()) {
            self.gpu.write(self.buffer.raw().raw(), &self.resources);
        }
        self.buffer.upload(&self.values);
        self.dirty = false;
    }
    fn draw(&self, frame: &RenderFrame<'_>) {
        self.draw_range(frame, 0..self.values.len() as u32);
    }
}
impl<T: ShaderData, const ONE: bool> Instances<T, ONE> {
    /// Records a draw for the selected instance range.
    ///
    /// # Panics
    /// Panics if the range is reversed or exceeds the instance list.
    pub fn draw_range(&self, frame: &RenderFrame<'_>, instances: Range<u32>) {
        assert!(instances.start <= instances.end && instances.end <= self.values.len() as u32, "invalid instance draw range");
        let command = frame.command();
        unsafe {
            self.gpu.inner.device.cmd_bind_pipeline(command, vk::PipelineBindPoint::GRAPHICS, self.gpu.pipeline);
            self.gpu
                .inner
                .device
                .cmd_push_constants(command, self.gpu.layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, frame.shared());
            self.gpu
                .inner
                .device
                .cmd_bind_descriptor_sets(command, vk::PipelineBindPoint::GRAPHICS, self.gpu.layout, 0, &[self.gpu.set], &[]);
            self.gpu
                .inner
                .device
                .cmd_draw(command, self.gpu.vertices, instances.end - instances.start, 0, instances.start);
        }
    }
}
impl<T: ShaderData> Deref for Instances<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
impl<T: ShaderData> DerefMut for Instances<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;
        &mut self.values
    }
}
pub type Instance<T> = Instances<T, true>;
impl<T: ShaderData> Deref for Instance<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.values[0]
    }
}
impl<T: ShaderData> DerefMut for Instance<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;
        &mut self.values[0]
    }
}
