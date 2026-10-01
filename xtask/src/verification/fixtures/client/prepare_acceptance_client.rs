// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::copy_client_sources::copy_client_sources;
use crate::{
    provisioning::{compute_artifact_digest, resolve_contained_path},
    verification::{
        commands::require_command_success::require_command_success,
        types::report_types::VerificationReport,
    },
};
use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};

/// Every gate compiles a fresh pinned client and runs it only after strict success.
pub fn prepare_acceptance_client(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
) -> Result<PathBuf> {
    let source = root.join("tests/acceptance/serial-rpc/client");
    let destination = resolve_contained_path(artifacts, Path::new("b2-client"))?;
    ensure!(!destination.exists(), "client output namespace must be new");
    std::fs::create_dir(&destination)?;
    for name in ["package.json", "package-lock.json", "tsconfig.json"] {
        let input = resolve_contained_path(&source, Path::new(name))?;
        let metadata = std::fs::symlink_metadata(&input)?;
        ensure!(
            metadata.is_file() && metadata.len() <= 262_144,
            "invalid client manifest"
        );
        std::fs::copy(&input, destination.join(name))?;
    }
    let mut bytes = 0;
    copy_client_sources(&source.join("src"), &destination.join("src"), &mut bytes)?;
    ensure!(bytes != 0, "empty client source");
    let node = report
        .tool_environment
        .get("EVE_NODE_BINARY")
        .context("pinned Node required")?
        .clone();
    let solc = report
        .tool_environment
        .get("EVE_SOLC_BINARY")
        .context("pinned Solc required")?
        .clone();
    let npm = Path::new(&node)
        .parent()
        .and_then(Path::parent)
        .context("pinned Node distribution root")?
        .join("lib/node_modules/npm/bin/npm-cli.js");
    let package = destination.to_str().context("UTF-8 client path")?;
    let cache = resolve_contained_path(root, Path::new("local-tests/toolchain-b0/npm-cache"))?;
    require_command_success(
        root,
        artifacts,
        report,
        &node,
        &[
            npm.to_str().context("UTF-8 npm path")?,
            "ci",
            "--prefix",
            package,
            "--ignore-scripts",
            "--no-audit",
            "--no-fund",
            "--cache",
            cache.to_str().context("UTF-8 npm cache")?,
        ],
    )?;
    ensure!(
        compute_artifact_digest(&source.join("package-lock.json"))?
            == compute_artifact_digest(&destination.join("package-lock.json"))?,
        "npm changed pinned client lock"
    );
    let compiler = destination.join("node_modules/typescript/bin/tsc");
    let project = destination.join("tsconfig.json");
    require_command_success(
        root,
        artifacts,
        report,
        &node,
        &[
            compiler.to_str().context("UTF-8 TypeScript path")?,
            "--project",
            project.to_str().context("UTF-8 project path")?,
        ],
    )?;
    let entry = destination.join("build/run_acceptance.js");
    ensure!(entry.is_file(), "strict client build entry missing");
    report
        .tool_environment
        .insert("EVE_ACCEPTANCE_NODE".into(), node);
    report
        .tool_environment
        .insert("EVE_ACCEPTANCE_SOLC".into(), solc);
    report.tool_environment.insert(
        "EVE_ACCEPTANCE_CLIENT_ENTRY".into(),
        entry.to_str().context("UTF-8 client entry")?.into(),
    );
    Ok(entry)
}
