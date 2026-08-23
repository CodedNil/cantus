#![no_std]

#[cfg(not(target_arch = "spirv"))]
extern crate std;

extern crate self as isthmus;

mod contract;
mod data;
pub mod float;
#[cfg(not(target_arch = "spirv"))]
mod frame;
mod image;
#[cfg(not(target_arch = "spirv"))]
mod input;
pub mod text;

#[cfg(not(target_arch = "spirv"))]
mod backend;

pub use glam;
pub use spirv_std;

pub use isthmus_macros::{Program, ShaderData, paint};

#[cfg(not(target_arch = "spirv"))]
pub use backend::{
    context::SetupError,
    renderer::{Render, Renderer},
};
#[cfg(not(target_arch = "spirv"))]
pub use contract::SurfaceHandle;
pub use contract::{Fragment, FrameInfo, Quad};
pub use data::{ShaderData, Unorm8x4};
#[cfg(not(target_arch = "spirv"))]
pub use frame::Frame;
pub use image::Image;
#[cfg(not(target_arch = "spirv"))]
pub use input::{InputEvent, Rect, Response};
pub use text::TextFragment;

#[doc(hidden)]
pub mod __private {
    pub use crate::contract::{DrawRecord, ImageHeap, ShaderImage};
    pub use crate::data::ImageHandle;
    #[cfg(not(target_arch = "spirv"))]
    pub use crate::{
        backend::renderer::ShaderModule,
        contract::{PaintPipeline, PipelineToken, Program, ShaderSpec, shader_module_name},
    };
    #[cfg(not(target_arch = "spirv"))]
    pub use bytemuck;
}
