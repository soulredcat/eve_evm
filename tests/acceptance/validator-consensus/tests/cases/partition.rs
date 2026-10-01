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
fn t_c03_real_two_plus_two_partition_closes_links_and_heals_one_history() -> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("tc03", ClusterOptions::default())?;
    cluster.start()?;
    ensure!(
        cluster
            .proxies
            .observed_edges()
            .iter()
            .any(|&(source, target)| (source < 2) != (target < 2)),
        "no actual cross-group P2P links observed"
    );
    ensure!(
        cluster.proxies.partition(true) > 0,
        "partition did not close existing native P2P streams"
    );
    std::thread::sleep(Duration::from_secs(8));
    let settled: Vec<_> = (0..4)
        .map(|node| node_height(&cluster, node))
        .collect::<Result<_>>()?;
    std::thread::sleep(Duration::from_secs(8));
    let halted: Vec<_> = (0..4)
        .map(|node| node_height(&cluster, node))
        .collect::<Result<_>>()?;
    ensure!(
        settled == halted,
        "2+2 groups committed without native quorum"
    );
    cluster.proxies.partition(false);
    let target = settled.into_iter().max().unwrap() + 3;
    wait_for_height(&mut cluster, &[0, 1, 2, 3], target)?;
    let history = collect_certified_history(&cluster, 0, target - 1, &BTreeMap::new())?;
    for node in 1..4 {
        let peer = collect_certified_history(&cluster, node, target - 1, &BTreeMap::new())?;
        ensure!(
            peer.iter()
                .zip(&history)
                .all(|(left, right)| left.block.block_id == right.block.block_id),
            "healed nodes disagree"
        );
    }
    let replay = replay_history(&cluster, &history)?;
    cluster.stop()?;
    compare_stopped_stores(&cluster, &[0, 1, 2, 3], &replay)?;
    Ok(())
}
