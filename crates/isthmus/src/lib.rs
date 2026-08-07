#![no_std]

#[cfg(feature = "cpu")]
extern crate std;

extern crate self as isthmus;

pub mod contract;
pub mod data;

#[cfg(feature = "cpu")]
pub mod cpu;

pub use glam;
#[doc(hidden)]
pub use isthmus_macros::lower_pass;
pub use isthmus_macros::{Render, Varyings, data, outline, pass, shader_module};

#[cfg(feature = "cpu")]
pub use contract::{PassContract, PassShared, Pipeline, Texture2DArray, pass_module_name};
pub use contract::{Vertex, reference};
#[cfg(feature = "cpu")]
pub use cpu::{
    Binding, CpuResource, FilteringSampler, ResourceBindings, Storage, TextureView,
    context::{Context, Render, SetupError},
    pass::{Pass, PassBuilder, StatePass},
    program::Program,
    surface::{Present, SurfaceTarget},
    texture::{FilterableFloatFormat, SampledTexture, SampledTextureDimension, TextureWriteError},
    wgpu,
};
pub use data::{BufferCursor, BufferData, Unorm8x4};
