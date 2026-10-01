// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;
use eve_state::{
    B256, StateError, decode_state_version, development_state_budget, encode_state_version,
    validate_retained_block,
};

#[test]
fn narrow_retained_block_and_version_codec_validate_without_complete_state() {
    let parent = support::genesis();
    let child = support::child(&parent, parent.state.clone());
    let budget = development_state_budget();
    validate_retained_block(&child.target, Some(&parent.target), &child.block, &budget).unwrap();
    let bytes = encode_state_version(&child.target).unwrap();
    assert_eq!(decode_state_version(&bytes).unwrap(), child.target);
    let mut trailing = bytes.to_vec();
    trailing.push(0);
    assert_eq!(
        decode_state_version(&trailing),
        Err(StateError::NonCanonicalEncoding)
    );
    assert_eq!(
        decode_state_version(&vec![0; 4_097]),
        Err(StateError::BudgetExceeded)
    );
}

#[test]
fn retained_metadata_rejects_hash_roots_app_domain_and_parent_substitution() {
    let parent = support::genesis();
    let child = support::child(&parent, parent.state.clone());
    let budget = development_state_budget();
    let mut block = child.block.clone();
    block.header.state_root = B256::repeat_byte(1);
    assert!(validate_retained_block(&child.target, Some(&parent.target), &block, &budget).is_err());
    let mut version = child.target.clone();
    version.application = None;
    assert_eq!(
        encode_state_version(&version),
        Err(StateError::VersionMismatch)
    );
    let mut wrong_parent = parent.target.clone();
    wrong_parent.height = 8;
    assert!(
        validate_retained_block(&child.target, Some(&wrong_parent), &child.block, &budget).is_err()
    );
    block = child.block.clone();
    block.header.receipts_root = B256::repeat_byte(1);
    assert!(validate_retained_block(&child.target, Some(&parent.target), &block, &budget).is_err());
}
