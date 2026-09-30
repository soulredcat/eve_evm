use crate::support;

use alloy_primitives::{Address, B256, Bytes, U256};
use eve_state::{
    StateError, development_state_budget, validate_complete_state, validate_state_commit,
    validate_state_version,
};
use support::{commits::structural_genesis, fixtures::complete_state};

#[test]
fn ts01_referenced_code_is_checked_instead_of_silently_becoming_empty() {
    let state = complete_state("two_slot_contract");
    let hash = state.accounts[&Address::repeat_byte(0x11)].code_hash;
    let mut missing = state.clone();
    missing.codes.remove(&hash);
    assert_eq!(
        validate_complete_state(&missing, &development_state_budget()),
        Err(StateError::MissingCode(hash))
    );
    let mut corrupt = state;
    corrupt.codes.insert(hash, Bytes::from_static(&[0x00]));
    assert_eq!(
        validate_complete_state(&corrupt, &development_state_budget()),
        Err(StateError::CodeHashMismatch(hash))
    );
}

#[test]
fn ts01_incomplete_materialization_cannot_match_a_complete_version() {
    let commit = structural_genesis("two_slot_contract");
    let mut incomplete = commit.state.clone();
    incomplete.accounts.remove(&Address::repeat_byte(0x11));
    assert_eq!(
        validate_state_version(&incomplete, &commit.target, &development_state_budget()),
        Err(StateError::VersionMismatch)
    );
    validate_state_version(&commit.state, &commit.target, &development_state_budget()).unwrap();
}

#[test]
fn ts03_zero_slots_are_not_an_alternate_canonical_state_encoding() {
    let mut state = complete_state("two_slot_contract");
    state
        .accounts
        .get_mut(&Address::repeat_byte(0x11))
        .unwrap()
        .storage
        .insert(U256::from(9), U256::ZERO);
    assert_eq!(
        validate_complete_state(&state, &development_state_budget()),
        Err(StateError::NonCanonicalStorage)
    );
}

#[test]
fn ts02_complete_marker_is_bound_to_header_height_hash_and_network() {
    let original = structural_genesis("two_slot_contract");
    let mut hash = original.clone();
    hash.target.execution_hash.0 = B256::repeat_byte(0x44);
    assert!(validate_state_commit(&hash, &development_state_budget()).is_err());
    let mut height = original.clone();
    height.target.height = 1;
    assert!(validate_state_commit(&height, &development_state_budget()).is_err());
    let mut network = original;
    network.target.identity.genesis.0 = B256::repeat_byte(0x55);
    assert!(validate_state_commit(&network, &development_state_budget()).is_err());
}

#[test]
fn ts08_system_ledger_mutation_changes_application_binding_without_changing_evm_root() {
    let (_, original) = support::execution::executed_commit();
    let mut state = original.state.clone();
    let record = state.system.values_mut().next().unwrap();
    let eve_state::SystemValue::Fee { burned, .. } = &mut record.value else {
        panic!("expected literal fee ledger");
    };
    *burned += U256::from(1);
    let changed = eve_state::build_state_commit(
        original.parent.clone(),
        state,
        original.block.clone(),
        &development_state_budget(),
    )
    .unwrap();
    assert_eq!(changed.target.evm_root, original.target.evm_root);
    assert_eq!(
        changed.target.execution_hash,
        original.target.execution_hash
    );
    assert_ne!(changed.target.system_root, original.target.system_root);
    assert_ne!(changed.target.application, original.target.application);
    assert_ne!(
        changed.target.content_digest,
        original.target.content_digest
    );
}
