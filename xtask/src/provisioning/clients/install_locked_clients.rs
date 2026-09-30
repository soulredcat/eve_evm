use crate::provisioning::{
    artifacts::compute_artifact_digest,
    execution::run_checked_command,
    paths::resolve_contained_path,
    types::{ClientPins, ProvisionedArtifact},
};
use anyhow::{Result, ensure};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn install_locked_clients(
    root: &Path,
    output: &Path,
    node: &ProvisionedArtifact,
    pins: &ClientPins,
) -> Result<PathBuf> {
    let source =
        resolve_contained_path(root, Path::new("xtask/tests/provisioning_client_fixture"))?;
    ensure!(
        compute_artifact_digest(&source.join("package.json"))? == pins.manifest_sha256
            && compute_artifact_digest(&source.join("package-lock.json"))? == pins.lock_sha256,
        "client fixture source/lock digest mismatch"
    );
    let directory = resolve_contained_path(output, Path::new("clients"))?;
    std::fs::create_dir_all(&directory)?;
    for file in ["package.json", "package-lock.json"] {
        let target = resolve_contained_path(&directory, Path::new(file))?;
        if target.exists() {
            ensure!(
                compute_artifact_digest(&target)? == compute_artifact_digest(&source.join(file))?,
                "existing client metadata differs from pinned fixture"
            );
        } else {
            std::fs::copy(source.join(file), target)?;
        }
    }
    let modules = resolve_contained_path(&directory, Path::new("node_modules"))?;
    if modules.exists() {
        ensure!(
            modules.is_dir(),
            "client module root must be an ordinary directory"
        );
    }
    let cache = resolve_contained_path(output, Path::new("npm-cache"))?;
    std::fs::create_dir_all(&cache)?;
    let npm = resolve_contained_path(
        output,
        Path::new("node/lib/node_modules/npm/bin/npm-cli.js"),
    )?;
    run_checked_command(
        Command::new(&node.executable)
            .arg(npm)
            .current_dir(&directory)
            .args([
                "ci",
                "--ignore-scripts",
                "--no-audit",
                "--no-fund",
                "--cache",
            ])
            .arg(cache)
            .env("npm_config_update_notifier", "false"),
        output,
        "npm-client-install",
    )?;
    ensure!(
        compute_artifact_digest(&directory.join("package-lock.json"))? == pins.lock_sha256,
        "npm changed frozen lockfile"
    );
    for (name, expected) in [("typescript", &pins.typescript), ("viem", &pins.viem)] {
        let package = resolve_contained_path(
            &directory,
            &Path::new("node_modules").join(name).join("package.json"),
        )?;
        let metadata: serde_json::Value = serde_json::from_slice(&std::fs::read(package)?)?;
        ensure!(
            metadata["version"].as_str() == Some(expected.as_str()),
            "installed client version differs from pin"
        );
    }
    Ok(directory)
}
