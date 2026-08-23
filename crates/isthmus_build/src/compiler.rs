use crate::artifact::{shader_artifact, shader_dependency, shader_is_fresh, shader_source, workspace_root};
use spirv_builder::{Capability, ModuleResult, SpirvBuilder, SpirvMetadata};
use std::{
    fs,
    path::{Path, PathBuf},
    string::String,
    vec::Vec,
};

const SHADER_TARGET: &str = "spirv-unknown-vulkan1.4";

/// Compiles a Rust-GPU crate to SPIR-V, then updates its artifact.
///
/// Returns whether compilation ran and refreshed the artifact.
///
/// # Errors
///
/// Returns an error if compilation or file access fails.
pub fn build_shader(crate_dir: &Path) -> Result<(PathBuf, bool), String> {
    let output = shader_artifact(crate_dir)?;
    let shader_source = shader_source(crate_dir)?;
    let dependency = shader_dependency(crate_dir)?;
    if shader_is_fresh(&output, &shader_source, dependency.as_deref()) {
        return Ok((output, false));
    }
    let target = workspace_root(crate_dir)?.join("target/isthmus");
    let source = shader_workspace(crate_dir, &target)?;
    let shader = compile_shader(&source, &target.join("build"))?;
    let parent = output.parent().ok_or_else(|| String::from("shader artifact has no parent"))?;
    fs::create_dir_all(parent).map_err(|error| std::format!("failed to create shader artifact directory: {error}"))?;
    fs::write(&output, shader).map_err(|error| std::format!("failed to write shader artifact: {error}"))?;
    Ok((output, true))
}

fn shader_workspace(crate_dir: &Path, target: &Path) -> Result<PathBuf, String> {
    let manifest_text = fs::read_to_string(crate_dir.join("Cargo.toml")).map_err(|error| std::format!("failed to read shader manifest: {error}"))?;
    let manifest = manifest_text
        .parse::<toml_edit::DocumentMut>()
        .map_err(|error| std::format!("invalid shader manifest: {error}"))?;
    let source = shader_source(crate_dir)?;
    if source == crate_dir.join("src/lib.rs") {
        return Ok(crate_dir.to_path_buf());
    }
    let source = fs::canonicalize(&source).map_err(|error| std::format!("failed to locate shader source {}: {error}", source.display()))?;
    let artifact = shader_artifact(crate_dir)?;
    let artifact_name = artifact.file_name().ok_or_else(|| String::from("shader artifact has no file name"))?.to_owned();
    let artifact = fs::canonicalize(artifact.parent().ok_or_else(|| String::from("shader artifact has no parent"))?)
        .map_err(|error| std::format!("failed to locate shader artifact directory: {error}"))?
        .join(artifact_name);
    let dependency = manifest["dependencies"]["isthmus"]["path"]
        .as_str()
        .ok_or_else(|| String::from("shader source requires a path dependency on isthmus"))?;
    let dependency = fs::canonicalize(crate_dir.join(dependency)).map_err(|error| std::format!("failed to locate isthmus dependency: {error}"))?;
    let generated = target.join("source");
    fs::create_dir_all(&generated).map_err(|error| std::format!("failed to create shader workspace: {error}"))?;
    write_if_changed(
        &generated.join("Cargo.toml"),
        &std::format!(
            "[package]\nname = \"isthmus-shader\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[package.metadata.isthmus]\nartifact = {:?}\n\n[lib]\npath = \"lib.rs\"\n\n[workspace]\n\n[dependencies]\nisthmus = {{ path = {:?} }}\n",
            artifact,
            dependency
        ),
    )?;
    write_if_changed(
        &generated.join("lib.rs"),
        &std::format!("#![no_std]\n#![allow(dead_code, unused_imports)]\n#[path = {:?}]\npub mod render;\n", source),
    )?;
    Ok(generated)
}

fn write_if_changed(path: &Path, contents: &str) -> Result<(), String> {
    if fs::read_to_string(path).is_ok_and(|current| current == contents) {
        return Ok(());
    }
    fs::write(path, contents).map_err(|error| std::format!("failed to write {}: {error}", path.display()))
}

fn compile_shader(source: &Path, target: &Path) -> Result<Vec<u8>, String> {
    let build = SpirvBuilder::new(source, SHADER_TARGET)
        .deny_warnings(true)
        .shader_crate_default_features(false)
        .target_dir_path(target)
        .spirv_metadata(SpirvMetadata::None)
        .scalar_block_layout(true)
        .capability(Capability::RuntimeDescriptorArray)
        .capability(Capability::SampledImageArrayNonUniformIndexing)
        .capability(Capability::ShaderNonUniform)
        .extension("SPV_EXT_descriptor_indexing")
        .release(true)
        .build()
        .map_err(|error| std::format!("failed to build Rust-GPU shaders: {error}"))?;
    let module = match build.module {
        ModuleResult::SingleModule(module) => module,
        ModuleResult::MultiModule(_) => {
            return Err(String::from("Rust-GPU unexpectedly produced multiple modules"));
        }
    };
    fs::read(module).map_err(|error| std::format!("failed to read built SPIR-V shader: {error}"))
}
