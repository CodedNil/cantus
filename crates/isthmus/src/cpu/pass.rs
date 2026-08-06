use super::buffer::DataBuffer;
use crate::{
    Binding, BufferData, FilterableFloatFormat, PassContract, PassShared, Render, ResourceBindings,
    SampledTexture, SampledTextureDimension,
};
use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
    slice,
};
use std::{format, vec::Vec};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, Buffer, ColorTargetState, ColorWrites, Device,
    FragmentState, MultisampleState, PipelineCompilationOptions, PrimitiveState, Queue, RenderPass,
    RenderPipeline, RenderPipelineDescriptor, ShaderModule, TextureFormat, VertexState,
};

/// Resources shared while constructing passes.
pub struct PassBuilder<'a, Shared> {
    device: &'a Device,
    queue: &'a Queue,
    shader: &'a ShaderModule,
    format: TextureFormat,
    shared: &'a Buffer,
    shared_type: PhantomData<Shared>,
}

impl<'a, Shared: BufferData> PassBuilder<'a, Shared> {
    pub(crate) const fn new(
        device: &'a Device,
        queue: &'a Queue,
        shader: &'a ShaderModule,
        format: TextureFormat,
        shared: &'a Buffer,
    ) -> Self {
        Self {
            device,
            queue,
            shader,
            format,
            shared,
            shared_type: PhantomData,
        }
    }

    pub const fn device(&self) -> &Device {
        self.device
    }

    pub const fn queue(&self) -> &Queue {
        self.queue
    }

    pub const fn format(&self) -> TextureFormat {
        self.format
    }

    pub fn sampled_texture<D: SampledTextureDimension>(
        &self,
        label: &str,
        size: wgpu::Extent3d,
        format: FilterableFloatFormat,
    ) -> SampledTexture<D> {
        SampledTexture::new(self.device, self.queue, label, size, format)
    }

    pub fn filtering_sampler(&self, label: &str) -> crate::FilteringSampler {
        crate::FilteringSampler::new(self.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(label),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        }))
    }

    pub fn storage<T: BufferData>(
        &self,
        label: &str,
        values: impl IntoIterator<Item = T>,
    ) -> crate::Storage<T> {
        let values = values.into_iter().collect::<Vec<_>>();
        crate::Storage::new(self.device, self.queue, label, &values)
    }

    pub fn instances<S: PassContract + PassShared<Shared>>(
        &self,
        count: usize,
        resources: S::Resources<'_>,
    ) -> Pass<S> {
        Pass {
            core: self.build::<S>(count.max(1), resources.into_bindings()),
            instances: Vec::with_capacity(count),
            draw_instances: count as u32,
        }
    }

    pub fn state<S: PassContract + PassShared<Shared>>(
        &self,
        resources: S::Resources<'_>,
    ) -> StatePass<S>
    where
        S::Instance: Default,
    {
        self.state_with(resources, S::Instance::default())
    }

    pub fn state_with<S: PassContract + PassShared<Shared>>(
        &self,
        resources: S::Resources<'_>,
        state: S::Instance,
    ) -> StatePass<S> {
        StatePass {
            core: self.build::<S>(1, resources.into_bindings()),
            state,
        }
    }

    pub fn with_instances<S: PassContract + PassShared<Shared>>(
        &self,
        resources: S::Resources<'_>,
        instances: impl IntoIterator<Item = S::Instance>,
    ) -> Pass<S> {
        let instances = instances.into_iter().collect::<Vec<_>>();
        Pass {
            core: self.build::<S>(instances.len().max(1), resources.into_bindings()),
            draw_instances: instances.len() as u32,
            instances,
        }
    }

    fn build<S: PassContract + PassShared<Shared>>(
        &self,
        count: usize,
        resources: Vec<Binding>,
    ) -> PassCore {
        let name = S::NAME;
        let entry = name.replace("::", "_");
        let pipeline = self.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(name),
            layout: None,
            vertex: VertexState {
                module: self.shader,
                entry_point: Some(&format!("{entry}_vertex")),
                buffers: &[],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: self.shader,
                entry_point: Some(&format!("{entry}_fragment")),
                targets: &[Some(ColorTargetState {
                    format: self.format,
                    blend: Some(S::PIPELINE.blend),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: S::PIPELINE.topology,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let buffer = S::INSTANCE_BUFFER.then(|| {
            DataBuffer::new::<S::Instance>(self.device, self.queue, &format!("{name} Instances"), count)
        });
        let shared = S::SHARED_BUFFER.then(|| self.shared.clone());
        let bind_group = create_bind_group(
            self.device,
            name,
            &pipeline,
            shared.as_ref(),
            buffer.as_ref(),
            &resources,
        );
        PassCore {
            pipeline,
            vertices: S::PIPELINE.vertices,
            buffer,
            bind_group,
            shared,
            resources,
        }
    }
}

struct PassCore {
    pipeline: RenderPipeline,
    vertices: u32,
    buffer: Option<DataBuffer>,
    bind_group: BindGroup,
    shared: Option<Buffer>,
    resources: Vec<Binding>,
}

impl PassCore {
    fn draw<'pass>(&'pass self, pass: &mut RenderPass<'pass>, instances: Range<u32>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..self.vertices, instances);
    }

    fn upload<S: PassContract>(&mut self, instances: &[S::Instance]) {
        let grew = self
            .buffer
            .as_mut()
            .is_some_and(|buffer| buffer.grow::<S::Instance>(instances.len()));
        if let Some(buffer) = self.buffer.as_ref().filter(|_| grew) {
            self.bind_group = create_bind_group(
                buffer.device(),
                S::NAME,
                &self.pipeline,
                self.shared.as_ref(),
                Some(buffer),
                &self.resources,
            );
        }
        if let Some(buffer) = &mut self.buffer {
            buffer.upload(instances);
        }
    }
}

/// A GPU pass with a dynamic instance list.
pub struct Pass<S: PassContract> {
    core: PassCore,
    pub instances: Vec<S::Instance>,
    draw_instances: u32,
}

impl<S: PassContract> Pass<S> {
    /// Draws a subset of this pass's instances.
    ///
    /// # Panics
    /// Panics when the range exceeds the instances owned by the pass.
    pub fn draw_range<'pass>(&'pass self, pass: &mut RenderPass<'pass>, instances: Range<u32>) {
        let count = if S::INSTANCE_BUFFER {
            self.instances.len() as u32
        } else {
            self.draw_instances
        };
        assert!(
            instances.start <= instances.end && instances.end <= count,
            "draw range exceeds pass instances"
        );
        self.core.draw(pass, instances);
    }
}

impl<S: PassContract> Render for Pass<S> {
    fn prepare(&mut self) {
        self.core.upload::<S>(&self.instances);
    }

    fn draw<'pass>(&'pass self, pass: &mut RenderPass<'pass>) {
        let count = if S::INSTANCE_BUFFER {
            self.instances.len() as u32
        } else {
            self.draw_instances
        };
        self.draw_range(pass, 0..count);
    }
}

fn create_bind_group(
    device: &Device,
    name: &str,
    pipeline: &RenderPipeline,
    shared: Option<&Buffer>,
    buffer: Option<&DataBuffer>,
    resources: &[Binding],
) -> BindGroup {
    let mut entries = Vec::with_capacity(2 + resources.len());
    if let Some(shared) = shared {
        entries.push(BindGroupEntry {
            binding: 0,
            resource: shared.as_entire_binding(),
        });
    }
    if let Some(buffer) = buffer {
        entries.push(BindGroupEntry {
            binding: 1,
            resource: buffer.raw().as_entire_binding(),
        });
    }
    entries.extend(
        resources
            .iter()
            .enumerate()
            .map(|(index, resource)| BindGroupEntry {
                binding: index as u32 + 2,
                resource: resource.resource(),
            }),
    );
    device.create_bind_group(&BindGroupDescriptor {
        label: Some(&format!("{name} Bind Group")),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &entries,
    })
}

pub struct StatePass<S: PassContract> {
    core: PassCore,
    state: S::Instance,
}

impl<S: PassContract> Render for StatePass<S> {
    fn prepare(&mut self) {
        self.core.upload::<S>(slice::from_ref(&self.state));
    }

    fn draw<'pass>(&'pass self, pass: &mut RenderPass<'pass>) {
        self.core.draw(pass, 0..1);
    }
}

impl<S: PassContract> Deref for StatePass<S> {
    type Target = S::Instance;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<S: PassContract> DerefMut for StatePass<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
