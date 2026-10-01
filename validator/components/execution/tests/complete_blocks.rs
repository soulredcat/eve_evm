// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/build_complete_parent.rs"]
mod parent_fixture;
mod support;

use alloy_primitives::{B256, Bytes, TxKind, U256};
use eve_evm::{ExecutionBlockInput, execute_state_block};
use eve_state::{apply_state_journal, development_state_budget, encode_state_commit};

#[test]
fn canonical_block_builder_derives_base_fee_and_matches_atomic_journal() {
    let parent = parent_fixture::build_complete_parent(None);
    let budget = development_state_budget();
    let before = encode_state_commit(&parent, &budget).unwrap();
    let input = ExecutionBlockInput {
        timestamp: parent.target.timestamp + 1,
        proposer: support::environment().proposer,
        previous_consensus_hash: B256::repeat_byte(1),
    };
    let raw = support::legacy(
        0,
        TxKind::Call(parent_fixture::CONTRACT),
        U256::ZERO,
        100_000,
        Bytes::new(),
    );
    let first = execute_state_block(
        &parent,
        &input,
        std::slice::from_ref(&raw),
        &budget,
        16 * 1_048_576,
    )
    .unwrap();
    let again = execute_state_block(&parent, &input, &[raw], &budget, 16 * 1_048_576).unwrap();
    assert_eq!(
        first.commit.block.header.base_fee_per_gas,
        Some(875_000_000)
    );
    assert_eq!(first.commit, again.commit);
    assert_eq!(
        first.commit.state.accounts[&parent_fixture::CONTRACT].storage[&U256::ZERO],
        U256::from(99)
    );
    let replayed =
        apply_state_journal(&parent.state, &parent.target, &first.journal, &budget).unwrap();
    assert_eq!(replayed, first.commit.state);
    assert_eq!(encode_state_commit(&parent, &budget).unwrap(), before);
    let second = execute_state_block(
        &first.commit,
        &ExecutionBlockInput {
            timestamp: input.timestamp + 1,
            ..input
        },
        &[],
        &budget,
        16 * 1_048_576,
    )
    .unwrap();
    assert!(second.commit.block.header.base_fee_per_gas.unwrap() < 875_000_000);
}

#[test]
fn candidate_builder_rejects_invalid_order_time_and_reservation_without_parent_changes() {
    let parent = parent_fixture::build_complete_parent(None);
    let budget = development_state_budget();
    let before = parent.clone();
    let mut input = ExecutionBlockInput {
        timestamp: parent.target.timestamp,
        proposer: support::environment().proposer,
        previous_consensus_hash: B256::ZERO,
    };
    let raw = support::legacy(
        0,
        TxKind::Call(parent_fixture::CONTRACT),
        U256::ZERO,
        100_000,
        Bytes::new(),
    );
    assert!(
        execute_state_block(
            &parent,
            &input,
            &[raw.clone(), raw],
            &budget,
            16 * 1_048_576
        )
        .is_err()
    );
    assert!(execute_state_block(&parent, &input, &[], &budget, 1).is_err());
    input.timestamp -= 1;
    assert!(execute_state_block(&parent, &input, &[], &budget, 16 * 1_048_576).is_err());
    assert_eq!(parent, before);
}
