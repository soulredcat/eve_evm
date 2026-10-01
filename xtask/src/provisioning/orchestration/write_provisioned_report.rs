// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::provisioning::{
    artifacts::compute_artifact_digest, paths::resolve_contained_path, types::ProvisionedTools,
};
use anyhow::Result;
use std::path::Path;

pub fn write_provisioned_report(
    pin_path: &Path,
    output: &Path,
    mut report: ProvisionedTools,
) -> Result<ProvisionedTools> {
    report.pin_file_sha256 = compute_artifact_digest(pin_path)?;
    let receipt = resolve_contained_path(output, Path::new("provisioned-tools.json"))?;
    std::fs::write(receipt, serde_json::to_vec_pretty(&report)?)?;
    Ok(report)
}
