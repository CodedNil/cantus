#[cfg(feature = "cpu")]
use crate::{cpu::Binding, data::BufferData};
pub use spirv_std::Sampler;
use spirv_std::image::Image2dArray;
#[cfg(feature = "cpu")]
use std::vec::Vec;

/// Values produced by a vertex shader before Isthmus lowers them to the GPU ABI.
pub struct Vertex<T> {
    pub position: glam::Vec4,
    pub varyings: T,
}

pub type Texture2DArray = Image2dArray;

/// Borrows a value directly from a generated shader storage binding.
pub fn reference<T>(records: &[T], index: usize) -> &T {
    #[cfg(target_arch = "spirv")]
    // SAFETY: generated indices address buffers created from the same pass contract.
    unsafe {
        records.get_unchecked(index)
    }
    #[cfg(not(target_arch = "spirv"))]
    &records[index]
}

/// Fixed state used to create a render pipeline.
#[cfg(feature = "cpu")]
#[derive(Clone, Copy)]
pub struct Pipeline {
    pub topology: wgpu::PrimitiveTopology,
    pub blend: wgpu::BlendState,
    pub vertices: u32,
}

/// Describes one typed render pass.
#[cfg(feature = "cpu")]
pub trait PassContract {
    type Instance: BufferData;
    type Resources<'a>;
    const NAME: &'static str;
    const PIPELINE: Pipeline;

    fn bindings(resources: Self::Resources<'_>) -> Vec<Binding>;
}

/// Verifies that a pass accepts a program's shared data.
#[cfg(feature = "cpu")]
pub trait PassShared<T: BufferData>: PassContract {
    const SHARED_BUFFER: bool;
}

/// Removes the crate name from a Rust module path.
#[cfg(feature = "cpu")]
#[must_use]
pub const fn pass_module_name(path: &str) -> &str {
    let bytes = path.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i] != b':' {
        i += 1;
    }
    while i < bytes.len() && bytes[i] == b':' {
        i += 1;
    }
    path.split_at(i).1
}
