use std::{
    fs,
    path::{Component, Path, PathBuf},
    vec::Vec,
};

/// Resolves and verifies a package's checked-in shader.
///
/// # Errors
/// Returns an error when metadata is invalid or the artifact is stale.
pub fn checked_shader_artifact(crate_dir: &Path) -> Result<PathBuf, String> {
    let artifact = try_shader_artifact(crate_dir)?;
    if artifact.is_file() && artifact_is_fresh(crate_dir, &artifact) {
        Ok(artifact)
    } else {
        Err(String::from(
            "shader artifact is stale; run `isthmus build` to regenerate it",
        ))
    }
}

/// Finds the shader package at or below `start`.
///
/// # Errors
/// Returns an error when the workspace is unreadable or has no unique shader package.
pub fn find_shader_crate(start: &Path) -> Result<PathBuf, String> {
    let start = fs::canonicalize(start)
        .map_err(|error| std::format!("failed to locate shader workspace: {error}"))?;
    if has_shader_metadata(&start.join("Cargo.toml")) {
        return Ok(start);
    }
    let mut packages = Vec::new();
    collect_shader_packages(&start, &mut packages)?;
    match packages.as_slice() {
        [package] => Ok(package.clone()),
        [] => Err(String::from("no package with `[package.metadata.isthmus]` found")),
        _ => Err(String::from(
            "multiple shader packages found; pass a package path",
        )),
    }
}

#[cfg(feature = "compiler")]
pub(crate) fn shader_artifact(crate_dir: &Path) -> Result<PathBuf, String> {
    try_shader_artifact(crate_dir)
}

#[cfg(feature = "compiler")]
pub(crate) fn shader_features(crate_dir: &Path) -> Result<Vec<String>, String> {
    let manifest = read_manifest(crate_dir)?;
    let Some(features) = manifest_path(&manifest, &["package", "metadata", "isthmus", "features"])
    else {
        return Ok(vec![String::from("shader")]);
    };
    features
        .as_array()
        .ok_or_else(|| String::from("package.metadata.isthmus.features must be an array"))?
        .iter()
        .map(|feature| {
            feature
                .as_str()
                .map(String::from)
                .ok_or_else(|| String::from("Isthmus shader features must be strings"))
        })
        .collect()
}

fn try_shader_artifact(crate_dir: &Path) -> Result<PathBuf, String> {
    let manifest = read_manifest(crate_dir)?;
    let relative = manifest_path(&manifest, &["package", "metadata", "isthmus", "artifact"])
        .and_then(toml_edit::Item::as_str)
        .ok_or_else(|| String::from("missing `package.metadata.isthmus.artifact`"))?;
    Ok(crate_dir.join(relative))
}

fn read_manifest(crate_dir: &Path) -> Result<toml_edit::DocumentMut, String> {
    fs::read_to_string(crate_dir.join("Cargo.toml"))
        .map_err(|error| std::format!("failed to read shader package manifest: {error}"))?
        .parse()
        .map_err(|error| std::format!("invalid shader package manifest: {error}"))
}

fn manifest_path<'a>(
    manifest: &'a toml_edit::DocumentMut,
    path: &[&str],
) -> Option<&'a toml_edit::Item> {
    path.iter()
        .try_fold(manifest.as_item(), |item, key| item.get(*key))
}

const FINGERPRINT_VERSION: &str = concat!("isthmus-v3-", env!("CARGO_PKG_VERSION"));
const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x100_0000_01b3;

pub fn artifact_is_fresh(crate_dir: &Path, artifact: &Path) -> bool {
    let Ok(stored) = fs::read_to_string(fingerprint_path(artifact)) else {
        return false;
    };
    let mut lines = stored.lines();
    if lines.next() != Some(FINGERPRINT_VERSION) {
        return false;
    }
    let Some(digest) = lines.next() else {
        return false;
    };
    let Ok(root) = workspace_root(crate_dir) else {
        return false;
    };
    let sources = lines
        .map(Path::new)
        .map(|source| {
            if source.is_absolute()
                || source
                    .components()
                    .any(|component| component == Component::ParentDir)
            {
                return None;
            }
            Some(root.join(source))
        })
        .collect::<Option<Vec<_>>>();
    let Some(sources) = sources else {
        return false;
    };
    artifact_digest(&root, artifact, &sources).as_deref() == Some(digest)
}

#[cfg(feature = "compiler")]
pub(crate) fn write_fingerprint(
    crate_dir: &Path,
    artifact: &Path,
    sources: &[PathBuf],
) -> Result<(), String> {
    let root = workspace_root(crate_dir)?;
    let mut sources = sources
        .iter()
        .map(|source| {
            source
                .strip_prefix(&root)
                .map_err(|_| {
                    std::format!("shader dependency is outside the workspace: {}", source.display())
                })
                .map(Path::to_path_buf)
        })
        .collect::<Result<Vec<_>, _>>()?;
    sources.sort();
    sources.dedup();
    let absolute = sources.iter().map(|source| root.join(source)).collect::<Vec<_>>();
    let digest = artifact_digest(&root, artifact, &absolute)
        .ok_or_else(|| String::from("failed to fingerprint shader sources"))?;
    let mut contents = std::format!("{FINGERPRINT_VERSION}\n{digest}\n");
    for source in sources {
        contents.push_str(&source.to_string_lossy());
        contents.push('\n');
    }
    fs::write(fingerprint_path(artifact), contents)
        .map_err(|error| std::format!("failed to write shader fingerprint: {error}"))
}

fn artifact_digest(root: &Path, artifact: &Path, sources: &[PathBuf]) -> Option<String> {
    let mut sources = sources.to_vec();
    sources.sort();
    sources.dedup();
    let source_hash = sources.into_iter().try_fold(OFFSET, |hash, path| {
        let relative = path.strip_prefix(root).ok()?;
        let hash = hash_bytes(hash, relative.to_string_lossy().as_bytes());
        Some(hash_bytes(hash, &fs::read(path).ok()?))
    })?;
    Some(std::format!(
        "{} {}",
        format_fingerprint(source_hash),
        format_fingerprint(hash_bytes(OFFSET, &fs::read(artifact).ok()?)),
    ))
}

pub fn workspace_root(crate_dir: &Path) -> Result<PathBuf, String> {
    let crate_dir = fs::canonicalize(crate_dir)
        .map_err(|error| std::format!("failed to locate shader crate: {error}"))?;
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

fn has_shader_metadata(manifest: &Path) -> bool {
    fs::read_to_string(manifest)
        .ok()
        .and_then(|manifest| manifest.parse::<toml_edit::DocumentMut>().ok())
        .is_some_and(|manifest| {
            manifest_path(&manifest, &["package", "metadata", "isthmus", "artifact"])
                .is_some_and(toml_edit::Item::is_str)
        })
}

fn collect_shader_packages(directory: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
    if has_shader_metadata(&directory.join("Cargo.toml")) {
        output.push(directory.to_path_buf());
    }
    let entries = fs::read_dir(directory)
        .map_err(|error| std::format!("failed to scan {}: {error}", directory.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if entry.file_type().is_ok_and(|kind| kind.is_dir())
            && !name.starts_with('.')
            && !matches!(name, "assets" | "target")
        {
            collect_shader_packages(&path, output)?;
        }
    }
    Ok(())
}

fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for &byte in bytes {
        hash = (hash ^ u64::from(byte)).wrapping_mul(PRIME);
    }
    hash
}

fn fingerprint_path(artifact: &Path) -> PathBuf {
    let mut path = artifact.as_os_str().to_os_string();
    path.push(".fingerprint");
    PathBuf::from(path)
}

fn format_fingerprint(fingerprint: u64) -> String {
    std::format!("{fingerprint:016x}")
}
