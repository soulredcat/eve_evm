// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/build_complete_parent.rs"]
mod parent_fixture;
mod support;

use alloy_primitives::{Bytes, U256};
use eve_evm::{
    SimulationContext, SimulationError, SimulationLimits, SimulationRequest,
    simulate_complete_state,
};
use eve_state::development_state_budget;

#[test]
fn unsigned_type_two_defaults_zero_tip_and_reports_actual_effective_gas_price() {
    let parent =
        parent_fixture::build_complete_parent(Some(&hex::decode("3a60005260206000f3").unwrap()));
    let budget = development_state_budget();
    let limits = SimulationLimits {
        maximum_gas: 30_000_000,
        maximum_calldata_bytes: 131_072,
        maximum_memory_bytes: 16 * 1_048_576,
        maximum_estimation_attempts: 32,
        maximum_access_list_entries: 256,
        maximum_access_list_storage_keys: 1_024,
    };
    let context = SimulationContext {
        state: &parent.state,
        version: &parent.target,
        header: &parent.block.header,
        state_budget: &budget,
        limits: &limits,
        reserved_clone_bytes: 16 * 1_048_576,
    };
    let mut request = SimulationRequest {
        from: support::sender(),
        to: Some(parent_fixture::CONTRACT),
        value: U256::ZERO,
        data: Bytes::new(),
        gas: Some(100_000),
        gas_price: None,
        max_fee_per_gas: Some(2_000_000_000),
        max_priority_fee_per_gas: None,
        transaction_type: None,
        access_list: None,
    };
    let result = simulate_complete_state(&context, &request).unwrap();
    assert_eq!(
        U256::from_be_slice(result.execution.output().unwrap()),
        U256::from(1_000_000_000_u64)
    );
    request.max_priority_fee_per_gas = Some(500_000_000);
    let result = simulate_complete_state(&context, &request).unwrap();
    assert_eq!(
        U256::from_be_slice(result.execution.output().unwrap()),
        U256::from(1_500_000_000_u64)
    );
    request.max_fee_per_gas = Some(1);
    request.max_priority_fee_per_gas = Some(0);
    assert!(matches!(
        simulate_complete_state(&context, &request),
        Err(SimulationError::InvalidTransaction(_))
    ));
}
