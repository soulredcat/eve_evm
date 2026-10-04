// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    cases::public_follower::{PublicFollower, create_public_follower},
    support::Cluster,
};
use anyhow::{Result, ensure};
use std::os::unix::fs::DirBuilderExt;

/// Fresh consumer fixture for the same generated namespace, never a reset launch counter.
pub(super) fn recreate_checkpoint_follower(
    cluster: &Cluster,
    previous: &PublicFollower,
    phase: &str,
) -> Result<PublicFollower> {
    ensure!(
        previous.child.is_none(),
        "CHECKPOINT_INTERRUPTION_OLD_CHILD_NOT_REAPED"
    );
    ensure!(
        matches!(phase, "baseline" | "restored"),
        "CHECKPOINT_INTERRUPTION_PHASE"
    );
    let artifact = previous.repository_root.join("local-tests");
    ensure!(
        previous.data.starts_with(&artifact),
        "CHECKPOINT_INTERRUPTION_DATA_CONTAINMENT"
    );
    let logs = artifact.join(format!("checkpoint-interruption-{phase}"));
    std::fs::DirBuilder::new().mode(0o700).create(&logs)?;
    let mut follower = create_public_follower(cluster)?;
    follower.repository_root = previous.repository_root.clone();
    follower.data = previous.data.clone();
    follower.stdout = logs.join("public.stdout");
    follower.stderr = logs.join("public.stderr");
    Ok(follower)
}
