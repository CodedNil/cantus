use crate::artifact::{shader_artifact, shader_features, workspace_root};
use spirv_builder::{ModuleResult, SpirvBuilder, SpirvMetadata};
use std::{
    fs,
    path::{Path, PathBuf},
    string::String,
    vec::Vec,
};

const SHADER_TARGET: &str = "spirv-unknown-naga-wgsl";

/// Compiles a Rust-GPU crate directly to WGSL, then updates its artifact.
///
/// Returns whether the artifact changed.
///
/// # Errors
///
/// Returns an error if compilation or file access fails.
pub fn build_shader(crate_dir: &Path) -> Result<(PathBuf, bool), String> {
    let output = shader_artifact(crate_dir)?;
    let target = workspace_root(crate_dir)?.join("target/isthmus");
    let shader = compile_shader(crate_dir, &target)?;
    if fs::read_to_string(&output).is_ok_and(|current| current == shader) {
        return Ok((output, false));
    }
    let parent = output.parent().ok_or_else(|| String::from("shader artifact has no parent"))?;
    fs::create_dir_all(parent).map_err(|error| std::format!("failed to create shader artifact directory: {error}"))?;
    fs::write(&output, shader).map_err(|error| std::format!("failed to write shader artifact: {error}"))?;
    Ok((output, true))
}

fn compile_shader(source: &Path, target: &Path) -> Result<String, String> {
    let build = SpirvBuilder::new(source, SHADER_TARGET)
        .deny_warnings(true)
        .shader_crate_default_features(false)
        .shader_crate_features(shader_features(source)?)
        .target_dir_path(target)
        .spirv_metadata(SpirvMetadata::Full)
        .release(true)
        .build()
        .map_err(|error| std::format!("failed to build Rust-GPU shaders: {error}"))?;
    let module = match build.module {
        ModuleResult::SingleModule(module) => module,
        ModuleResult::MultiModule(_) => {
            return Err(String::from("Rust-GPU unexpectedly produced multiple modules"));
        }
    };
    let shader = fs::read_to_string(module.with_extension("wgsl")).map_err(|error| std::format!("failed to read built WGSL shader: {error}"))?;
    Ok(shader.lines().map(str::trim_end).collect::<Vec<_>>().join("\n") + "\n")
}
