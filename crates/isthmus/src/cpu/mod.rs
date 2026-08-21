use crate::data::ShaderData;
use std::{rc::Rc, sync::Arc};

pub(crate) mod buffer;
pub mod context;
pub mod pass;
pub mod program;
pub mod surface;
pub mod texture;

pub use ash;
pub use texture::TextureView;

pub enum Binding {
    Texture(Arc<texture::Image>),
    Sampler,
    Storage(Rc<buffer::Buffer>),
}
pub trait ResourceBinding {
    fn binding(&self) -> Binding;
}
impl<D> ResourceBinding for TextureView<D> {
    fn binding(&self) -> Binding {
        Binding::Texture(self.image.clone())
    }
}
impl<T: ShaderData> ResourceBinding for Storage<T> {
    fn binding(&self) -> Binding {
        Binding::Storage(self.buffer.raw().clone())
    }
}

pub struct Storage<T: ShaderData> {
    buffer: buffer::DataBuffer<T>,
}
impl<T: ShaderData> Storage<T> {
    pub(crate) fn new(context: &context::Context, label: &str, values: &[T]) -> Self {
        let mut value = Self::with_capacity(context, label, values.len());
        value.upload(values);
        value
    }
    pub(crate) fn with_capacity(context: &context::Context, label: &str, capacity: usize) -> Self {
        Self {
            buffer: buffer::DataBuffer::new(context, label, capacity),
        }
    }
    pub fn upload(&mut self, values: &[T]) {
        self.buffer.upload(values);
    }
}
