use super::{compute_artifact_digest, validate_archive_links, validate_archive_members};
use crate::provisioning::{
    execution::run_checked_command, paths::resolve_contained_path, types::ArtifactPin,
};
use anyhow::{Result, ensure};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn extract_pinned_archive(
    output: &Path,
    archive: &Path,
    name: &str,
    pin: &ArtifactPin,
) -> Result<PathBuf> {
    ensure!(
        compute_artifact_digest(archive)? == pin.sha256,
        "archive changed before extraction"
    );
    let root = pin
        .archive_root
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("archive root is required"))?;
    let listing = Command::new("tar")
        .args(["--list", "--gzip", "--file"])
        .arg(archive)
        .output()?;
    ensure!(listing.status.success(), "cannot inspect pinned archive");
    validate_archive_members(&String::from_utf8(listing.stdout)?, root)?;
    let metadata = Command::new("tar")
        .args([
            "--list",
            "--gzip",
            "--verbose",
            "--full-time",
            "--numeric-owner",
            "--file",
        ])
        .arg(archive)
        .output()?;
    ensure!(
        metadata.status.success(),
        "cannot inspect pinned archive metadata"
    );
    validate_archive_links(&String::from_utf8(metadata.stdout)?, root)?;
    let destination = resolve_contained_path(output, Path::new(name))?;
    ensure!(
        !destination.exists(),
        "source/tool directory exists without a reusable verified receipt"
    );
    std::fs::create_dir(&destination)?;
    run_checked_command(
        Command::new("tar")
            .args([
                "--extract",
                "--gzip",
                "--no-same-owner",
                "--no-same-permissions",
                "--strip-components=1",
                "--file",
            ])
            .arg(archive)
            .arg("--directory")
            .arg(&destination),
        output,
        &format!("extract-{name}"),
    )?;
    Ok(destination)
}
