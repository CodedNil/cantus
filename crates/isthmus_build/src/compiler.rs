use crate::artifact::{
    artifact_is_fresh, shader_artifact, shader_features, workspace_root, write_fingerprint,
};
use naga::{
    ShaderStage, Statement,
    back::wgsl::{WriterFlags, write_string},
    compact::{KeepUnused, compact},
    front::spv::{Options, parse_u8_slice},
    valid::{Capabilities, ValidationFlags, Validator},
};
use spirv_builder::{ModuleResult, SpirvBuilder, SpirvMetadata};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    string::String,
    vec::Vec,
};

macro_rules! ensure {
    ($condition:expr, $($message:tt)*) => {
        if !$condition {
            return Err(std::format!($($message)*));
        }
    };
}

const SHADER_TARGET: &str = "spirv-unknown-vulkan1.1";

/// Compiles and validates a Rust-GPU crate, then updates its artifact.
///
/// Returns whether the artifact changed.
///
/// # Errors
///
/// Returns an error if compilation, validation, or file access fails.
pub fn build_shader(crate_dir: &Path) -> Result<(PathBuf, bool), String> {
    let output = shader_artifact(crate_dir)?;
    if output.is_file() && artifact_is_fresh(crate_dir, &output) {
        return Ok((output, false));
    }
    let target = workspace_root(crate_dir)?.join("target/isthmus");
    let (bytes, sources) = compile_shader(crate_dir, &target)?;
    let shader = translate_shader(&bytes)?;
    let changed = if fs::read_to_string(&output).is_ok_and(|current| current == shader) {
        false
    } else {
        let parent = output
            .parent()
            .ok_or_else(|| String::from("shader artifact has no parent"))?;
        fs::create_dir_all(parent)
            .map_err(|error| std::format!("failed to create shader artifact directory: {error}"))?;
        fs::write(&output, shader)
            .map_err(|error| std::format!("failed to write shader artifact: {error}"))?;
        true
    };
    write_fingerprint(crate_dir, &output, &sources)?;
    Ok((output, changed))
}

fn compile_shader(source: &Path, target: &Path) -> Result<(Vec<u8>, Vec<PathBuf>), String> {
    let features = shader_features(source)?;
    let build = SpirvBuilder::new(source, SHADER_TARGET)
        .deny_warnings(true)
        .shader_crate_default_features(false)
        .shader_crate_features(features)
        .target_dir_path(target)
        .spirv_metadata(SpirvMetadata::Full)
        .release(true)
        .build()
        .map_err(|error| std::format!("failed to build Rust-GPU shaders: {error}"))?;
    let module = match &build.module {
        ModuleResult::SingleModule(module) => module,
        ModuleResult::MultiModule(_) => {
            return Err(String::from("Rust-GPU unexpectedly produced multiple modules"));
        }
    };
    let bytes =
        fs::read(&module).map_err(|error| std::format!("failed to read built shader: {error}"))?;
    Ok((bytes, shader_dependencies(source, &module, target)?))
}

fn shader_dependencies(source: &Path, module: &Path, target: &Path) -> Result<Vec<PathBuf>, String> {
    let module_name = module
        .file_name()
        .ok_or_else(|| String::from("built shader has no file name"))?;
    let dependency_file = target
        .join(SHADER_TARGET)
        .join("release")
        .join(module_name)
        .with_extension("spv.d");
    let contents = fs::read_to_string(dependency_file)
        .map_err(|error| std::format!("failed to read shader dependencies: {error}"))?;
    let contents = contents.replace("\\\n", "");
    let (_, dependencies) = contents
        .lines()
        .next()
        .and_then(|line| line.split_once(": "))
        .ok_or_else(|| String::from("invalid shader dependency file"))?;
    let workspace = workspace_root(source)?;
    let workspace_target = workspace.join("target");
    let mut sources = split_makefile_paths(dependencies)
        .into_iter()
        .filter(|path| path.starts_with(&workspace) && !path.starts_with(&workspace_target))
        .collect::<BTreeSet<_>>();
    for source in ["src/compiler.rs", "src/artifact.rs"] {
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join(source);
        if source.starts_with(&workspace) {
            sources.insert(source);
        }
    }
    for path in sources.clone() {
        if let Some(manifest) = path
            .ancestors()
            .take_while(|directory| directory.starts_with(&workspace))
            .map(|directory| directory.join("Cargo.toml"))
            .find(|manifest| manifest.is_file())
        {
            sources.insert(manifest);
        }
    }
    sources.extend([workspace.join("Cargo.toml"), workspace.join("Cargo.lock")]);
    Ok(sources.into_iter().filter(|path| path.is_file()).collect())
}

fn split_makefile_paths(input: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let mut path = String::new();
    let mut escaped = false;
    for character in input.chars() {
        if escaped {
            path.push(character);
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character.is_whitespace() {
            if !path.is_empty() {
                paths.push(PathBuf::from(core::mem::take(&mut path)));
            }
        } else {
            path.push(character);
        }
    }
    if !path.is_empty() {
        paths.push(PathBuf::from(path));
    }
    paths
}

fn translate_shader(bytes: &[u8]) -> Result<String, String> {
    let mut module = parse_u8_slice(
        bytes,
        &Options {
            adjust_coordinate_space: false,
            strict_capabilities: true,
            block_ctx_dump_prefix: None,
        },
    )
    .map_err(|error| std::format!("WGPU cannot parse shader: {error}"))?;
    let mut passes = BTreeMap::<String, u8>::new();
    let mut entry_names = BTreeSet::new();
    let mut implementation_names = Vec::new();
    for entry in &mut module.entry_points {
        let (name, stage, suffix) = if let Some(name) = entry.name.strip_suffix("::vertex") {
            ensure!(
                entry.stage == ShaderStage::Vertex,
                "{name}: vertex entry point has the wrong shader stage"
            );
            (name, 1, "vertex")
        } else if let Some(name) = entry.name.strip_suffix("::fragment") {
            ensure!(
                entry.stage == ShaderStage::Fragment,
                "{name}: fragment entry point has the wrong shader stage"
            );
            (name, 2, "fragment")
        } else {
            return Err(std::format!("unexpected shader entry point: {}", entry.name));
        };
        let pass_name = String::from(name);
        let portable_name = std::format!("{}_{}", name.replace("::", "_"), suffix);
        ensure!(
            entry_names.insert(portable_name.clone()),
            "shader pass names collide after conversion to WGSL: {pass_name}"
        );
        entry.name = portable_name;
        if let Some(function) = entry.function.body.iter().find_map(|statement| match statement {
            Statement::Call { function, .. } => Some(*function),
            _ => None,
        }) {
            implementation_names.push((function, std::format!("{}_impl", entry.name)));
        }
        let stages = passes.entry(pass_name.clone()).or_default();
        ensure!(*stages & stage == 0, "{pass_name}: duplicate shader stage");
        *stages |= stage;
    }
    for (function, name) in implementation_names {
        module.functions[function].name = Some(name);
    }
    ensure!(!passes.is_empty(), "shader contains no render passes");
    for (name, stages) in passes {
        ensure!(stages == 3, "{name}: missing vertex or fragment entry point");
    }
    Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&module)
        .map_err(|error| std::format!("WGPU cannot validate shader before compaction: {error}"))?;
    compact(&mut module, KeepUnused::No);
    let info = Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&module)
        .map_err(|error| std::format!("WGPU cannot validate shader: {error}"))?;
    let shader = write_string(&module, &info, WriterFlags::empty())
        .map_err(|error| std::format!("WGPU cannot translate shader to WGSL: {error}"))?;
    let emitted = naga::front::wgsl::parse_str(&shader)
        .map_err(|error| std::format!("WGPU cannot parse generated WGSL: {error}"))?;
    Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&emitted)
        .map_err(|error| std::format!("WGPU cannot validate generated WGSL: {error}"))?;
    Ok(shader.lines().map(str::trim_end).collect::<Vec<_>>().join("\n") + "\n")
}
