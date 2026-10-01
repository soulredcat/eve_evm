// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{
    Cluster, ClusterOptions, cluster::wait_for_height, cluster_lease, collect_certified_history,
    compare_stopped_stores, fixture::TRANSITION_ADDRESS, replay_history, signed_transaction,
    submit_transaction,
};
use alloy_consensus::ReceiptEnvelope;
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::{Address, Bytes, keccak256};
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::consensus::certificates::validator_address;
use eve_consensus_comet::consensus::certificates::{
    ClassicalValidator, canonicalize_validator_set, hash_validator_set,
};
use std::collections::BTreeMap;

#[test]
fn t_c07_authenticated_fixture_rotates_leaves_and_jails_at_native_historical_boundaries()
-> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create(
        "tc07",
        ClusterOptions {
            transitions: true,
            observer: true,
            poison: false,
        },
    )?;
    let normal = std::env::var_os("EVE_VALIDATOR_NORMAL_BINARY")
        .context("default validator binary required")?;
    let command = crate::support::process::node_command(&cluster, 0, "init-dev", "");
    let arguments: Vec<_> = command
        .get_args()
        .map(|argument| argument.to_owned())
        .collect();
    let rejected = std::process::Command::new(&normal)
        .args(&arguments)
        .output()?;
    ensure!(
        !rejected.status.success(),
        "default build accepted temporary adapter genesis"
    );
    let flag = arguments
        .iter()
        .position(|argument| argument == "--acceptance-fixture")
        .context("fixture CLI flag missing")?;
    let without_fixture: Vec<_> = arguments
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != flag && *index != flag + 1)
        .map(|(_, argument)| argument)
        .collect();
    ensure!(
        !std::process::Command::new(&normal)
            .args(&without_fixture)
            .output()?
            .status
            .success()
            && !std::process::Command::new(&cluster.binary)
                .args(&without_fixture)
                .output()?
                .status
                .success(),
        "marked genesis ran with missing acceptance manifest or default application rules"
    );
    cluster.start()?;
    let mut roster: Vec<_> = cluster
        .nodes
        .iter()
        .take(4)
        .map(|node| ClassicalValidator {
            public_key: node.public_key,
            voting_power: 10_000,
        })
        .collect();
    let mut changes = BTreeMap::new();
    let mut triggers = Vec::new();
    for action in 1..=3_u8 {
        let mut input = keccak256(b"transition(uint8)")[..4].to_vec();
        input.extend_from_slice(&[0; 31]);
        input.push(action);
        let tx = signed_transaction(
            &cluster.authority,
            u64::from(action - 1),
            Address::from_slice(&hex::decode(TRANSITION_ADDRESS)?),
            200_000,
            Bytes::from(input),
        );
        let height = submit_transaction(&cluster, 3, &tx)?;
        let retained = collect_certified_history(&cluster, 3, height, &changes)?;
        let executed = replay_history(&cluster, &retained)?;
        let commit = &executed[height as usize];
        let index = commit
            .block
            .transactions
            .iter()
            .position(|raw| raw == &tx)
            .context("transition source transaction missing")?;
        let mut receipt_bytes = commit.block.receipts[index].as_ref();
        let receipt = ReceiptEnvelope::decode_2718(&mut receipt_bytes)
            .map_err(|_| anyhow::anyhow!("actual transition receipt invalid"))?;
        ensure!(
            receipt.is_success()
                && receipt_bytes.is_empty()
                && receipt.logs().iter().any(|log| log.address
                    == Address::from_slice(&hex::decode(TRANSITION_ADDRESS).unwrap())
                    && log
                        .topics()
                        .get(1)
                        .is_some_and(|topic| topic.as_slice() == cluster.fixture_digest)),
            "native update source has no successful canonical fixture receipt"
        );
        if action == 1 {
            roster.retain(|validator| validator.public_key != cluster.nodes[0].public_key);
            roster.push(ClassicalValidator {
                public_key: cluster.nodes[4].public_key,
                voting_power: 10_000,
            });
        } else {
            roster.retain(|validator| {
                validator.public_key != cluster.nodes[usize::from(action - 1)].public_key
            });
        }
        roster = canonicalize_validator_set(&roster)
            .map_err(|error| anyhow::anyhow!("updated canonical roster: {error:?}"))?;
        changes.insert(height + 2, roster.clone());
        triggers.push((
            height,
            hash_validator_set(&roster)
                .map_err(|error| anyhow::anyhow!("updated set hash: {error:?}"))?,
        ));
        wait_for_height(&mut cluster, &[1, 2, 3, 4], height + 3)?;
    }
    let through = triggers.last().unwrap().0 + 3;
    let history = collect_certified_history(&cluster, 3, through, &changes)?;
    for &(trigger, hash) in &triggers {
        ensure!(
            history[trigger as usize].block.header.next_validators_hash == hash,
            "H+1 next-set hash missing"
        );
        ensure!(
            history[(trigger + 1) as usize].block.header.validators_hash == hash,
            "H+2 active-set hash missing"
        );
        let last = history[(trigger + 2) as usize]
            .block
            .last_commit
            .as_ref()
            .context("H+3 previous commit missing")?;
        ensure!(
            last.height == trigger + 2
                && last.block_id == history[(trigger + 1) as usize].commit.block_id,
            "H+3 previous-commit history was rebound to wrong height"
        );
    }
    let replay = replay_history(&cluster, &history)?;
    ensure!(
        replay.last().unwrap().state.accounts[&cluster.authority_address].nonce == 3,
        "transition sequence was not executed exactly once"
    );
    cluster.stop()?;
    let callbacks = crate::support::history::read_finalized_callbacks(&cluster, 3)?;
    for (trigger, _) in triggers {
        let previous = &history[(trigger + 1) as usize];
        let callback = &callbacks[&(trigger + 3)];
        let votes = callback
            .decided_last_commit
            .as_ref()
            .context("persisted H+3 native decided_last_commit missing")?;
        ensure!(
            votes.round == previous.commit.round && votes.votes.len() == previous.validators.len(),
            "H+3 callback uses wrong historical round/roster"
        );
        for ((vote, validator), signature) in votes
            .votes
            .iter()
            .zip(&previous.validators)
            .zip(&previous.commit.signatures)
        {
            let info = vote
                .validator
                .as_ref()
                .context("persisted native vote validator missing")?;
            ensure!(
                info.address == validator_address(&validator.public_key)
                    && info.power == validator.voting_power
                    && vote.block_id_flag == signature.block_id_flag,
                "H+3 actual callback differs from applicable H+2 roster/commit"
            );
        }
    }
    compare_stopped_stores(&cluster, &[1, 2, 3, 4], &replay)?;
    Ok(())
}
