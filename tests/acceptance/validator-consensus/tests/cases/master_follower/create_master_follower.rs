// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MasterFollower;
use crate::support::Cluster;
use anyhow::{Context, Result, ensure};
use std::path::PathBuf;

pub(super) fn create_master_follower(cluster: &Cluster) -> Result<MasterFollower> {
    let source_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .context("MASTER_FOLLOWER_REPOSITORY_ROOT")?
        .canonicalize()?;
    let artifact = cluster.artifact.canonicalize()?;
    ensure!(
        artifact.starts_with(source_root.join("local-tests")),
        "MASTER_FOLLOWER_DATA_CONTAINMENT"
    );
    let binary = PathBuf::from(
        std::env::var_os("EVE_MASTER_BINARY")
            .context("EVE_MASTER_BINARY release executable is required")?,
    )
    .canonicalize()?;
    ensure!(binary.is_file(), "MASTER_FOLLOWER_BINARY_MISSING");
    let namespace = super::create_master_namespace::create_master_namespace()?;
    let repository_root = namespace.path().canonicalize()?;
    let data = repository_root.join("local-tests/master-follower-state");
    Ok(MasterFollower {
        binary,
        repository_root,
        data,
        genesis: cluster.genesis_path.clone(),
        validator: cluster.nodes[0].rpc,
        stdout: artifact.join("master-follower.stdout"),
        stderr: artifact.join("master-follower.stderr"),
        artifact,
        launch_count: 0,
        child: None,
        process_start: None,
        _namespace: namespace,
    })
}
