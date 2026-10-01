// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::build_tool_environment;
use crate::provisioning::{
    artifacts::compute_artifact_digest, clients::compute_client_tree_digest,
    execution::verify_tool_version, paths::resolve_contained_path, pins::load_tool_pins,
    receipts::validate_receipt_record, types::ProvisionedTools,
};
use anyhow::{Result, ensure};
use std::path::Path;

/// Validate source/local bytes before a bulk gate consumes environment paths.
pub fn validate_provisioned_tools(
    root: &Path,
    pins_path: &Path,
    receipt_path: &Path,
) -> Result<ProvisionedTools> {
    let root = root.canonicalize()?;
    let relative = if receipt_path.is_absolute() {
        receipt_path.strip_prefix(&root)?
    } else {
        receipt_path
    };
    ensure!(
        relative.starts_with("local-tests"),
        "receipt must remain in ignored task storage"
    );
    let receipt_path = resolve_contained_path(&root, relative)?;
    ensure!(
        std::fs::metadata(&receipt_path)?.len() <= 128 * 1024,
        "provisioned receipt exceeds limit"
    );
    let output = receipt_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("receipt has no parent"))?;
    let pins = load_tool_pins(&root, pins_path)?;
    let report: ProvisionedTools = serde_json::from_slice(&std::fs::read(&receipt_path)?)?;
    ensure!(
        report.version == 1 && report.platform == pins.platform && report.artifacts.len() == 5,
        "unsupported/incomplete provisioning report"
    );
    ensure!(
        report.pin_file_sha256
            == compute_artifact_digest(&resolve_contained_path(&root, pins_path)?)?
            && report.client_lock_sha256 == pins.clients.lock_sha256,
        "provision report source pins changed"
    );
    let comet_recipe = format!(
        "go1.27.1-readonly-trimpath-no-host-vcs-CGO1-clang19-v2;{}",
        pins.comet.source_identity
    );
    for (name, pin, recipe, args) in [
        ("go", &pins.go, "digest-pinned-prebuilt-v1", vec!["version"]),
        ("comet", &pins.comet, comet_recipe.as_str(), vec!["version"]),
        (
            "openssl",
            &pins.openssl,
            "Configure-linux-x86_64-gcc-no-shared-no-tests-no-docs-v1",
            vec!["version"],
        ),
        (
            "solidity",
            &pins.solidity,
            "digest-pinned-prebuilt-v1",
            vec!["--version"],
        ),
        (
            "node",
            &pins.node,
            "digest-pinned-prebuilt-v1",
            vec!["--version"],
        ),
    ] {
        let artifact = report
            .artifacts
            .iter()
            .find(|artifact| artifact.name == name)
            .ok_or_else(|| anyhow::anyhow!("required artifact missing: {name}"))?;
        let archive = resolve_contained_path(
            output,
            &Path::new("downloads").join(format!("{name}-{}.artifact", pin.sha256)),
        )?;
        ensure!(
            compute_artifact_digest(&archive)? == pin.sha256,
            "cached source/archive bytes changed for {name}"
        );
        validate_receipt_record(output, name, pin, recipe, artifact)?;
        verify_tool_version(&artifact.executable, &args, &pin.expected_version)?;
    }
    let modules = resolve_contained_path(output, Path::new("clients/node_modules"))?;
    ensure!(
        compute_client_tree_digest(&modules)? == report.client_tree_sha256,
        "client module bytes changed after provisioning"
    );
    ensure!(
        compute_artifact_digest(&output.join("clients/package-lock.json"))?
            == pins.clients.lock_sha256,
        "local client lockfile changed"
    );
    ensure!(
        report.environment == build_tool_environment(&root, output, &report.artifacts)?,
        "environment does not match checked paths"
    );
    Ok(report)
}
