// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{
    Cluster, ClusterOptions, cluster::wait_for_height, cluster_lease, collect_certified_history,
    compare_stopped_stores, fixture::REVERT_ADDRESS, replay_history, signed_transaction,
    submit_transaction,
};
use alloy_consensus::ReceiptEnvelope;
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::{Address, Bytes};
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::consensus::certificates::validator_address;
use eve_consensus_comet::wire::tendermint::types::{CanonicalProposal, CanonicalVote};
use prost::Message;
use std::collections::BTreeMap;

#[test]
fn t_c05_native_invalid_proposal_is_nil_voted_while_valid_evm_revert_commits() -> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create(
        "tc05",
        ClusterOptions {
            poison: true,
            ..Default::default()
        },
    )?;
    cluster.start()?;
    let transaction = signed_transaction(
        &cluster.authority,
        0,
        Address::from_slice(&hex::decode(REVERT_ADDRESS)?),
        100_000,
        Bytes::new(),
    );
    let height = submit_transaction(&cluster, 0, &transaction)?;
    wait_for_height(&mut cluster, &[0, 1, 2, 3], height + 1)?;
    let history = collect_certified_history(&cluster, 0, height, &BTreeMap::new())?;
    ensure!(
        history[0].commit.round > 0,
        "selected first proposer fault was not rejected before later native round"
    );
    ensure!(
        history
            .iter()
            .all(|block| block.block.transactions.iter().all(|tx| tx != &[0xff])),
        "invalid prepared transaction entered decided history"
    );
    let replay = replay_history(&cluster, &history)?;
    let commit = &replay[height as usize];
    ensure!(
        commit.block.transactions.contains(&transaction),
        "valid revert transaction missing"
    );
    let index = commit
        .block
        .transactions
        .iter()
        .position(|raw| raw == &transaction)
        .unwrap();
    let mut raw = commit.block.receipts[index].as_ref();
    let receipt = ReceiptEnvelope::decode_2718(&mut raw)
        .map_err(|_| anyhow::anyhow!("actual revert receipt decoding failed"))?;
    ensure!(
        !receipt.is_success() && raw.is_empty() && receipt.logs().is_empty(),
        "valid EVM revert did not retain failure receipt without reverted logs"
    );
    ensure!(
        commit.state.accounts[&cluster.authority_address].nonce == 1,
        "valid revert nonce did not advance"
    );
    cluster.stop()?;
    let selected = cluster
        .nodes
        .iter()
        .enumerate()
        .min_by_key(|(_, node)| validator_address(&node.public_key))
        .context("selected proposer missing")?
        .0;
    let expected_trace = format!(
        "EVE_B3_POISONED_PROPOSAL height=1 proposer={} transaction=ff",
        hex::encode(validator_address(&cluster.nodes[selected].public_key))
    );
    let traces: Vec<_> = (0..4)
        .map(|node| std::fs::read_to_string(cluster.nodes[node].data.join("validator.stderr.log")))
        .collect::<std::io::Result<_>>()?;
    ensure!(
        traces
            .iter()
            .map(|output| output
                .lines()
                .filter(|line| line.contains("EVE_B3_POISONED_PROPOSAL"))
                .count())
            .sum::<usize>()
            == 1
            && traces[selected].lines().any(|line| line == expected_trace),
        "bounded selected native Prepare fault did not actually execute once"
    );
    let selected_history = crate::support::history::read_signing_history(&cluster, selected)?;
    let proposal = selected_history
        .iter()
        .find(|record| record["hrs"]["height"] == 1 && record["hrs"]["step"] == 1)
        .context("poisoned native proposal has no durable signature record")?;
    let poisoned_round = proposal["hrs"]["round"]
        .as_i64()
        .context("poisoned proposal round missing")?;
    let proposal_bytes: Vec<u8> = serde_json::from_value(proposal["sign_bytes"].clone())?;
    let signed_proposal = CanonicalProposal::decode_length_delimited(proposal_bytes.as_slice())?;
    let rejected_hash = signed_proposal
        .block_id
        .context("poisoned signed proposal block id missing")?
        .hash;
    let expected_rejection = format!(
        "EVE_B3_REJECTED_PROPOSAL height=1 proposer={} hash={}",
        hex::encode(validator_address(&cluster.nodes[selected].public_key)),
        hex::encode(rejected_hash)
    );
    ensure!(
        i64::from(history[0].commit.round) > poisoned_round,
        "poisoned round was decided instead of rejected"
    );
    let mut nil_voters = 0;
    for (node, trace) in traces.iter().enumerate() {
        let records = crate::support::history::read_signing_history(&cluster, node)?;
        let mut found = false;
        for record in records.iter().filter(|record| {
            record["hrs"]["height"] == 1
                && record["hrs"]["round"] == poisoned_round
                && record["hrs"]["step"] == 2
        }) {
            let bytes: Vec<u8> = serde_json::from_value(record["sign_bytes"].clone())?;
            let vote = CanonicalVote::decode_length_delimited(bytes.as_slice())?;
            found |= vote.block_id.is_none();
        }
        nil_voters += usize::from(found && trace.lines().any(|line| line == expected_rejection));
    }
    ensure!(
        nil_voters >= 3,
        "actual malformed proposal did not reach canonical rejection and native nil prevotes from quorum"
    );
    compare_stopped_stores(&cluster, &[0, 1, 2, 3], &replay)?;
    Ok(())
}
