#[cfg(feature = "compiler")]
use std::vec::Vec;
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

#[cfg(feature = "compiler")]
pub(crate) fn shader_features(crate_dir: &Path) -> Result<Vec<String>, String> {
    let manifest = read_manifest(crate_dir)?;
    let Some(features) = manifest_path(&manifest, &["package", "metadata", "isthmus", "features"]) else {
        return Ok(vec![String::from("shader")]);
    };
    features
        .as_array()
        .ok_or_else(|| String::from("package.metadata.isthmus.features must be an array"))?
        .iter()
        .map(|feature| feature.as_str().map(String::from).ok_or_else(|| String::from("Isthmus shader features must be strings")))
        .collect()
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
