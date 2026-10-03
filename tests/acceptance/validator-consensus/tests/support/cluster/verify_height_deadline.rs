// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::Cluster;
use anyhow::Result;
use std::time::Instant;

pub(super) fn verify_height_deadline(
    cluster: &Cluster,
    nodes: &[usize],
    height: i64,
    deadline: Instant,
) -> Result<()> {
    if Instant::now() < deadline {
        return Ok(());
    }
    super::write_height_deadline_diagnostics::write_height_deadline_diagnostics(cluster, nodes)?;
    anyhow::bail!(
        "B3_PROGRESS_DEADLINE: native/application progress deadline at {height}; artifacts {}",
        cluster.artifact.display()
    )
}
