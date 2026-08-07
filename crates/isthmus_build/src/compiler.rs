use crate::artifact::{shader_artifact, shader_features, workspace_root};
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
    let target = workspace_root(crate_dir)?.join("target/isthmus");
    let shader = translate_shader(&compile_shader(crate_dir, &target)?)?;
    if fs::read_to_string(&output).is_ok_and(|current| current == shader) {
        return Ok((output, false));
    }
    let parent = output
        .parent()
        .ok_or_else(|| String::from("shader artifact has no parent"))?;
    fs::create_dir_all(parent)
        .map_err(|error| std::format!("failed to create shader artifact directory: {error}"))?;
    fs::write(&output, shader)
        .map_err(|error| std::format!("failed to write shader artifact: {error}"))?;
    Ok((output, true))
}

fn compile_shader(source: &Path, target: &Path) -> Result<Vec<u8>, String> {
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
    fs::read(&module).map_err(|error| std::format!("failed to read built shader: {error}"))
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
    let shader = localize_phi_names(&shader);
    let emitted = naga::front::wgsl::parse_str(&shader)
        .map_err(|error| std::format!("WGPU cannot parse generated WGSL: {error}"))?;
    Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&emitted)
        .map_err(|error| std::format!("WGPU cannot validate generated WGSL: {error}"))?;
    Ok(shader.lines().map(str::trim_end).collect::<Vec<_>>().join("\n") + "\n")
}

/// Renumbers Naga's phi temporaries per function.
///
/// Naga draws phi names from a module-wide counter, so adding one expression
/// renumbers every phi after it and rewrites most of the checked-in artifact.
/// Numbering them within each function keeps an edit's diff local to the
/// function that actually changed.
fn localize_phi_names(shader: &str) -> String {
    let mut output = String::with_capacity(shader.len());
    let mut names = BTreeMap::<&str, usize>::new();
    let mut depth = 0usize;
    let mut rest = shader;
    loop {
        let (head, tail) = rest.find("phi_").map_or((rest, ""), |at| rest.split_at(at));
        for character in head.chars() {
            match character {
                '{' => depth += 1,
                '}' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        names.clear();
                    }
                }
                _ => {}
            }
        }
        output.push_str(head);
        if tail.is_empty() {
            return output;
        }
        let digits = tail[4..].bytes().take_while(u8::is_ascii_digit).count();
        if digits > 0 && tail.as_bytes().get(4 + digits) == Some(&b'_') {
            let next = names.len();
            let name = &tail[..=4 + digits];
            output.push_str(&std::format!("phi_{}_", *names.entry(name).or_insert(next)));
            rest = &tail[name.len()..];
        } else {
            output.push_str("phi_");
            rest = &tail[4..];
        }
    }
}
