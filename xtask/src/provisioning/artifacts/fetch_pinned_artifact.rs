use super::compute_artifact_digest;
use crate::provisioning::{
    execution::run_checked_command, paths::resolve_contained_path, types::ArtifactPin,
};
use anyhow::{Result, ensure};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn fetch_pinned_artifact(output: &Path, name: &str, pin: &ArtifactPin) -> Result<PathBuf> {
    let downloads = resolve_contained_path(output, Path::new("downloads"))?;
    std::fs::create_dir_all(&downloads)?;
    let target = resolve_contained_path(&downloads, Path::new(name))?;
    if !target.exists() {
        let temporary = resolve_contained_path(&downloads, Path::new(&format!("{name}.download")))?;
        ensure!(
            !temporary.exists(),
            "incomplete download already exists; inspect/remove the task-owned file before retry"
        );
        run_checked_command(
            Command::new("curl")
                .args([
                    "--fail",
                    "--location",
                    "--proto",
                    "=https",
                    "--proto-redir",
                    "=https",
                    "--tlsv1.2",
                    "--retry",
                    "2",
                    "--max-time",
                    "300",
                    "--max-filesize",
                    "134217728",
                    "--output",
                ])
                .arg(&temporary)
                .arg(&pin.url),
            output,
            &format!("download-{name}"),
        )?;
        ensure!(
            compute_artifact_digest(&temporary)? == pin.sha256,
            "downloaded artifact digest mismatch"
        );
        std::fs::rename(temporary, &target)?;
    }
    ensure!(
        compute_artifact_digest(&target)? == pin.sha256,
        "existing artifact digest mismatch"
    );
    Ok(target)
}
