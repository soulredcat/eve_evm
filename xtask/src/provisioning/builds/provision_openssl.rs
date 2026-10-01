// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::provisioning::{
    artifacts::{extract_pinned_archive, fetch_pinned_artifact},
    execution::run_checked_command,
    receipts::{load_verified_receipt, write_tool_receipt},
    types::{ArtifactPin, ProvisionedArtifact},
};
use anyhow::Result;
use std::{path::Path, process::Command};

pub fn provision_openssl(
    output: &Path,
    pin: &ArtifactPin,
    jobs: usize,
) -> Result<ProvisionedArtifact> {
    let recipe = "Configure-linux-x86_64-gcc-no-shared-no-tests-no-docs-v1";
    if let Some(receipt) = load_verified_receipt(output, "openssl", pin, recipe, &["version"])? {
        return Ok(receipt);
    }
    let archive = fetch_pinned_artifact(output, &format!("openssl-{}.artifact", pin.sha256), pin)?;
    let source = extract_pinned_archive(output, &archive, "openssl", pin)?;
    run_checked_command(
        Command::new("perl")
            .current_dir(&source)
            .args([
                "Configure",
                "linux-x86_64",
                "no-shared",
                "no-tests",
                "no-docs",
            ])
            .env("CC", "gcc"),
        output,
        "openssl-configure",
    )?;
    run_checked_command(
        Command::new("make")
            .current_dir(&source)
            .arg(format!("-j{jobs}")),
        output,
        "openssl-build",
    )?;
    write_tool_receipt(output, "openssl", pin, recipe, &["version"])
}
