#[cfg(not(target_arch = "spirv"))]
use crate::{
    cpu::{Binding, ResourceBinding, Storage, TextureView, texture::SampledTextureDimension},
    data::ShaderData,
};
#[cfg(not(target_arch = "spirv"))]
use ash::vk;
pub use spirv_std::Sampler;
use spirv_std::image::{Image2d, Image2dArray};
#[cfg(not(target_arch = "spirv"))]
use std::boxed::Box;

pub type Texture2D = Image2d;
pub type Texture2DArray = Image2dArray;

/// Maps a shader-side resource type to the host object that binds it.
#[cfg(not(target_arch = "spirv"))]
pub trait ResourceType {
    type Binding: ResourceBinding;
}

#[cfg(not(target_arch = "spirv"))]
impl<T: ShaderData> ResourceType for [T] {
    type Binding = Storage<T>;
}

#[cfg(not(target_arch = "spirv"))]
impl<D: SampledTextureDimension> ResourceType for D {
    type Binding = TextureView<D>;
}

/// Coordinates for a two-triangle unit quad from its vertex index.
pub const fn quad_coord(vertex: u32) -> glam::Vec2 {
    glam::vec2((vertex & 1) as f32, (vertex >> 1) as f32)
}

/// Converts a top-left-origin pixel position to clip space.
///
/// Vulkan's positive-height viewport maps -1 NDC Y to the top edge, preserving
/// the renderer's logical top-left coordinate system.
pub fn pixel_to_ndc(pixel: glam::Vec2, screen_size: glam::Vec2) -> glam::Vec4 {
    let ndc = pixel / screen_size * 2.0 - 1.0;
    glam::vec4(ndc.x, ndc.y, 0.0, 1.0)
}

/// Fixed state used to create a render pipeline.
#[cfg(not(target_arch = "spirv"))]
#[derive(Clone, Copy)]
pub struct Pipeline {
    pub topology: vk::PrimitiveTopology,
    pub blend: Blend,
    pub vertices: u32,
}

#[cfg(not(target_arch = "spirv"))]
#[derive(Clone, Copy)]
pub enum Blend {
    Replace,
    Alpha,
    PremultipliedAlpha,
    Add,
}

/// Describes one typed render pass.
#[cfg(not(target_arch = "spirv"))]
pub trait PassContract {
    type Instance: ShaderData;
    type Resources<'a>;
    const NAME: &'static str;
    const PIPELINE: Pipeline;

    fn bindings(resources: Self::Resources<'_>) -> Box<[Binding]>;
}

/// Verifies that a pass accepts a program's shared data.
#[cfg(not(target_arch = "spirv"))]
pub trait PassShared<T: ShaderData>: PassContract {}

/// Removes the crate name from a Rust module path.
#[cfg(not(target_arch = "spirv"))]
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
