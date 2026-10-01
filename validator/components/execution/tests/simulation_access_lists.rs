// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/build_complete_parent.rs"]
mod parent_fixture;
mod support;

use alloy_primitives::{B256, Bytes, U256};
use eve_evm::{
    AccessList, AccessListItem, SimulationContext, SimulationError, SimulationLimits,
    SimulationRequest, estimate_complete_state_gas, simulate_complete_state,
};
use eve_state::development_state_budget;

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

#[test]
fn inferred_and_explicit_access_list_calls_charge_real_intrinsic_and_warm_storage() {
    let parent = parent_fixture::build_complete_parent(Some(
        &hex::decode("60005460005260206000f3").unwrap(),
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
    let cold = simulate_complete_state(&context, &call).unwrap();
    assert_eq!(cold.execution.tx_gas_used(), 23_118);
    call.access_list = Some(AccessList(vec![AccessListItem {
        address: parent_fixture::CONTRACT,
        storage_keys: vec![B256::ZERO],
    }]));
    let warm = simulate_complete_state(&context, &call).unwrap();
    // Additional intrinsic4300, SLOAD falls2100->100; net increase2300.
    assert_eq!(warm.execution.tx_gas_used(), 25_418);
    call.transaction_type = Some(1);
    assert_eq!(
        simulate_complete_state(&context, &call)
            .unwrap()
            .execution
            .tx_gas_used(),
        25_418
    );
    call.transaction_type = Some(2);
    call.max_fee_per_gas = Some(2_000_000_000);
    assert_eq!(
        simulate_complete_state(&context, &call)
            .unwrap()
            .execution
            .tx_gas_used(),
        25_418
    );
    assert_eq!(
        estimate_complete_state_gas(&context, &call)
            .unwrap()
            .gas_limit,
        25_418
    );
    assert_eq!(
        parent.state.accounts[&parent_fixture::CONTRACT]
            .storage
            .len(),
        0
    );
}

#[test]
fn explicit_types_reject_incompatible_fields_and_lists_are_bounded_before_cloning() {
    let parent = parent_fixture::build_complete_parent(None);
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
    for kind in [3, 4, 127] {
        let mut call = request();
        call.transaction_type = Some(kind);
        assert!(matches!(
            simulate_complete_state(&context, &call),
            Err(SimulationError::InvalidRequest(_))
        ));
    }
    let mut call = request();
    call.transaction_type = Some(0);
    call.access_list = Some(AccessList::default());
    assert!(matches!(
        simulate_complete_state(&context, &call),
        Err(SimulationError::InvalidRequest(_))
    ));
    call = request();
    call.transaction_type = Some(1);
    call.max_fee_per_gas = Some(2_000_000_000);
    assert!(matches!(
        simulate_complete_state(&context, &call),
        Err(SimulationError::InvalidRequest(_))
    ));
    call = request();
    call.transaction_type = Some(2);
    call.gas_price = Some(2_000_000_000);
    assert!(matches!(
        simulate_complete_state(&context, &call),
        Err(SimulationError::InvalidRequest(_))
    ));
    let tiny = SimulationLimits {
        maximum_access_list_entries: 0,
        ..limits
    };
    let bounded = SimulationContext {
        limits: &tiny,
        reserved_clone_bytes: 0,
        ..context
    };
    call = request();
    call.access_list = Some(AccessList(vec![AccessListItem {
        address: parent_fixture::CONTRACT,
        storage_keys: vec![],
    }]));
    assert_eq!(
        simulate_complete_state(&bounded, &call).unwrap_err(),
        SimulationError::Limit("simulation access-list entries")
    );
    let tiny = SimulationLimits {
        maximum_access_list_storage_keys: 0,
        ..limits
    };
    let bounded = SimulationContext {
        limits: &tiny,
        reserved_clone_bytes: 0,
        ..context
    };
    call.access_list.as_mut().unwrap().0[0]
        .storage_keys
        .push(B256::ZERO);
    assert_eq!(
        simulate_complete_state(&bounded, &call).unwrap_err(),
        SimulationError::Limit("simulation access-list storage keys")
    );
}
