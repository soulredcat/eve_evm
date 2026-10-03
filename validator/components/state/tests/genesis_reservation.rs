// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;
use eve_state::{
    StateError, development_state_budget, estimate_genesis_initialization_reservation,
};

#[test]
fn actual_small_genesis_fits_without_charging_default_state_maxima() {
    let genesis = support::genesis_input();
    let budget = development_state_budget();
    let actual = estimate_genesis_initialization_reservation(&genesis, &budget).unwrap();
    assert!(actual < 256 * 1_048_576);
    let mut larger_final_limits = budget;
    larger_final_limits.maximum_accounts *= 2;
    larger_final_limits.maximum_storage_slots *= 2;
    assert_eq!(
        estimate_genesis_initialization_reservation(&genesis, &larger_final_limits).unwrap(),
        actual
    );
}

#[test]
fn invalid_genesis_and_actual_resource_limits_reject_before_state_initialization() {
    let mut genesis = support::genesis_input();
    let budget = development_state_budget();
    genesis.network_name = "x".repeat(65_536);
    assert!(matches!(
        estimate_genesis_initialization_reservation(&genesis, &budget),
        Err(StateError::Genesis(_))
    ));
    let genesis = support::genesis_input();
    let mut restricted = budget;
    restricted.maximum_accounts = genesis.accounts.len();
    assert_eq!(
        estimate_genesis_initialization_reservation(&genesis, &restricted),
        Err(StateError::BudgetExceeded)
    );
    restricted = budget;
    restricted.maximum_code_bytes = 1;
    assert_eq!(
        estimate_genesis_initialization_reservation(&genesis, &restricted),
        Err(StateError::BudgetExceeded)
    );
}

#[test]
fn actual_code_growth_increases_genesis_charge_without_changing_its_input() {
    let mut genesis = support::genesis_input();
    let budget = development_state_budget();
    let before = estimate_genesis_initialization_reservation(&genesis, &budget).unwrap();
    let code = genesis.accounts.last_mut().unwrap();
    code.code = eve_state::Bytes::from(vec![0_u8; 1_024]);
    let after = estimate_genesis_initialization_reservation(&genesis, &budget).unwrap();
    assert!(after > before);
    assert_eq!(genesis.accounts.last().unwrap().code.len(), 1_024);
}
