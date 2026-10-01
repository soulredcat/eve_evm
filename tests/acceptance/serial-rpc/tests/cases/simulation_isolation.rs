// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::development_fixture::development_fixture;
use alloy_primitives::{Bytes, U256};
use eve_evm::{
    SimulationContext, SimulationError, SimulationLimits, SimulationRequest,
    estimate_clone_reservation, estimate_complete_state_gas, simulate_complete_state,
};
use eve_state::encode_state_commit;

#[test]
fn te06_state_changing_call_and_exact_gas_estimation_preserve_both_canonical_domains() {
    // PUSH1(99), PUSH1(0), SSTORE, STOP: cold empty-to-nonzero storage costs
    // 22100 plus six PUSH gas and 21000 intrinsic gas under Shanghai.
    let fixture = development_fixture(Bytes::from_static(&[0x60, 0x63, 0x60, 0, 0x55, 0]));
    let before = encode_state_commit(&fixture.parent, &fixture.budget).unwrap();
    let limits = SimulationLimits {
        maximum_gas: 100_000,
        maximum_calldata_bytes: 131_072,
        maximum_memory_bytes: 1_048_576,
        maximum_estimation_attempts: 32,
        maximum_access_list_entries: 256,
        maximum_access_list_storage_keys: 1024,
    };
    let context = SimulationContext {
        state: &fixture.parent.state,
        version: &fixture.parent.target,
        header: &fixture.parent.block.header,
        state_budget: &fixture.budget,
        limits: &limits,
        reserved_clone_bytes: estimate_clone_reservation(&fixture.parent.state).unwrap(),
    };
    let request = SimulationRequest {
        from: fixture.sender,
        to: Some(fixture.contract),
        value: U256::ZERO,
        data: Bytes::new(),
        gas: Some(100_000),
        gas_price: None,
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
        transaction_type: None,
        access_list: None,
    };
    let outcome = simulate_complete_state(&context, &request).unwrap();
    assert!(outcome.execution.is_success());
    assert_eq!(outcome.execution.tx_gas_used(), 43_106);
    let estimate = estimate_complete_state_gas(&context, &request).unwrap();
    assert_eq!(estimate.gas_limit, 43_106);
    assert_eq!(estimate.gas_used_at_limit, 43_106);
    assert!(estimate.attempts <= limits.maximum_estimation_attempts);
    let mut exact = request.clone();
    exact.gas = Some(estimate.gas_limit);
    assert!(
        simulate_complete_state(&context, &exact)
            .unwrap()
            .execution
            .is_success()
    );
    exact.gas = Some(estimate.gas_limit - 1);
    assert!(
        !simulate_complete_state(&context, &exact)
            .unwrap()
            .execution
            .is_success()
    );
    assert_eq!(
        encode_state_commit(&fixture.parent, &fixture.budget).unwrap(),
        before,
        "Accounts, code, slots, nonce, fee ledger, roots, block and durable identity must remain byte-identical"
    );
    assert!(
        fixture.parent.state.accounts[&fixture.contract]
            .storage
            .is_empty()
    );
}

#[test]
fn te06_reverted_call_and_estimation_cannot_publish_storage_or_fee_changes() {
    // A write followed by REVERT with empty data. Expected failure is execution,
    // not an invalid envelope and never a successful fixed gas placeholder.
    let fixture = development_fixture(Bytes::from_static(&[
        0x60, 55, 0x60, 0, 0x55, 0x60, 0, 0x60, 0, 0xfd,
    ]));
    let before = encode_state_commit(&fixture.parent, &fixture.budget).unwrap();
    let limits = SimulationLimits {
        maximum_gas: 100_000,
        maximum_calldata_bytes: 131_072,
        maximum_memory_bytes: 1_048_576,
        maximum_estimation_attempts: 32,
        maximum_access_list_entries: 256,
        maximum_access_list_storage_keys: 1024,
    };
    let context = SimulationContext {
        state: &fixture.parent.state,
        version: &fixture.parent.target,
        header: &fixture.parent.block.header,
        state_budget: &fixture.budget,
        limits: &limits,
        reserved_clone_bytes: estimate_clone_reservation(&fixture.parent.state).unwrap(),
    };
    let request = SimulationRequest {
        from: fixture.sender,
        to: Some(fixture.contract),
        value: U256::ZERO,
        data: Bytes::new(),
        gas: Some(100_000),
        gas_price: None,
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
        transaction_type: None,
        access_list: None,
    };
    assert!(matches!(
        simulate_complete_state(&context, &request)
            .unwrap()
            .execution,
        eve_evm::ExecutionResult::Revert { .. }
    ));
    assert_eq!(
        estimate_complete_state_gas(&context, &request),
        Err(SimulationError::Revert(Bytes::new()))
    );
    assert_eq!(
        encode_state_commit(&fixture.parent, &fixture.budget).unwrap(),
        before
    );
}
