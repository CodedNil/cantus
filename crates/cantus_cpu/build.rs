use spirv_builder::{SpirvBuilder, SpirvMetadata};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("cantus_cpu must live two levels below the workspace root");
    let shader_crate = workspace_dir.join("crates/cantus_gpu");
    let shared_crate = workspace_dir.join("crates/cantus_shared");
    let dest_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("cantus.spv");

    println!("cargo:rerun-if-changed={}", shader_crate.display());
    println!("cargo:rerun-if-changed={}", shared_crate.display());
    println!(
        "cargo:rerun-if-changed={}",
        workspace_dir.join("rust-toolchain.toml").display()
    );

    let result = SpirvBuilder::new(shader_crate, "spirv-unknown-vulkan1.1")
        .spirv_metadata(SpirvMetadata::None)
        .release(true)
        .uniform_buffer_standard_layout(true)
        .relax_block_layout(true)
        .scalar_block_layout(true)
        .build()
        .expect("Failed to build Rust-GPU shaders");

    fs::create_dir_all(dest_path.parent().unwrap()).expect("Failed to create shader output dir");
    fs::copy(result.module.unwrap_single(), dest_path).expect("Failed to copy Rust-GPU shader");
}
