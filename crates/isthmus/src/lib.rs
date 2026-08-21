#![no_std]

#[cfg(not(target_arch = "spirv"))]
extern crate std;

extern crate self as isthmus;

mod contract;
mod data;

#[cfg(not(target_arch = "spirv"))]
pub mod cpu;

pub use glam;
pub use spirv_std;

#[doc(hidden)]
pub use isthmus_macros::{Render, data, host, shader, shader_module, system};

pub use contract::{Sampler, Texture2D, Texture2DArray, pixel_to_ndc, quad_coord};
#[cfg(not(target_arch = "spirv"))]
pub use cpu::{
    Storage, TextureView, ash,
    context::{Context, Render, RenderFrame, SetupError},
    pass::{Instance, Instances, PassBuilder},
    program::{Color, Program},
    surface::{Present, Surface, SurfaceTarget},
    texture::{Extent3d, FilterableFloatFormat, SampledTexture, SampledTextureDimension, TextureWriteError},
};
pub use data::{ShaderData, Unorm8x4};

#[doc(hidden)]
pub mod __private {
    #[cfg(not(target_arch = "spirv"))]
    pub use crate::{
        contract::{Blend, PassContract, PassShared, Pipeline, ResourceType, pass_module_name},
        cpu::{Binding, ResourceBinding},
    };
    #[cfg(not(target_arch = "spirv"))]
    pub use bytemuck;
}
