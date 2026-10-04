// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::cases::{
    master_follower::create_master_namespace,
    public_follower::{PublicFollower, create_public_follower},
};
use crate::support::Cluster;
use anyhow::{Result, ensure};
use std::os::unix::fs::{DirBuilderExt, MetadataExt};

/// Only fixture paths change; the release executable, configured genesis,
/// listeners and actual native source come from the existing public fixture.
pub(in crate::cases) fn create_checkpoint_follower(
    cluster: &Cluster,
) -> Result<(tempfile::TempDir, PublicFollower)> {
    let namespace = create_master_namespace()?;
    let root = namespace.path().canonicalize()?;
    ensure!(
        root.metadata()?.mode() & 0o077 == 0,
        "PUBLIC_CHECKPOINT_NAMESPACE_MODE"
    );
    let artifact = root.join("local-tests");
    std::fs::DirBuilder::new().mode(0o700).create(&artifact)?;
    let mut follower = create_public_follower(cluster)?;
    follower.repository_root = root;
    follower.data = artifact.join("public-checkpoint-state");
    follower.stdout = artifact.join("public-checkpoint.stdout");
    follower.stderr = artifact.join("public-checkpoint.stderr");
    Ok((namespace, follower))
}
