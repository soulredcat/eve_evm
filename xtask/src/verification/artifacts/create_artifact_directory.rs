use anyhow::{Context, Result, ensure};
use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub fn create_artifact_directory(root: &Path) -> Result<PathBuf> {
    let root = root.canonicalize()?;
    let local = root.join("local-tests");
    if !local.exists() {
        std::fs::create_dir(&local)?;
    }
    ensure!(
        !std::fs::symlink_metadata(&local)?.file_type().is_symlink(),
        "local-tests cannot be a symlink"
    );
    let local = local.canonicalize()?;
    ensure!(
        local.starts_with(&root),
        "local artifact path escapes repository"
    );
    let status = std::process::Command::new("git")
        .args([
            "check-ignore",
            "--quiet",
            "--",
            "local-tests/verification/probe.json",
        ])
        .current_dir(&root)
        .status()
        .context("Check local artifact ignore policy")?;
    ensure!(status.success(), "local artifacts must be ignored by Git");
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let directory = local.join(format!("verify-{}-{stamp}", std::process::id()));
    std::fs::create_dir(&directory)?;
    Ok(directory)
}
