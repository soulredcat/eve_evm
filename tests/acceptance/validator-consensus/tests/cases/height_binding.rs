// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{
    Cluster, ClusterOptions, cluster::wait_for_height, cluster_lease, collect_certified_history,
    replay_history, signed_transaction, submit_transaction,
};
use alloy_primitives::{Address, Bytes};
use anyhow::{Result, ensure};
use std::collections::BTreeMap;

#[test]
fn t_c09_execution_commitment_at_h_is_bound_by_actual_native_header_h_plus_one() -> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("tc09", ClusterOptions::default())?;
    cluster.start()?;
    let tx = signed_transaction(
        &cluster.authority,
        0,
        Address::repeat_byte(0x99),
        21_000,
        Bytes::new(),
    );
    let height = submit_transaction(&cluster, 0, &tx)?;
    wait_for_height(&mut cluster, &[0, 1, 2, 3], height + 2)?;
    let history = collect_certified_history(&cluster, 0, height + 1, &BTreeMap::new())?;
    let replay = replay_history(&cluster, &history)?;
    ensure!(
        cluster.genesis.target.application.is_none(),
        "invented genesis AppCommitment(0)"
    );
    ensure!(
        history[0].block.header.app_hash == cluster.genesis.target.content_digest.as_slice(),
        "development-only H0 content anchor differs"
    );
    let commitment = replay[height as usize].target.application.unwrap().0;
    ensure!(
        history[height as usize].block.header.app_hash == commitment.as_slice(),
        "H+1 native header does not bind H execution"
    );
    ensure!(
        history[(height - 1) as usize].block.header.app_hash != commitment.as_slice(),
        "off-by-one H header was accepted as H execution anchor"
    );
    cluster.stop()?;
    Ok(())
}
