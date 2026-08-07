#![cfg_attr(target_arch = "spirv", no_std)]

#[cfg(feature = "cpu")]
pub mod app;
#[cfg(feature = "shader")]
pub mod render;
