use naga::{
    front::spv::{Options, parse_u8_slice},
    valid::{Capabilities, ValidationFlags, Validator},
};
use spirv_builder::{SpirvBuilder, SpirvMetadata};
use std::{fs, path::Path};

fn main() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let shader_path = crate_dir.join("../../assets/cantus.spv");
    let bytes = fs::read(
        SpirvBuilder::new(crate_dir, "spirv-unknown-vulkan1.1")
            .spirv_metadata(SpirvMetadata::None)
            .release(true)
            .uniform_buffer_standard_layout(true)
            .relax_block_layout(true)
            .scalar_block_layout(true)
            .build()
            .expect("failed to build Rust-GPU shaders")
            .module
            .unwrap_single(),
    )
    .expect("failed to read built shader");
    let module =
        parse_u8_slice(&bytes, &Options::default()).expect("naga failed to parse the built shader");
    Validator::new(ValidationFlags::all(), Capabilities::all())
        .validate(&module)
        .expect("naga rejected the built shader");

    if fs::read(&shader_path).is_ok_and(|current| current == bytes) {
        println!("{} is up to date", shader_path.display());
    } else {
        fs::write(&shader_path, bytes).expect("failed to write shader artifact");
        println!("wrote {}", shader_path.display());
    }
}
