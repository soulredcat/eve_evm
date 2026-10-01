// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::provisioning::{
    artifacts::{extract_pinned_archive, fetch_pinned_artifact},
    paths::resolve_contained_path,
    receipts::{load_verified_receipt, write_tool_receipt},
    types::{ArtifactPin, ProvisionedArtifact},
};
use anyhow::Result;
use std::path::Path;

pub fn provision_prebuilt(
    output: &Path,
    name: &str,
    pin: &ArtifactPin,
    arguments: &[&str],
) -> Result<ProvisionedArtifact> {
    let recipe = "digest-pinned-prebuilt-v1";
    if let Some(receipt) = load_verified_receipt(output, name, pin, recipe, arguments)? {
        return Ok(receipt);
    }
    let archive = fetch_pinned_artifact(output, &format!("{name}-{}.artifact", pin.sha256), pin)?;
    if pin.archive_root.is_some() {
        extract_pinned_archive(output, &archive, name, pin)?;
    } else {
        let directory = resolve_contained_path(output, Path::new(name))?;
        anyhow::ensure!(
            !directory.exists(),
            "binary directory exists without verified receipt"
        );
        std::fs::create_dir(&directory)?;
        let executable = resolve_contained_path(&directory, Path::new(&pin.executable))?;
        std::fs::copy(archive, &executable)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(executable, std::fs::Permissions::from_mode(0o755))?;
        }
    }
    write_tool_receipt(output, name, pin, recipe, arguments)
}
