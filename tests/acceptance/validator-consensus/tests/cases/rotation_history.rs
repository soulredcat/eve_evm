// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{
    Cluster, ClusterOptions, cluster_lease, collect_certified_history, compare_stopped_stores,
    replay_history,
};
use anyhow::{Result, ensure};
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn t_c01_four_actual_engines_rotate_proposers_and_replay_one_history() -> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("tc01", ClusterOptions::default())?;
    ensure!(
        cluster
            .nodes
            .iter()
            .map(|node| node.node_id.clone())
            .collect::<BTreeSet<_>>()
            .len()
            == 4,
        "routing identities must be distinct"
    );
    ensure!(
        cluster
            .nodes
            .iter()
            .map(|node| node.public_key)
            .collect::<BTreeSet<_>>()
            .len()
            == 4,
        "signing identities must be distinct"
    );
    cluster.start()?;
    crate::support::cluster::wait_for_height(&mut cluster, &[0, 1, 2, 3], 9)?;
    let history = collect_certified_history(&cluster, 0, 8, &BTreeMap::new())?;
    let proposers: BTreeSet<_> = history
        .iter()
        .map(|certified| certified.block.header.proposer_address.clone())
        .collect();
    ensure!(
        proposers.len() == 4,
        "actual native proposer rotation incomplete"
    );
    for node in 1..4 {
        let peer = collect_certified_history(&cluster, node, 8, &BTreeMap::new())?;
        ensure!(
            peer.iter()
                .zip(&history)
                .all(|(left, right)| left.block.block_id == right.block.block_id),
            "validators committed different native histories"
        );
    }
    let replay = replay_history(&cluster, &history)?;
    cluster.stop()?;
    compare_stopped_stores(&cluster, &[0, 1, 2, 3], &replay)?;
    Ok(())
}
