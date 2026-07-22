use naga::{
    front::spv::{Options, parse_u8_slice},
    valid::{Capabilities, ValidationFlags, Validator},
};
use spirv_builder::{SpirvBuilder, SpirvMetadata};
use std::{fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let shader_path = manifest_dir.join("../../assets/cantus.spv");
    let result = SpirvBuilder::new(&manifest_dir, "spirv-unknown-vulkan1.1")
        .spirv_metadata(SpirvMetadata::None)
        .release(true)
        .uniform_buffer_standard_layout(true)
        .relax_block_layout(true)
        .scalar_block_layout(true)
        .build()
        .expect("failed to build Rust-GPU shaders");
    let spirv_path = result.module.unwrap_single();

    let bytes = fs::read(spirv_path).expect("failed to read built shader");
    let module =
        parse_u8_slice(&bytes, &Options::default()).expect("naga failed to parse the built shader");
    Validator::new(ValidationFlags::all(), Capabilities::all())
        .validate(&module)
        .expect("naga rejected the built shader");

    fs::copy(spirv_path, &shader_path).expect("failed to write shader artifact");
    println!("wrote {}", shader_path.display());
}
