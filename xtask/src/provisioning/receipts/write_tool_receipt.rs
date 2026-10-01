// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::provisioning::{
    artifacts::compute_artifact_digest,
    execution::verify_tool_version,
    paths::resolve_contained_path,
    types::{ArtifactPin, ProvisionedArtifact},
};
use anyhow::{Result, ensure};
use std::{io::Write, path::Path};

pub fn write_tool_receipt(
    output: &Path,
    name: &str,
    pin: &ArtifactPin,
    recipe: &str,
    arguments: &[&str],
) -> Result<ProvisionedArtifact> {
    let executable = resolve_contained_path(output, &Path::new(name).join(&pin.executable))?;
    let digest = compute_artifact_digest(&executable)?;
    if let Some(expected) = &pin.binary_sha256 {
        ensure!(digest == *expected, "prebuilt executable digest mismatch");
    }
    let receipt = ProvisionedArtifact {
        name: name.to_owned(),
        executable: executable.clone(),
        source_sha256: pin.sha256.clone(),
        executable_sha256: digest,
        version_output: verify_tool_version(&executable, arguments, &pin.expected_version)?,
        recipe_identity: recipe.to_owned(),
    };
    let path = resolve_contained_path(output, Path::new(&format!("{name}.receipt.json")))?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    file.write_all(&serde_json::to_vec_pretty(&receipt)?)?;
    file.sync_all()?;
    Ok(receipt)
}
