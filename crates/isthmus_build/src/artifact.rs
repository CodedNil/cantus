use std::{
    fs,
    path::{Path, PathBuf},
    string::String,
    time::SystemTime,
};

/// Resolves a package's checked-in shader from its manifest metadata.
///
/// # Errors
/// Returns an error when the manifest is unreadable or declares no artifact.
pub fn shader_artifact(crate_dir: &Path) -> Result<PathBuf, String> {
    let manifest = read_manifest(crate_dir)?;
    let relative = manifest_path(&manifest, &["package", "metadata", "isthmus", "artifact"])
        .and_then(toml_edit::Item::as_str)
        .ok_or_else(|| String::from("missing `package.metadata.isthmus.artifact`"))?;
    Ok(crate_dir.join(relative))
}

/// Resolves the Rust module that owns a package's paint declarations.
///
/// # Errors
/// Returns an error when the manifest is unreadable.
pub fn shader_source(crate_dir: &Path) -> Result<PathBuf, String> {
    let manifest = read_manifest(crate_dir)?;
    Ok(manifest_path(&manifest, &["package", "metadata", "isthmus", "source"])
        .and_then(toml_edit::Item::as_str)
        .map_or_else(|| crate_dir.join("src/lib.rs"), |source| crate_dir.join(source)))
}

/// Resolves the local Isthmus dependency whose ABI and macros affect shader output.
///
/// # Errors
/// Returns an error when the manifest is unreadable.
pub fn shader_dependency(crate_dir: &Path) -> Result<Option<PathBuf>, String> {
    let manifest = read_manifest(crate_dir)?;
    Ok(manifest_path(&manifest, &["dependencies", "isthmus", "path"])
        .and_then(toml_edit::Item::as_str)
        .map(|dependency| crate_dir.join(dependency)))
}

/// Returns whether an artifact is newer than every Rust source that can affect it.
pub fn shader_is_fresh(artifact: &Path, source: &Path, dependency: Option<&Path>) -> bool {
    let Ok(built) = fs::metadata(artifact).and_then(|metadata| metadata.modified()) else {
        return false;
    };
    let shader = newest_rust_source(source.parent().unwrap_or(source));
    let engine = dependency.and_then(|dependency| newest_rust_source(&dependency.join("src")));
    let crates = dependency.and_then(Path::parent);
    let macros = crates.and_then(|crates| newest_rust_source(&crates.join("isthmus_macros/src")));
    let compiler = crates.and_then(|crates| newest_rust_source(&crates.join("isthmus_build/src")));
    shader.into_iter().chain(engine).chain(macros).chain(compiler).all(|modified| modified <= built)
}

fn newest_rust_source(path: &Path) -> Option<SystemTime> {
    fs::read_dir(path)
        .ok()?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                newest_rust_source(&path)
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                entry.metadata().ok()?.modified().ok()
            } else {
                None
            }
        })
        .max()
}

fn read_manifest(crate_dir: &Path) -> Result<toml_edit::DocumentMut, String> {
    fs::read_to_string(crate_dir.join("Cargo.toml"))
        .map_err(|error| std::format!("failed to read shader package manifest: {error}"))?
        .parse()
        .map_err(|error| std::format!("invalid shader package manifest: {error}"))
}

fn manifest_path<'a>(manifest: &'a toml_edit::DocumentMut, path: &[&str]) -> Option<&'a toml_edit::Item> {
    path.iter().try_fold(manifest.as_item(), |item, key| item.get(*key))
}

#[cfg(feature = "compiler")]
pub(crate) fn workspace_root(crate_dir: &Path) -> Result<PathBuf, String> {
    let crate_dir = fs::canonicalize(crate_dir).map_err(|error| std::format!("failed to locate shader crate: {error}"))?;
    Ok(crate_dir
        .ancestors()
        .find(|path| {
            fs::read_to_string(path.join("Cargo.toml"))
                .ok()
                .and_then(|manifest| manifest.parse::<toml_edit::DocumentMut>().ok())
                .is_some_and(|manifest| manifest.get("workspace").is_some())
        })
        .unwrap_or(&crate_dir)
        .to_path_buf())
}
