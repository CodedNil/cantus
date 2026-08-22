use crate::data::ShaderData;
use std::{marker::PhantomData, rc::Rc};

pub mod buffer;
pub mod canvas;
pub mod context;
pub mod image;
pub mod renderer;
pub mod surface;

pub struct Buffer<T: ShaderData> {
    context: context::Context,
    binding: Rc<buffer::StreamBinding>,
    lifetime: BufferLifetime,
    marker: PhantomData<T>,
}

#[derive(Clone, Copy)]
pub enum BufferLifetime {
    Static,
    Frame,
}

impl<T: ShaderData> Buffer<T> {
    pub(crate) fn binding(&self) -> Rc<dyn buffer::BufferBinding> {
        self.binding.clone()
    }

    pub(crate) fn new(context: &context::Context, lifetime: BufferLifetime) -> Self {
        Self {
            context: context.clone(),
            binding: Rc::new(buffer::StreamBinding::default()),
            lifetime,
            marker: PhantomData,
        }
    }

    pub(crate) fn with_data(context: &context::Context, values: &[T]) -> Self {
        let value = Self::new(context, BufferLifetime::Static);
        value.upload(values);
        value
    }

    pub fn upload(&self, values: &[T]) {
        let range = match self.lifetime {
            BufferLifetime::Static => self.context.upload_static(values),
            BufferLifetime::Frame => self.context.upload(values),
        };
        self.binding.update(range);
    }
}
