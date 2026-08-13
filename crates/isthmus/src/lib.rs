#![no_std]

#[cfg(feature = "cpu")]
extern crate std;

extern crate self as isthmus;

mod contract;
mod data;

#[cfg(feature = "cpu")]
pub mod cpu;

pub use glam;
pub use spirv_std;

#[doc(hidden)]
pub use isthmus_macros::lower_pass;
pub use isthmus_macros::{Render, Varyings, data, outline, pass, shader_module};

pub use contract::{Sampler, Texture2DArray, Vertex, reference};
#[cfg(feature = "cpu")]
pub use cpu::{
    FilteringSampler, Storage, TextureView,
    context::{Context, Render, SetupError},
    pass::{Instance, Instances, PassBuilder},
    program::Program,
    surface::{Present, SurfaceTarget},
    texture::{FilterableFloatFormat, SampledTexture, SampledTextureDimension, TextureWriteError},
    wgpu,
};
pub use data::{BufferData, Unorm8x4};

#[doc(hidden)]
pub mod __private {
    pub use crate::{contract::reference, data::align_to};
    #[cfg(feature = "cpu")]
    pub use crate::{
        contract::{PassContract, PassShared, Pipeline, pass_module_name},
        cpu::Binding,
    };
}
