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

    let mut builder = SpirvBuilder::new(shader_crate, "spirv-unknown-vulkan1.1");
    builder.toolchain_overwrite = Some(rust_toolchain_channel(workspace_dir));

    let result = builder
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

fn rust_toolchain_channel(workspace_dir: &Path) -> String {
    let toolchain = fs::read_to_string(workspace_dir.join("rust-toolchain.toml"))
        .expect("Failed to read rust-toolchain.toml");

    toolchain
        .lines()
        .find_map(|line| {
            let line = line.trim();
            line.strip_prefix("channel")
                .and_then(|line| line.trim_start().strip_prefix('='))
                .map(|line| line.trim().trim_matches('"').to_owned())
        })
        .expect("rust-toolchain.toml must define toolchain.channel")
}
