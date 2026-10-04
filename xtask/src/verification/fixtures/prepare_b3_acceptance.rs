// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    provisioning::compute_artifact_digest,
    verification::{
        commands::require_command_success::require_command_success,
        types::report_types::VerificationReport,
    },
};
use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};

/// Build and identify the explicit disposable-consensus acceptance configuration.
pub fn prepare_b3_acceptance(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
) -> Result<()> {
    ensure!(
        cfg!(target_os = "linux"),
        "B3 requires the Linux reference process/Unix transport platform"
    );
    require_command_success(
        root,
        artifacts,
        report,
        env!("CARGO"),
        &["build", "--locked", "-p", "eve-validator"],
    )?;
    require_command_success(
        root,
        artifacts,
        report,
        env!("CARGO"),
        &[
            "build",
            "--release",
            "--locked",
            "-p",
            "eve-public",
            "-p",
            "eve-master",
        ],
    )?;
    let configured = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target"));
    let target = if configured.is_absolute() {
        configured
    } else {
        root.join(configured)
    };
    let validator = target.join("debug/eve-validator");
    let public_follower = target.join("release/eve-public");
    let master_follower = target.join("release/eve-master");
    ensure!(
        public_follower.is_file(),
        "actual public follower executable missing"
    );
    report.tool_environment.insert(
        "EVE_PUBLIC_BINARY".into(),
        public_follower
            .to_str()
            .context("UTF-8 public follower executable")?
            .into(),
    );
    ensure!(
        master_follower.is_file(),
        "actual master follower executable missing"
    );
    report.tool_environment.insert(
        "EVE_MASTER_BINARY".into(),
        master_follower
            .to_str()
            .context("UTF-8 master follower executable")?
            .into(),
    );
    let normal = artifacts.join("eve-validator-normal");
    std::fs::copy(&validator, &normal)?;
    report.tool_environment.insert(
        "EVE_VALIDATOR_NORMAL_BINARY".into(),
        normal
            .to_str()
            .context("UTF-8 default validator executable")?
            .into(),
    );
    require_command_success(
        root,
        artifacts,
        report,
        env!("CARGO"),
        &[
            "clippy",
            "--locked",
            "-p",
            "eve-validator",
            "--all-targets",
            "--features",
            "development-acceptance",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    require_command_success(
        root,
        artifacts,
        report,
        env!("CARGO"),
        &[
            "build",
            "--locked",
            "-p",
            "eve-validator",
            "--features",
            "development-acceptance",
        ],
    )?;
    ensure!(
        validator.is_file(),
        "actual validator acceptance executable missing"
    );
    let comet = report
        .tool_environment
        .get("COMETBFT_BINARY")
        .context("verified native engine missing")?
        .clone();
    let digest = report
        .tool_environment
        .get("COMETBFT_SHA256")
        .context("verified native engine digest missing")?
        .clone();
    report.tool_environment.insert(
        "EVE_VALIDATOR_DEV_BINARY".into(),
        validator
            .to_str()
            .context("UTF-8 validator executable")?
            .into(),
    );
    report.tool_environment.insert("EVE_COMET".into(), comet);
    report
        .tool_environment
        .insert("EVE_COMET_SHA256".into(), digest);
    let transition = root.join(
        "tests/acceptance/validator-consensus/tests/fixtures/AcceptanceValidatorTransitions.sol",
    );
    report.tool_environment.insert(
        "EVE_B3_TRANSITION_SOURCE".into(),
        transition
            .to_str()
            .context("UTF-8 transition fixture source")?
            .into(),
    );
    if let Some(evidence) = report
        .tool_evidence
        .as_mut()
        .and_then(serde_json::Value::as_object_mut)
    {
        evidence.insert(
            "b3_validator_acceptance_binary_sha256".into(),
            compute_artifact_digest(&validator)?.into(),
        );
        evidence.insert(
            "b3_validator_normal_binary_sha256".into(),
            compute_artifact_digest(&normal)?.into(),
        );
        evidence.insert(
            "public_follower_binary_sha256".into(),
            compute_artifact_digest(&public_follower)?.into(),
        );
        evidence.insert(
            "master_follower_binary_sha256".into(),
            compute_artifact_digest(&master_follower)?.into(),
        );
    }
    report.topology = "Single-host four independent classical EVE validator processes, distinct disposable signing keys/stores, pinned native engines, bounded owned peer proxies and real durable application execution; independent machine failure domains and PQ activation are not demonstrated".into();
    report.key_epochs = "Disposable genesis-enrolled Ed25519 identities; genesis-bound authenticated development acceptance transitions only; no production key ceremony".into();
    Ok(())
}
