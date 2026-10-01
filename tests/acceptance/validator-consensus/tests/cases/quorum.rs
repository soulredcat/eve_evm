// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{
    Cluster, ClusterOptions,
    cluster::{node_height, wait_for_height},
    cluster_lease, collect_certified_history, compare_stopped_stores, replay_history,
};
use anyhow::{Result, ensure};
use std::{collections::BTreeMap, time::Duration};

#[test]
fn t_c02_three_of_four_progress_two_of_four_halt_without_quorum_lowering() -> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("tc02", ClusterOptions::default())?;
    cluster.start()?;
    cluster.stop_node(3, false)?;
    let before = node_height(&cluster, 0)?;
    wait_for_height(&mut cluster, &[0, 1, 2], before + 3)?;
    cluster.stop_node(2, false)?;
    std::thread::sleep(Duration::from_secs(8));
    let settled = [node_height(&cluster, 0)?, node_height(&cluster, 1)?];
    std::thread::sleep(Duration::from_secs(8));
    ensure!(
        [node_height(&cluster, 0)?, node_height(&cluster, 1)?] == settled,
        "two voters progressed after inflight drain"
    );
    cluster.restart(2)?;
    cluster.restart(3)?;
    wait_for_height(&mut cluster, &[0, 1, 2, 3], settled[0].max(settled[1]) + 3)?;
    let height = settled[0].max(settled[1]) + 2;
    let history = collect_certified_history(&cluster, 0, height, &BTreeMap::new())?;
    let replay = replay_history(&cluster, &history)?;
    cluster.stop()?;
    compare_stopped_stores(&cluster, &[0, 1, 2, 3], &replay)?;
    Ok(())
}
