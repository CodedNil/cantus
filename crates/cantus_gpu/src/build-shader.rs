use spirv_builder::{SpirvBuilder, SpirvMetadata};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("cantus_gpu must live two levels below the workspace root");
    let shader_path = workspace_dir.join("assets/cantus.spv");

    let result = SpirvBuilder::new(&manifest_dir, "spirv-unknown-vulkan1.1")
        .spirv_metadata(SpirvMetadata::None)
        .release(true)
        .uniform_buffer_standard_layout(true)
        .relax_block_layout(true)
        .scalar_block_layout(true)
        .build()
        .expect("failed to build Rust-GPU shaders");

    fs::create_dir_all(shader_path.parent().unwrap()).expect("failed to create shader directory");
    fs::copy(result.module.unwrap_single(), &shader_path).expect("failed to write shader artifact");

    println!("wrote {}", shader_path.display());
}
