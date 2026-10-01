// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Cluster;
use anyhow::{Context, Result, ensure};
use eve_state::StateCommit;
use eve_storage::state::{
    capture_state_snapshot, development_state_storage_budget, open_state_repository,
    read_snapshot_commit, state_reader,
};

pub(crate) fn compare_stopped_stores(
    cluster: &Cluster,
    nodes: &[usize],
    expected: &[StateCommit],
) -> Result<()> {
    for &index in nodes {
        ensure!(
            cluster.nodes[index].child.is_none(),
            "stop owned writer before independent store inspection"
        );
        let repository = open_state_repository(
            &cluster.nodes[index].data.join("state"),
            &cluster.genesis,
            development_state_storage_budget(),
        )?;
        let reader = state_reader(&repository);
        let snapshot = capture_state_snapshot(&reader)?;
        for commit in expected {
            let retained =
                read_snapshot_commit(&snapshot, commit.target.height)?.with_context(|| {
                    format!(
                        "expected finalized recovery commit missing at node {index}, height {}",
                        commit.target.height
                    )
                })?;
            ensure!(
                retained == *commit,
                "canonical replay/stored state differs at node {index}, height {}",
                commit.target.height
            );
        }
    }
    Ok(())
}
