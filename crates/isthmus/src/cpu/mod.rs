use core::marker::PhantomData;
use std::vec::Vec;

#[doc(hidden)]
pub use ::wgpu;

mod buffer;
mod context;
mod pass;
mod program;
mod surface;
mod texture;

pub use context::{Context, Render, SetupError};
pub use pass::{Pass, PassBuilder, StatePass};
pub use program::Program;
pub use surface::{Present, SurfaceTarget};
pub use texture::{FilterableFloatFormat, SampledTexture, SampledTextureDimension, TextureWriteError};

#[doc(hidden)]
pub trait CpuResource {
    fn clone_binding(&self) -> Binding;
}

#[doc(hidden)]
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

    /// Wraps a view that matches the shader's dimension and sample type.
    ///
    /// # Safety
    /// The view must be filterable floating-point data with dimension `D`.
    pub const unsafe fn from_raw(raw: wgpu::TextureView) -> Self {
        Self::new(raw)
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

impl<T: crate::BufferData> CpuResource for Storage<T> {
    fn clone_binding(&self) -> Binding {
        Binding::Storage(self.raw.clone())
    }
}

/// Immutable typed data uploaded once and shared by any number of passes.
pub struct Storage<T: crate::BufferData> {
    raw: wgpu::Buffer,
    marker: PhantomData<T>,
}

impl<T: crate::BufferData> Storage<T> {
    pub(crate) fn new(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, values: &[T]) -> Self {
        let raw = buffer::storage(device, queue, label, values);
        Self {
            raw,
            marker: PhantomData,
        }
    }
}

#[doc(hidden)]
pub trait ResourceBindings {
    fn into_bindings(self) -> Vec<Binding>;
}

impl ResourceBindings for () {
    fn into_bindings(self) -> Vec<Binding> {
        Vec::new()
    }
}
