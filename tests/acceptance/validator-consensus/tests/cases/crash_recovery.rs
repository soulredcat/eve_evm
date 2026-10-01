// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{
    Cluster, ClusterOptions,
    cluster::{node_height, wait_for_height},
    cluster_lease, collect_certified_history, compare_stopped_stores,
    fixture::REVERT_ADDRESS,
    history::{read_signing_history, record_recovery_phase},
    replay_history, signed_transaction, submit_transaction,
};
use alloy_primitives::{Address, Bytes};
use anyhow::{Context, Result, ensure};
use std::collections::BTreeMap;

#[test]
fn t_c06_actual_signer_and_application_crashes_preserve_signatures_and_single_fees() -> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("tc06", ClusterOptions::default())?;
    cluster.retain_failed_namespace = true;
    cluster.start()?;
    let transaction = signed_transaction(
        &cluster.authority,
        0,
        Address::from_slice(&hex::decode(REVERT_ADDRESS)?),
        100_000,
        Bytes::new(),
    );
    let transaction_height = submit_transaction(&cluster, 0, &transaction)?;
    wait_for_height(&mut cluster, &[0, 1, 2, 3], transaction_height + 1)?;
    record_recovery_phase(&cluster, "before_single_node_crash")?;
    cluster.stop_node(0, true)?;
    let signed_before = read_signing_history(&cluster, 0)?;
    ensure!(
        !signed_before.is_empty(),
        "actual crash had no durable signer history"
    );
    let remaining = node_height(&cluster, 1)?;
    wait_for_height(&mut cluster, &[1, 2, 3], remaining + 2)?;
    cluster.restart(0)?;
    wait_for_height(&mut cluster, &[0, 1, 2, 3], remaining + 3)?;
    record_recovery_phase(&cluster, "before_all_node_crash")?;
    for node in 0..4 {
        cluster.stop_node(node, true)?;
    }
    record_recovery_phase(&cluster, "all_nodes_crashed")?;
    let before_restart = read_signing_history(&cluster, 0)?;
    ensure!(
        before_restart.starts_with(&signed_before),
        "restart changed already durable returned signatures"
    );
    for node in 0..4 {
        cluster.restart(node)?;
    }
    record_recovery_phase(&cluster, "all_nodes_restarted")?;
    wait_for_height(&mut cluster, &[0, 1, 2, 3], remaining + 5)?;
    let height = remaining + 4;
    let history = collect_certified_history(&cluster, 0, height, &BTreeMap::new())?;
    ensure!(
        history
            .iter()
            .flat_map(|block| &block.block.transactions)
            .filter(|raw| raw.as_slice() == transaction.as_ref())
            .count()
            == 1,
        "transaction duplicated after actual app restart"
    );
    let replay = replay_history(&cluster, &history)?;
    ensure!(
        replay.last().unwrap().state.accounts[&cluster.authority_address].nonce == 1,
        "revert fee/nonce applied more than once"
    );
    cluster.stop()?;
    let after = read_signing_history(&cluster, 0)?;
    ensure!(
        after.starts_with(&before_restart),
        "all-node crash altered durable signer prefix"
    );
    for node in 0..4 {
        let records = read_signing_history(&cluster, node)?;
        let address = eve_consensus_comet::consensus::certificates::validator_address(
            &cluster.nodes[node].public_key,
        );
        for certified in &history {
            for signature in certified.commit.signatures.iter().filter(|signature| {
                signature.block_id_flag == 2 && signature.validator_address == address
            }) {
                let retained = records
                    .iter()
                    .find(|record| {
                        record["hrs"]["height"] == certified.commit.height
                            && record["hrs"]["round"] == certified.commit.round
                            && record["hrs"]["step"] == 3
                    })
                    .context("actual returned commit signature has no durable HRS record")?;
                let stored: Vec<u8> = serde_json::from_value(retained["signature"].clone())?;
                ensure!(
                    stored == signature.signature,
                    "native returned commit signature differs from synced record"
                );
            }
        }
    }
    compare_stopped_stores(&cluster, &[0, 1, 2, 3], &replay)?;
    cluster.retain_failed_namespace = false;
    Ok(())
}
