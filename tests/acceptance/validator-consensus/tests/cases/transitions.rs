// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{
    Cluster, ClusterOptions,
    cluster::wait_for_height,
    cluster_lease, collect_certified_history, compare_stopped_stores,
    fixture::TRANSITION_ADDRESS,
    replay_history, signed_transaction,
    transitions::{submit_transition, verify_default_build_guards, verify_transition_receipt},
};
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
    )
    .context("B3_TC07_CREATE")?;
    verify_default_build_guards(&cluster).context("B3_TC07_BUILD_GUARDS")?;
    cluster.start().context("B3_TC07_START")?;
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
        let height = submit_transition(&cluster, &tx, action)?;
        let retained = collect_certified_history(&cluster, 3, height, &changes)
            .context("B3_TC07_CERTIFICATE_HISTORY")?;
        let executed = replay_history(&cluster, &retained).context("B3_TC07_REPLAY")?;
        let commit = executed
            .get(height as usize)
            .context("B3_TC07_EXECUTION_MISSING")?;
        let index = commit
            .block
            .transactions
            .iter()
            .position(|raw| raw == &tx)
            .context("B3_TC07_TRANSACTION_MISSING")?;
        verify_transition_receipt(
            commit.block.receipts[index].as_ref(),
            cluster.fixture_digest,
        )
        .context("B3_TC07_RECEIPT")?;
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
            .map_err(|error| anyhow::anyhow!("updated canonical roster: {error:?}"))
            .context("B3_TC07_ROSTER")?;
        changes.insert(height + 2, roster.clone());
        triggers.push((
            height,
            hash_validator_set(&roster)
                .map_err(|error| anyhow::anyhow!("updated set hash: {error:?}"))
                .context("B3_TC07_ROSTER_HASH")?,
        ));
        wait_for_height(&mut cluster, &[1, 2, 3, 4], height + 3).context("B3_TC07_PROGRESS")?;
    }
    let through = triggers.last().unwrap().0 + 3;
    let history = collect_certified_history(&cluster, 3, through, &changes)
        .context("B3_TC07_FINAL_HISTORY")?;
    for &(trigger, hash) in &triggers {
        ensure!(
            history
                .get(trigger as usize)
                .context("B3_TC07_HISTORY_MISSING")?
                .block
                .header
                .next_validators_hash
                == hash,
            "B3_TC07_NEXT_SET_HASH: H+1 next-set hash missing"
        );
        ensure!(
            history
                .get((trigger + 1) as usize)
                .context("B3_TC07_HISTORY_MISSING")?
                .block
                .header
                .validators_hash
                == hash,
            "B3_TC07_ACTIVE_SET_HASH: H+2 active-set hash missing"
        );
        let last = history
            .get((trigger + 2) as usize)
            .context("B3_TC07_HISTORY_MISSING")?
            .block
            .last_commit
            .as_ref()
            .context("B3_TC07_PREVIOUS_COMMIT_MISSING")?;
        ensure!(
            last.height == trigger + 2
                && last.block_id
                    == history
                        .get((trigger + 1) as usize)
                        .context("B3_TC07_HISTORY_MISSING")?
                        .commit
                        .block_id,
            "B3_TC07_PREVIOUS_COMMIT: H+3 previous-commit history was rebound to wrong height"
        );
    }
    let replay = replay_history(&cluster, &history).context("B3_TC07_FINAL_REPLAY")?;
    ensure!(
        replay
            .last()
            .context("B3_TC07_REPLAY_EMPTY")?
            .state
            .accounts
            .get(&cluster.authority_address)
            .context("B3_TC07_AUTHORITY_MISSING")?
            .nonce
            == 3,
        "B3_TC07_NONCE: transition sequence was not executed exactly once"
    );
    cluster.stop().context("B3_TC07_STOP")?;
    let callbacks = crate::support::history::read_finalized_callbacks(&cluster, 3)
        .context("B3_TC07_CALLBACKS")?;
    for (trigger, _) in triggers {
        let previous = history
            .get((trigger + 1) as usize)
            .context("B3_TC07_HISTORY_MISSING")?;
        let callback = callbacks
            .get(&(trigger + 3))
            .context("B3_TC07_CALLBACK_MISSING")?;
        let votes = callback
            .decided_last_commit
            .as_ref()
            .context("B3_TC07_DECIDED_COMMIT_MISSING")?;
        ensure!(
            votes.round == previous.commit.round && votes.votes.len() == previous.validators.len(),
            "B3_TC07_CALLBACK_ROSTER: H+3 callback uses wrong historical round/roster"
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
                .context("B3_TC07_VOTE_VALIDATOR_MISSING")?;
            ensure!(
                info.address == validator_address(&validator.public_key)
                    && info.power == validator.voting_power
                    && vote.block_id_flag == signature.block_id_flag,
                "B3_TC07_CALLBACK_VOTE: H+3 callback differs from applicable H+2 roster/commit"
            );
        }
    }
    compare_stopped_stores(&cluster, &[1, 2, 3, 4], &replay).context("B3_TC07_STORES")?;
    Ok(())
}
