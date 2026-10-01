// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/build_complete_parent.rs"]
mod parent_fixture;
mod support;

use alloy_primitives::{Address, B256, Bytes, TxKind, U256};
use eve_evm::{ExecutionBlockInput, NATIVE_INTERFACE_INACTIVE_REVERT_DATA, execute_state_block};
use eve_protocol_config::native::SYSTEM_INTERFACE_ADDRESS;
use eve_state::{SystemValue, development_state_budget};

#[test]
fn inactive_native_direct_call_reverts_custody_and_keeps_included_fee_nonce_semantics() {
    let parent = parent_fixture::build_complete_parent(None);
    let budget = development_state_budget();
    let input = ExecutionBlockInput {
        timestamp: parent.target.timestamp,
        proposer: support::environment().proposer,
        previous_consensus_hash: B256::ZERO,
    };
    let raw = support::legacy(
        0,
        TxKind::Call(SYSTEM_INTERFACE_ADDRESS),
        U256::from(42),
        100_000,
        Bytes::new(),
    );
    let result = execute_state_block(&parent, &input, &[raw], &budget, 16 * 1_048_576).unwrap();
    assert_eq!(
        result.outcomes[0].execution.output().unwrap().as_ref(),
        NATIVE_INTERFACE_INACTIVE_REVERT_DATA
    );
    assert!(!result.outcomes[0].execution.is_success());
    assert_eq!(result.commit.block.header.gas_used, 26_000);
    assert_eq!(
        result.commit.state.accounts[&SYSTEM_INTERFACE_ADDRESS],
        parent.state.accounts[&SYSTEM_INTERFACE_ADDRESS]
    );
    assert_eq!(result.commit.state.accounts[&support::sender()].nonce, 1);
    for (key, record) in &parent.state.system {
        if !matches!(record.value, SystemValue::Fee { .. }) {
            assert_eq!(&result.commit.state.system[key], record);
        }
    }
}

#[test]
fn inactive_native_internal_call_staticcall_and_delegatecall_return_failure() {
    for runtime in [
        "6000600060006000602a61f100620186a0f160005260206000f3",
        "600060006000600061f100620186a0fa60005260206000f3",
        "600060006000600061f100620186a0f460005260206000f3",
    ] {
        let parent = parent_fixture::build_complete_parent(Some(&hex::decode(runtime).unwrap()));
        let input = ExecutionBlockInput {
            timestamp: parent.target.timestamp,
            proposer: support::environment().proposer,
            previous_consensus_hash: B256::ZERO,
        };
        let raw = support::legacy(
            0,
            TxKind::Call(parent_fixture::CONTRACT),
            U256::ZERO,
            200_000,
            Bytes::new(),
        );
        let result = execute_state_block(
            &parent,
            &input,
            &[raw],
            &development_state_budget(),
            16 * 1_048_576,
        )
        .unwrap();
        assert!(result.outcomes[0].execution.is_success());
        assert_eq!(
            U256::from_be_slice(result.outcomes[0].execution.output().unwrap()),
            U256::ZERO
        );
        assert_eq!(
            result.commit.state.accounts[&SYSTEM_INTERFACE_ADDRESS],
            parent.state.accounts[&SYSTEM_INTERFACE_ADDRESS]
        );
        assert_eq!(
            result.commit.state.accounts[&parent_fixture::CONTRACT].balance,
            U256::from(100)
        );
    }
}

#[test]
fn native_boundary_preserves_ethereum_identity_precompile() {
    let parent = parent_fixture::build_complete_parent(None);
    let input = ExecutionBlockInput {
        timestamp: parent.target.timestamp,
        proposer: support::environment().proposer,
        previous_consensus_hash: B256::ZERO,
    };
    let raw = support::legacy(
        0,
        TxKind::Call(Address::with_last_byte(4)),
        U256::ZERO,
        100_000,
        Bytes::from_static(b"ab"),
    );
    let result = execute_state_block(
        &parent,
        &input,
        &[raw],
        &development_state_budget(),
        16 * 1_048_576,
    )
    .unwrap();
    assert!(result.outcomes[0].execution.is_success());
    assert_eq!(
        result.outcomes[0].execution.output().unwrap().as_ref(),
        b"ab"
    );
}
