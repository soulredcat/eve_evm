// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{client::prepare_acceptance_client, shanghai::prepare_shanghai_fixtures};
use crate::{
    provisioning::compute_artifact_digest,
    verification::{
        commands::require_command_success::require_command_success,
        types::report_types::VerificationReport,
    },
};
use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};

/// Bind real compiled role binaries and checked fixture tools to acceptance children.
pub fn prepare_b2_acceptance(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
) -> Result<()> {
    report.topology = "Single-host component fixtures, one Comet engine API fixture, two independent development public stores/processes and one master RPC composition; no four-validator devnet".into();
    prepare_shanghai_fixtures(root, artifacts, report)?;
    prepare_acceptance_client(root, artifacts, report)?;
    require_command_success(
        root,
        artifacts,
        report,
        env!("CARGO"),
        &["build", "--locked", "-p", "eve-public", "-p", "eve-master"],
    )?;
    let configured = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target"));
    let target = if configured.is_absolute() {
        configured
    } else {
        root.join(configured)
    };
    let public = target
        .join("debug")
        .join(format!("eve-public{}", std::env::consts::EXE_SUFFIX));
    let master = target
        .join("debug")
        .join(format!("eve-master{}", std::env::consts::EXE_SUFFIX));
    ensure!(
        public.is_file() && master.is_file(),
        "acceptance role binaries missing"
    );
    report.tool_environment.insert(
        "EVE_PUBLIC_DEV_BINARY".into(),
        public.to_str().context("UTF-8 public binary")?.into(),
    );
    report.tool_environment.insert(
        "EVE_MASTER_DEV_BINARY".into(),
        master.to_str().context("UTF-8 master binary")?.into(),
    );
    let identities = serde_json::to_value(std::collections::BTreeMap::from([
        ("public_binary_sha256", compute_artifact_digest(&public)?),
        ("master_binary_sha256", compute_artifact_digest(&master)?),
        (
            "client_lock_sha256",
            compute_artifact_digest(
                &root.join("tests/acceptance/serial-rpc/client/package-lock.json"),
            )?,
        ),
    ]))?;
    if let Some(evidence) = report
        .tool_evidence
        .as_mut()
        .and_then(serde_json::Value::as_object_mut)
    {
        evidence.insert("b2_acceptance_identities".into(), identities);
    }
    Ok(())
}
