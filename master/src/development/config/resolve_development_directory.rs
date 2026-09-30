use anyhow::{Context, Result, ensure};
use std::path::{Component, Path, PathBuf};

pub fn resolve_development_directory(root: &Path, requested: &Path) -> Result<PathBuf> {
    let root = root.canonicalize().context("repository root must exist")?;
    let relative = if requested.is_absolute() {
        requested.strip_prefix(&root)?
    } else {
        requested
    };
    ensure!(
        relative.starts_with("local-tests") && relative != Path::new("local-tests"),
        "B1 development data requires dedicated ignored local-tests descendant"
    );
    let mut resolved = root.clone();
    for part in relative.components() {
        let Component::Normal(segment) = part else {
            anyhow::bail!("development data path must use ordinary contained components");
        };
        resolved.push(segment);
        if let Ok(metadata) = std::fs::symlink_metadata(&resolved) {
            ensure!(
                !metadata.file_type().is_symlink(),
                "development data path contains a symbolic link"
            );
            ensure!(
                resolved.canonicalize()?.starts_with(&root),
                "development data path escapes repository"
            );
        }
    }
    let ignored = std::process::Command::new("git")
        .current_dir(&root)
        .args(["check-ignore", "--quiet", "--"])
        .arg(resolved.join("_development_publication_guard"))
        .status()?;
    ensure!(
        ignored.success(),
        "development data must remain Git-ignored"
    );
    Ok(resolved)
}
