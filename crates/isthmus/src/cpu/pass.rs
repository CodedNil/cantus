use super::buffer::DataBuffer;
use crate::{
    contract::{PassContract, PassShared},
    cpu::{
        Binding, FilteringSampler, Storage,
        context::Render,
        texture::{FilterableFloatFormat, SampledTexture, SampledTextureDimension},
    },
    data::BufferData,
};
use core::ops::{Deref, DerefMut};
use std::{format, ops::Range, vec::Vec};
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
    shared: &'a DataBuffer<Shared>,
}

impl<'a, Shared: BufferData> PassBuilder<'a, Shared> {
    pub(super) const fn new(
        device: &'a Device,
        queue: &'a Queue,
        shader: &'a ShaderModule,
        format: TextureFormat,
        shared: &'a DataBuffer<Shared>,
    ) -> Self {
        Self {
            device,
            queue,
            shader,
            format,
            shared,
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

    pub fn filtering_sampler(&self, label: &str) -> FilteringSampler {
        FilteringSampler::new(self.device.create_sampler(&wgpu::SamplerDescriptor {
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
    ) -> Storage<T> {
        let values = values.into_iter().collect::<Vec<_>>();
        Storage::new(self.device, self.queue, label, &values)
    }

    pub fn storage_with_capacity<T: BufferData>(&self, label: &str, capacity: usize) -> Storage<T> {
        Storage::with_capacity(self.device, self.queue, label, capacity)
    }

    pub fn instances<S: PassContract + PassShared<Shared>>(
        &self,
        resources: S::Resources<'_>,
        instances: impl IntoIterator<Item = S::Instance>,
    ) -> Instances<S> {
        let instances = instances.into_iter().collect::<Vec<_>>();
        self.build(S::bindings(resources), instances)
    }

    pub fn instances_with_capacity<S: PassContract + PassShared<Shared>>(
        &self,
        resources: S::Resources<'_>,
        capacity: usize,
    ) -> Instances<S> {
        self.build(S::bindings(resources), Vec::with_capacity(capacity))
    }

    pub fn instance<S: PassContract + PassShared<Shared>>(
        &self,
        resources: S::Resources<'_>,
        instance: S::Instance,
    ) -> Instance<S> {
        self.build(S::bindings(resources), Vec::from([instance]))
    }

    fn build<S: PassContract + PassShared<Shared>, const ONE: bool>(
        &self,
        resources: Vec<Binding>,
        instances: Vec<S::Instance>,
    ) -> Instances<S, ONE> {
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
        let buffer = DataBuffer::new(
            self.device,
            self.queue,
            &format!("{name} Instances"),
            instances.capacity().max(instances.len()).max(1),
        );
        let shared = S::SHARED_BUFFER.then(|| self.shared.raw().clone());
        let bind_group =
            create_bind_group(self.device, name, &pipeline, shared.as_ref(), &buffer, &resources);
        Instances {
            pipeline,
            buffer,
            bind_group,
            shared,
            resources,
            values: instances,
        }
    }
}

/// The instance data and GPU state for one typed shader pass.
pub struct Instances<S: PassContract, const ONE: bool = false> {
    pipeline: RenderPipeline,
    buffer: DataBuffer<S::Instance>,
    bind_group: BindGroup,
    shared: Option<Buffer>,
    resources: Vec<Binding>,
    values: Vec<S::Instance>,
}

impl<S: PassContract, const ONE: bool> Render for Instances<S, ONE> {
    fn prepare(&mut self) {
        if self.buffer.grow(self.values.len()) {
            self.bind_group = create_bind_group(
                self.buffer.device(),
                S::NAME,
                &self.pipeline,
                self.shared.as_ref(),
                &self.buffer,
                &self.resources,
            );
        }
        self.buffer.upload(&self.values);
    }

    fn draw<'pass>(&'pass self, pass: &mut RenderPass<'pass>) {
        self.draw_range(pass, 0..self.values.len() as u32);
    }
}

impl<S: PassContract, const ONE: bool> Instances<S, ONE> {
    pub fn draw_range<'pass>(&'pass self, pass: &mut RenderPass<'pass>, instances: Range<u32>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..S::PIPELINE.vertices, instances);
    }
}

fn create_bind_group(
    device: &Device,
    name: &str,
    pipeline: &RenderPipeline,
    shared: Option<&Buffer>,
    buffer: &DataBuffer<impl BufferData>,
    resources: &[Binding],
) -> BindGroup {
    let mut entries = Vec::with_capacity(2 + resources.len());
    if let Some(shared) = shared {
        entries.push(BindGroupEntry {
            binding: 0,
            resource: shared.as_entire_binding(),
        });
    }
    entries.push(BindGroupEntry {
        binding: 1,
        resource: buffer.raw().as_entire_binding(),
    });
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

impl<S: PassContract> Deref for Instances<S> {
    type Target = Vec<S::Instance>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<S: PassContract> DerefMut for Instances<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

/// A typed shader pass containing exactly one instance-data value.
pub type Instance<S> = Instances<S, true>;

impl<S: PassContract> Deref for Instance<S> {
    type Target = S::Instance;

    fn deref(&self) -> &Self::Target {
        &self.values[0]
    }
}

impl<S: PassContract> DerefMut for Instance<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values[0]
    }
}
