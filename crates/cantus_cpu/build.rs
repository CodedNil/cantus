use spirv_builder::{SpirvBuilder, SpirvMetadata};
use std::{env, fs, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = format!("{manifest_dir}/../..");
    let shader_crate = format!("{workspace_dir}/crates/cantus_gpu");
    let shared_crate = format!("{workspace_dir}/crates/cantus_shared");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = format!("{out_dir}/cantus.spv");

    println!("cargo:rerun-if-changed={shader_crate}");
    println!("cargo:rerun-if-changed={shared_crate}");
    println!("cargo:rerun-if-changed={workspace_dir}/rust-toolchain.toml");

    let mut builder = SpirvBuilder::new(shader_crate, "spirv-unknown-vulkan1.1");
    builder.toolchain_overwrite = Some(rust_toolchain_channel(&workspace_dir));

    let result = builder
        .spirv_metadata(SpirvMetadata::None)
        .release(true)
        .uniform_buffer_standard_layout(true)
        .relax_block_layout(true)
        .scalar_block_layout(true)
        .build()
        .expect("Failed to build Rust-GPU shaders");

    fs::create_dir_all(Path::new(&dest_path).parent().unwrap())
        .expect("Failed to create shader output directory");
    fs::copy(result.module.unwrap_single(), dest_path).expect("Failed to copy Rust-GPU shader");
}

fn rust_toolchain_channel(workspace_dir: &str) -> String {
    let toolchain = fs::read_to_string(format!("{workspace_dir}/rust-toolchain.toml"))
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
