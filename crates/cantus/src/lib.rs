#![cfg_attr(target_arch = "spirv", no_std)]
#![cfg_attr(target_arch = "spirv", allow(dead_code, unused_imports))]

#[cfg(not(target_arch = "spirv"))]
pub mod app;
pub mod render;
