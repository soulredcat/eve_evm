use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, ensure};

pub fn validate_local_artifact_directory(requested: &Path, repository: &Path) -> Result<PathBuf> {
    let expected = repository
        .join("local-tests/consensus-b0")
        .canonicalize()
        .context("create ignored local-tests/consensus-b0 before running the API fixture")?;
    let requested = requested
        .canonicalize()
        .context("API artifact directory must already exist")?;
    ensure!(
        requested.starts_with(expected),
        "API artifacts must remain beneath repository local-tests/consensus-b0"
    );
    let ignored = Command::new("git")
        .current_dir(repository)
        .args(["check-ignore", "--quiet", "--"])
        .arg(requested.join("_fixture_publication_guard"))
        .status()
        .context("verify API artifacts are ignored")?;
    ensure!(
        ignored.success(),
        "API artifact location is not Git-ignored"
    );
    Ok(requested)
}
