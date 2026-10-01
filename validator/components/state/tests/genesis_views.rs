// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;
use eve_state::{
    Address, U256, capture_state_view, development_state_budget, initialize_development_state,
    read_account, read_code, read_storage,
};

#[test]
fn genesis_debits_bonds_without_minting_and_keeps_empty_fee_pools_absent() {
    let input = support::genesis_input();
    let commit = initialize_development_state(&input, &development_state_budget()).unwrap();
    let expected = input
        .accounts
        .iter()
        .fold(U256::ZERO, |sum, account| sum + account.funded_balance);
    let actual = commit
        .state
        .accounts
        .values()
        .fold(U256::ZERO, |sum, account| sum + account.balance);
    assert_eq!(actual, expected);
    let custody = "000000000000000000000000000000000000f100"
        .parse::<Address>()
        .unwrap();
    assert_eq!(
        commit.state.accounts[&custody].balance,
        input.economics.validator_self_bond * U256::from(4)
    );
    assert_eq!(
        commit.state.accounts[&input.accounts[0].address].balance,
        input.accounts[0].funded_balance - input.validators[0].self_bond
    );
    for pool in [
        "000000000000000000000000000000000000f101",
        "000000000000000000000000000000000000f102",
    ] {
        assert!(
            !commit
                .state
                .accounts
                .contains_key(&pool.parse::<Address>().unwrap())
        );
    }
    assert_eq!(commit.target.application, None);
}

#[test]
fn captured_views_do_not_mix_versions_and_missing_code_is_explicit() {
    let first = support::with_slots();
    let mut changed = first.state.clone();
    changed
        .accounts
        .get_mut(&support::contract())
        .unwrap()
        .storage
        .insert(U256::ZERO, U256::from(99));
    let second = support::child(&first, changed);
    let budget = development_state_budget();
    let old = capture_state_view(first.state, first.target, &budget).unwrap();
    let new = capture_state_view(second.state, second.target, &budget).unwrap();
    assert_eq!(
        read_storage(&old, support::contract(), U256::ZERO),
        U256::from(7)
    );
    assert_eq!(
        read_storage(&new, support::contract(), U256::ZERO),
        U256::from(99)
    );
    assert_eq!(
        read_storage(&new, support::contract(), U256::from(1)),
        U256::from(11)
    );
    assert!(read_account(&old, Address::repeat_byte(9)).is_none());
    assert!(read_code(&old, eve_state::B256::repeat_byte(9)).is_err());
}
