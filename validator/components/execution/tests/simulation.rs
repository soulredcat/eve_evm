// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/build_complete_parent.rs"]
mod parent_fixture;
mod support;

use alloy_primitives::{Bytes, U256};
use eve_evm::{
    ExecutionResult, SimulationContext, SimulationError, SimulationLimits, SimulationRequest,
    estimate_complete_state_gas, simulate_complete_state,
};
use eve_state::{development_state_budget, encode_state_commit};

fn request() -> SimulationRequest {
    SimulationRequest {
        from: support::sender(),
        to: Some(parent_fixture::CONTRACT),
        value: U256::ZERO,
        data: Bytes::new(),
        gas: Some(100_000),
        gas_price: None,
        max_fee_per_gas: None,
        max_priority_fee_per_gas: None,
        transaction_type: None,
        access_list: None,
    }
}

fn limits() -> SimulationLimits {
    SimulationLimits {
        maximum_gas: 30_000_000,
        maximum_calldata_bytes: 131_072,
        maximum_memory_bytes: 16 * 1_048_576,
        maximum_estimation_attempts: 32,
        maximum_access_list_entries: 256,
        maximum_access_list_storage_keys: 1_024,
    }
}

#[test]
fn unsigned_storage_call_and_estimation_leave_complete_canonical_state_unchanged() {
    let parent = parent_fixture::build_complete_parent(None);
    let budget = development_state_budget();
    let limits = limits();
    let before = encode_state_commit(&parent, &budget).unwrap();
    let context = SimulationContext {
        state: &parent.state,
        version: &parent.target,
        header: &parent.block.header,
        state_budget: &budget,
        limits: &limits,
        reserved_clone_bytes: 16 * 1_048_576,
    };
    let result = simulate_complete_state(&context, &request()).unwrap();
    assert!(result.execution.is_success());
    assert_eq!(result.execution.tx_gas_used(), 43_106);
    let estimate = estimate_complete_state_gas(&context, &request()).unwrap();
    assert_eq!(estimate.gas_limit, 43_106);
    assert_eq!(estimate.gas_used_at_limit, 43_106);
    assert!(estimate.attempts <= 32);
    assert_eq!(encode_state_commit(&parent, &budget).unwrap(), before);
}

#[test]
fn simulation_preserves_basefee_allows_contract_caller_and_does_not_synthesize_balance() {
    let parent = parent_fixture::build_complete_parent(Some(
        &hex::decode("486000523a60205260406000f3").unwrap(),
    ));
    let budget = development_state_budget();
    let limits = limits();
    let context = SimulationContext {
        state: &parent.state,
        version: &parent.target,
        header: &parent.block.header,
        state_budget: &budget,
        limits: &limits,
        reserved_clone_bytes: 16 * 1_048_576,
    };
    let mut call = request();
    call.from = parent_fixture::CONTRACT;
    let result = simulate_complete_state(&context, &call).unwrap();
    let output = result.execution.output().unwrap();
    assert_eq!(
        U256::from_be_slice(&output[..32]),
        U256::from(1_000_000_000_u64)
    );
    assert_eq!(U256::from_be_slice(&output[32..]), U256::ZERO);
    call.value = U256::from(101);
    assert_eq!(
        simulate_complete_state(&context, &call).unwrap_err(),
        SimulationError::InvalidRequest("simulation value exceeds balance")
    );
}

#[test]
fn simulation_and_estimation_propagate_real_revert_data_and_bound_resources() {
    let parent = parent_fixture::build_complete_parent(Some(
        &hex::decode("63deadbeef6000526004601cfd").unwrap(),
    ));
    let budget = development_state_budget();
    let limits = limits();
    let mut context = SimulationContext {
        state: &parent.state,
        version: &parent.target,
        header: &parent.block.header,
        state_budget: &budget,
        limits: &limits,
        reserved_clone_bytes: 16 * 1_048_576,
    };
    let result = simulate_complete_state(&context, &request()).unwrap();
    assert!(matches!(result.execution, ExecutionResult::Revert { .. }));
    assert_eq!(
        result.execution.output().unwrap().as_ref(),
        [0xde, 0xad, 0xbe, 0xef]
    );
    assert_eq!(
        estimate_complete_state_gas(&context, &request()),
        Err(SimulationError::Revert(Bytes::from_static(&[
            0xde, 0xad, 0xbe, 0xef
        ])))
    );
    context.reserved_clone_bytes = 1;
    assert!(matches!(
        simulate_complete_state(&context, &request()),
        Err(SimulationError::Complete(_))
    ));
    context.reserved_clone_bytes = 16 * 1_048_576;
    let mut call = request();
    call.gas = Some(30_000_001);
    assert!(matches!(
        simulate_complete_state(&context, &call),
        Err(SimulationError::Limit(_))
    ));
    call = request();
    call.gas_price = Some(1);
    call.max_fee_per_gas = Some(2);
    assert!(matches!(
        simulate_complete_state(&context, &call),
        Err(SimulationError::InvalidRequest(_))
    ));
    let small_limits = SimulationLimits {
        maximum_calldata_bytes: 1,
        ..limits
    };
    context.limits = &small_limits;
    call = request();
    call.data = Bytes::from_static(&[1, 2]);
    assert!(matches!(
        simulate_complete_state(&context, &call),
        Err(SimulationError::Limit(_))
    ));
}

#[test]
fn simulation_memory_cap_halts_actual_execution_and_transfer_estimate_is_not_a_placeholder() {
    let parent =
        parent_fixture::build_complete_parent(Some(&hex::decode("60016110005300").unwrap()));
    let budget = development_state_budget();
    let limits = SimulationLimits {
        maximum_memory_bytes: 1_024,
        ..limits()
    };
    let context = SimulationContext {
        state: &parent.state,
        version: &parent.target,
        header: &parent.block.header,
        state_budget: &budget,
        limits: &limits,
        reserved_clone_bytes: 16 * 1_048_576,
    };
    assert!(
        simulate_complete_state(&context, &request())
            .unwrap()
            .execution
            .is_halt()
    );
    let mut call = request();
    call.to = Some(support::environment().proposer);
    call.value = U256::from(1);
    assert_eq!(
        estimate_complete_state_gas(&context, &call)
            .unwrap()
            .gas_limit,
        21_000
    );
}
