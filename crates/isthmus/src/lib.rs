#![no_std]

#[cfg(feature = "cpu")]
extern crate std;

extern crate self as isthmus;

pub use glam;

mod contract;
mod data;

pub use contract::Vertex;
pub use contract::reference;
#[cfg(feature = "cpu")]
pub use contract::{PassContract, PassShared, Pipeline, Texture2D, Texture2DArray, pass_module_name};
#[cfg(feature = "cpu")]
#[doc(hidden)]
pub use data::BufferCursor;
#[cfg(feature = "cpu")]
pub use data::BufferData;
pub use data::{Flag, Unorm8x4};
#[doc(hidden)]
pub use isthmus_macros::__pass_impl;
pub use isthmus_macros::{Render, Varyings, data, outline, pass, shader_module};

#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "cpu")]
pub use cpu::{
    Binding, Context, CpuResource, FilterableFloatFormat, FilteringSampler, Pass, PassBuilder, Present,
    Program, Render, ResourceBindings, SampledTexture, SampledTextureDimension, SetupError, StatePass,
    Storage, SurfaceTarget, TextureView, TextureWriteError, wgpu,
};
