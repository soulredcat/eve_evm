use crate::support;

use alloy_primitives::{Address, B256, U256};
use eve_state::{
    compute_evm_root, compute_system_root, development_state_budget, validate_complete_state,
};
use support::fixtures::{complete_state, expected_root, fixture};

#[test]
fn ts01_independent_ethereumjs_account_storage_and_system_vectors_match() {
    for name in [
        "two_slot_contract",
        "updated_slot_preserves_other",
        "zero_slot_is_deleted",
        "deleted_account",
        "recreated_account",
        "execution_parent",
        "execution_post",
    ] {
        let input = fixture(name);
        let state = complete_state(name);
        validate_complete_state(&state, &development_state_budget()).unwrap();
        assert_eq!(
            compute_evm_root(&state.accounts).0,
            expected_root(&input, "evm_root"),
            "{name}"
        );
        assert_eq!(
            compute_system_root(&state.system).unwrap().0,
            expected_root(&input, "system_root"),
            "{name}"
        );
    }
}

#[test]
fn ts01_zero_storage_and_explicit_account_presence_have_distinct_meanings() {
    let deleted = complete_state("deleted_account");
    let recreated = complete_state("recreated_account");
    assert_eq!(
        compute_evm_root(&deleted.accounts).0,
        "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
            .parse::<B256>()
            .unwrap()
    );
    assert_ne!(
        compute_evm_root(&deleted.accounts),
        compute_evm_root(&recreated.accounts)
    );
    let state = complete_state("zero_slot_is_deleted");
    let account = state.accounts.get(&Address::repeat_byte(0x11)).unwrap();
    assert!(!account.storage.contains_key(&U256::ZERO));
    assert_eq!(account.storage.get(&U256::from(1)), Some(&U256::from(11)));
}

#[test]
fn ts03_literal_system_leaf_rejects_namespace_substitution() {
    let mut state = complete_state("two_slot_contract");
    let expected = compute_system_root(&state.system).unwrap();
    let original = state.system.values().next().unwrap().clone();
    state.system.values_mut().next().unwrap().logical_key = b"another-pool".as_slice().into();
    assert!(compute_system_root(&state.system).is_err());
    let original_key = *state.system.keys().next().unwrap();
    state.system.insert(original_key, original);
    assert_eq!(compute_system_root(&state.system).unwrap(), expected);
}
