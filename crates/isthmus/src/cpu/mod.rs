use crate::data::BufferData;
pub use ::wgpu;
use core::marker::PhantomData;
use std::vec::Vec;

pub(crate) mod buffer;
pub mod context;
pub mod pass;
pub mod program;
pub mod surface;
pub mod texture;

pub trait CpuResource {
    fn clone_binding(&self) -> Binding;
}

pub enum Binding {
    Texture(wgpu::TextureView),
    Sampler(wgpu::Sampler),
    Storage(wgpu::Buffer),
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
        Self {
            raw,
            dimension: PhantomData,
        }
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

impl<D> CpuResource for TextureView<D> {
    fn clone_binding(&self) -> Binding {
        Binding::Texture(self.raw.clone())
    }
}

impl CpuResource for FilteringSampler {
    fn clone_binding(&self) -> Binding {
        Binding::Sampler(self.0.clone())
    }
}

impl<T: BufferData> CpuResource for Storage<T> {
    fn clone_binding(&self) -> Binding {
        Binding::Storage(self.raw.clone())
    }
}

/// Immutable typed data uploaded once and shared by any number of passes.
pub struct Storage<T: BufferData> {
    raw: wgpu::Buffer,
    marker: PhantomData<T>,
}

impl<T: BufferData> Storage<T> {
    pub(crate) fn new(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, values: &[T]) -> Self {
        let raw = buffer::storage(device, queue, label, values);
        Self {
            raw,
            marker: PhantomData,
        }
    }
}

pub trait ResourceBindings {
    fn into_bindings(self) -> Vec<Binding>;
}

impl ResourceBindings for () {
    fn into_bindings(self) -> Vec<Binding> {
        Vec::new()
    }
}
