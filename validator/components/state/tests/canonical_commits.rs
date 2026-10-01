// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;
use eve_state::{
    B256, Bytes, StateError, SystemValue, compute_commit_identity, decode_state_commit,
    development_state_budget, encode_state_commit, validate_complete_state, validate_state_version,
};

#[test]
fn whole_commit_codec_roundtrips_rejects_trailing_data_and_binds_history_outside_evm_root() {
    let first = support::with_slots();
    let budget = development_state_budget();
    let bytes = encode_state_commit(&first, &budget).unwrap();
    assert_eq!(decode_state_commit(&bytes, &budget).unwrap(), first);
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode_state_commit(&trailing, &budget).is_err());
    let mut corrupt = first.state.clone();
    corrupt
        .block_hashes
        .insert(0, eve_state::ExecutionBlockHash(B256::repeat_byte(9)));
    assert!(validate_state_version(&corrupt, &first.target, &budget).is_err());
    assert_eq!(
        eve_state::compute_evm_root(&corrupt.accounts),
        first.target.evm_root
    );
}

#[test]
fn missing_or_mismatched_bytecode_rejects_and_system_only_change_changes_app_commitment() {
    let parent = support::genesis();
    let hash = parent.state.accounts[&support::contract()].code_hash;
    let budget = development_state_budget();
    let mut missing = parent.state.clone();
    missing.codes.remove(&hash);
    assert_eq!(
        validate_complete_state(&missing, &budget),
        Err(StateError::MissingCode(hash))
    );
    missing
        .codes
        .insert(hash, Bytes::from_static(b"wrong-code"));
    assert_eq!(
        validate_complete_state(&missing, &budget),
        Err(StateError::CodeHashMismatch(hash))
    );
    let baseline = support::child(&parent, parent.state.clone());
    let mut changed = parent.state.clone();
    for record in changed.system.values_mut() {
        if let SystemValue::Fee { burned, .. } = &mut record.value {
            *burned += eve_state::U256::from(1);
        }
    }
    let changed = support::child(&parent, changed);
    assert_eq!(changed.target.evm_root, baseline.target.evm_root);
    assert_eq!(
        changed.target.execution_hash,
        baseline.target.execution_hash
    );
    assert_ne!(changed.target.system_root, baseline.target.system_root);
    assert_ne!(changed.target.application, baseline.target.application);
    assert_ne!(
        compute_commit_identity(&changed, &budget).unwrap(),
        compute_commit_identity(&baseline, &budget).unwrap()
    );
}
