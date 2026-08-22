#![no_std]

#[cfg(not(target_arch = "spirv"))]
extern crate std;

extern crate self as isthmus;

mod contract;
mod data;
pub mod float;
#[cfg(not(target_arch = "spirv"))]
mod frame;
#[cfg(not(target_arch = "spirv"))]
mod input;
pub mod text;

#[cfg(not(target_arch = "spirv"))]
mod backend;

pub use glam;
pub use spirv_std;

#[doc(hidden)]
pub use isthmus_macros::{data, paint, shader_module};

#[cfg(not(target_arch = "spirv"))]
pub use backend::{
    context::SetupError,
    image::{ImageArray, TextureWriteError},
    renderer::Renderer,
    surface::Surface,
};
#[cfg(not(target_arch = "spirv"))]
pub use contract::SurfaceHandle;
pub use contract::{Fragment, FrameGlobals, FrameInfo, Quad};
pub use data::{ImageHandle, ShaderData, Unorm8x4};
#[cfg(not(target_arch = "spirv"))]
pub use frame::Frame;
#[cfg(not(target_arch = "spirv"))]
pub use input::{InputEvent, Rect, Response};
pub use text::TextFragment;

#[doc(hidden)]
pub mod __private {
    pub use crate::contract::{DrawRecord, Fragment, Images, sample_image};
    #[cfg(not(target_arch = "spirv"))]
    pub use crate::{
        backend::{Buffer, canvas::Canvas, renderer::ShaderModule},
        contract::{IntoQuad, PipelineIdentity, PipelineKey, ShaderSpec, shader_module_name},
    };
    #[cfg(not(target_arch = "spirv"))]
    pub use bytemuck;

    #[cfg(not(target_arch = "spirv"))]
    pub trait Paint<Shared: crate::ShaderData>: Sized {
        fn paint(self, canvas: &mut Canvas<Shared>);
    }
}
