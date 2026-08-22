use std::{
    fs,
    path::{Path, PathBuf},
    string::String,
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
