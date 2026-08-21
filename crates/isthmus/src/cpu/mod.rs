use crate::data::BufferData;
pub use ::wgpu;
use core::marker::PhantomData;

pub(crate) mod buffer;
pub mod context;
pub mod pass;
pub mod program;
pub mod surface;
pub mod texture;

pub enum Binding {
    Texture(wgpu::TextureView),
    Sampler(wgpu::Sampler),
    Storage(wgpu::Buffer),
}

pub trait ResourceBinding {
    fn binding(&self) -> Binding;
}

impl Binding {
    pub(crate) fn resource(&self) -> wgpu::BindingResource<'_> {
        match self {
            Self::Texture(view) => wgpu::BindingResource::TextureView(view),
            Self::Sampler(sampler) => wgpu::BindingResource::Sampler(sampler),
            Self::Storage(buffer) => buffer.as_entire_binding(),
        }
    }
}

pub struct TextureView<D> {
    pub(crate) raw: wgpu::TextureView,
    pub(crate) dimension: PhantomData<D>,
}

pub struct FilteringSampler(pub(crate) wgpu::Sampler);

impl<D> TextureView<D> {
    pub(crate) const fn new(raw: wgpu::TextureView) -> Self {
        Self { raw, dimension: PhantomData }
    }

    pub const fn raw(&self) -> &wgpu::TextureView {
        &self.raw
    }
}

impl FilteringSampler {
    pub(crate) const fn new(raw: wgpu::Sampler) -> Self {
        Self(raw)
    }

    pub const fn raw(&self) -> &wgpu::Sampler {
        &self.0
    }
}

impl<D> ResourceBinding for TextureView<D> {
    fn binding(&self) -> Binding {
        Binding::Texture(self.raw.clone())
    }
}

impl ResourceBinding for FilteringSampler {
    fn binding(&self) -> Binding {
        Binding::Sampler(self.0.clone())
    }
}

impl<T: BufferData> ResourceBinding for Storage<T> {
    fn binding(&self) -> Binding {
        Binding::Storage(self.buffer.raw().clone())
    }
}

/// Typed storage shared by any number of passes.
pub struct Storage<T: BufferData> {
    buffer: buffer::DataBuffer<T>,
}

impl<T: BufferData> Storage<T> {
    pub(crate) fn new(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, values: &[T]) -> Self {
        let mut storage = Self::with_capacity(device, queue, label, values.len());
        storage.upload(values);
        storage
    }

    pub(crate) fn with_capacity(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, capacity: usize) -> Self {
        Self {
            buffer: buffer::DataBuffer::new(device, queue, label, capacity),
        }
    }

    pub fn upload(&mut self, values: &[T]) {
        self.buffer.upload(values);
    }
}
