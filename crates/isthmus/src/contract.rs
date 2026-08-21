#[cfg(feature = "cpu")]
use crate::{
    cpu::{Binding, FilteringSampler, ResourceBinding, Storage, TextureView, texture::SampledTextureDimension},
    data::BufferData,
};
pub use spirv_std::Sampler;
use spirv_std::image::{Image2d, Image2dArray};
#[cfg(feature = "cpu")]
use std::boxed::Box;

/// Values produced by a vertex shader before Isthmus lowers them to the GPU ABI.
pub struct Vertex<T> {
    pub position: glam::Vec4,
    pub varyings: T,
}

pub type Texture2D = Image2d;
pub type Texture2DArray = Image2dArray;

/// Maps a shader-side resource type to the host object that binds it.
#[cfg(feature = "cpu")]
pub trait ResourceType {
    type Binding: ResourceBinding;
}

#[cfg(feature = "cpu")]
impl<T: BufferData> ResourceType for [T] {
    type Binding = Storage<T>;
}

#[cfg(feature = "cpu")]
impl ResourceType for Sampler {
    type Binding = FilteringSampler;
}

#[cfg(feature = "cpu")]
impl<D: SampledTextureDimension> ResourceType for D {
    type Binding = TextureView<D>;
}

/// Coordinates for a two-triangle unit quad from its vertex index.
pub const fn quad_coord(vertex: u32) -> glam::Vec2 {
    glam::vec2((vertex & 1) as f32, (vertex >> 1) as f32)
}

/// Converts a top-left-origin pixel position to clip space.
///
/// Rust-GPU's WGSL target applies the final vertex Y convention at the entry
/// point. Keep this conversion in the logical top-left space and let the
/// generated wrapper perform that one coordinate-system adjustment.
pub fn pixel_to_ndc(pixel: glam::Vec2, screen_size: glam::Vec2) -> glam::Vec4 {
    let ndc = pixel / screen_size * 2.0 - 1.0;
    glam::vec4(ndc.x, ndc.y, 0.0, 1.0)
}

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

    fn bindings(resources: Self::Resources<'_>) -> Box<[Binding]>;
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
