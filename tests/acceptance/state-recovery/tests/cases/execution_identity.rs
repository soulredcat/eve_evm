// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support;

use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{Address, B256, Bytes, U256};
use eve_evm::{estimate_clone_reservation, execute_complete_state};
use eve_state::{compute_evm_root, compute_system_root, development_state_budget};
use support::{
    commits::structural_genesis,
    execution::{environment, execution_fixture},
    fixtures::{expected_root, fixture},
};

#[test]
fn ts04_real_execution_matches_independent_roots_receipt_bytes_and_exact_fees() {
    let parent = structural_genesis("execution_parent");
    let input = execution_fixture();
    let raw: Bytes = input["raw_transaction"].as_str().unwrap().parse().unwrap();
    let reservation = estimate_clone_reservation(&parent.state).unwrap();
    let outcome = execute_complete_state(
        &parent.state,
        &parent.target,
        &environment(&parent),
        &[raw],
        &development_state_budget(),
        reservation,
    )
    .unwrap();
    let post = fixture("execution_post");
    assert_eq!(outcome.execution.gas_used, 26_006);
    assert_eq!(
        outcome.execution.outcomes[0].hash,
        expected_root(&input, "transaction_hash")
    );
    assert_eq!(
        outcome.execution.receipts[0].encoded_2718(),
        input["raw_receipt"]
            .as_str()
            .unwrap()
            .parse::<Bytes>()
            .unwrap()
            .to_vec()
    );
    assert_eq!(
        outcome.execution.transactions_root,
        expected_root(&input, "transaction_root")
    );
    assert_eq!(
        outcome.execution.receipts_root,
        expected_root(&input, "receipt_root")
    );
    assert_eq!(
        outcome.execution.fees.burn,
        U256::from(20_804_800_000_000_u64)
    );
    assert_eq!(
        outcome.execution.fees.node_pool,
        U256::from(15_603_600_000_000_u64)
    );
    assert_eq!(
        outcome.execution.fees.validator_pool,
        U256::from(15_603_600_000_000_u64)
    );
    assert_eq!(
        compute_evm_root(&outcome.state.accounts).0,
        expected_root(&post, "evm_root")
    );
    assert_eq!(
        compute_system_root(&outcome.state.system).unwrap().0,
        expected_root(&post, "system_root")
    );
    let contract: Address = input["contract"].as_str().unwrap().parse().unwrap();
    assert_eq!(
        outcome.state.accounts[&contract].storage[&U256::ZERO],
        U256::from(99)
    );
    assert_eq!(
        outcome.state.accounts[&contract].storage[&U256::from(1)],
        U256::from(11)
    );
    assert_eq!(
        parent.state.accounts[&contract].storage[&U256::ZERO],
        U256::from(7)
    );
    assert_eq!(
        parent.state.accounts[&contract].storage[&U256::from(1)],
        U256::from(11)
    );
    let balance = |state: &eve_state::CompleteState| {
        state
            .accounts
            .values()
            .fold(U256::ZERO, |sum, account| sum + account.balance)
    };
    assert_eq!(
        balance(&parent.state) - balance(&outcome.state),
        U256::from(20_804_800_000_000_u64)
    );
}

#[test]
fn ts04_clone_budget_and_stale_complete_view_fail_before_execution() {
    let parent = structural_genesis("execution_parent");
    let input = execution_fixture();
    let raw: Bytes = input["raw_transaction"].as_str().unwrap().parse().unwrap();
    let reservation = estimate_clone_reservation(&parent.state).unwrap();
    assert!(reservation > 0);
    assert!(
        execute_complete_state(
            &parent.state,
            &parent.target,
            &environment(&parent),
            std::slice::from_ref(&raw),
            &development_state_budget(),
            reservation - 1
        )
        .is_err()
    );
    let mut stale = parent.target.clone();
    stale.evm_root.0 = B256::repeat_byte(0x55);
    assert!(
        execute_complete_state(
            &parent.state,
            &stale,
            &environment(&parent),
            &[raw],
            &development_state_budget(),
            reservation
        )
        .is_err()
    );
}
