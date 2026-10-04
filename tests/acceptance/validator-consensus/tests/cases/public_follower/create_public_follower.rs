// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::PublicFollower;
use crate::support::Cluster;
use anyhow::{Context, Result, ensure};
use std::{net::TcpListener, path::PathBuf};

pub(in crate::cases) fn create_public_follower(cluster: &Cluster) -> Result<PublicFollower> {
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .context("PUBLIC_FOLLOWER_REPOSITORY_ROOT")?
        .canonicalize()?;
    let artifact = cluster.artifact.canonicalize()?;
    ensure!(
        artifact.starts_with(repository_root.join("local-tests")),
        "PUBLIC_FOLLOWER_DATA_CONTAINMENT"
    );
    let binary = PathBuf::from(
        std::env::var_os("EVE_PUBLIC_BINARY")
            .context("EVE_PUBLIC_BINARY release executable is required")?,
    )
    .canonicalize()?;
    ensure!(binary.is_file(), "PUBLIC_FOLLOWER_BINARY_MISSING");
    let http = TcpListener::bind("127.0.0.1:0")?;
    let ws = TcpListener::bind("127.0.0.1:0")?;
    Ok(PublicFollower {
        binary,
        repository_root,
        data: artifact.join("public-follower-state"),
        genesis: cluster.genesis_path.clone(),
        validator: cluster.nodes[0].rpc,
        http: http.local_addr()?,
        ws: ws.local_addr()?,
        stdout: artifact.join("public-follower.stdout"),
        stderr: artifact.join("public-follower.stderr"),
        launch_count: 0,
        child: None,
        process_start: None,
        checkpoint_height: None,
    })
}
