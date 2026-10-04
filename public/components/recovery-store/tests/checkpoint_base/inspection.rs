// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::*;
use eve_state::preflight_state_version;
use eve_storage::records::segmented::checkpoints::*;

#[test]
fn inspection_borrows_canonical_target_without_a_prepared_or_authenticated_state() {
    let parent = inert_parent();
    let target = prepared(4);
    let expected = metadata(parent, parent.cursor, 4);
    let bytes = encode(expected, &target);
    let inspected = inspect_checkpoint_base(&bytes, &limits()).unwrap();
    let view = checkpoint_base_inspection_view(&inspected);
    assert_eq!(view.metadata, expected);
    assert_eq!(
        view.target_version_bytes,
        prepared_checkpoint_base_target_bytes(&target)
    );
    assert_eq!(
        checkpoint_base_inspection_bytes(&inspected).as_ptr(),
        bytes.as_ptr()
    );
    assert_eq!(view.target_version_bytes.as_ptr(), bytes[364..].as_ptr());
    assert_eq!(
        preflight_state_version(view.target_version_bytes).unwrap(),
        version(4).identity.network_name.len()
    );
}

#[test]
fn inspection_checks_canonical_target_framing_and_hash_before_owned_target_decode() {
    let parent = inert_parent();
    let target = prepared(4);
    let bytes = encode(metadata(parent, parent.cursor, 4), &target);
    let mut malformed = bytes.clone();
    malformed[364] = 0;
    rehash(&mut malformed);
    assert!(matches!(
        inspect_checkpoint_base(&malformed, &limits()),
        Err(CheckpointBaseError::InvalidTarget)
    ));
    let mut corrupted = bytes.clone();
    corrupted[220] ^= 1;
    assert!(matches!(
        inspect_checkpoint_base(&corrupted, &limits()),
        Err(CheckpointBaseError::HashMismatch)
    ));
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(matches!(
        inspect_checkpoint_base(&trailing, &limits()),
        Err(CheckpointBaseError::MalformedEncoding)
    ));
    for length in [0, 363, bytes.len() - 1] {
        assert!(inspect_checkpoint_base(&bytes[..length], &limits()).is_err());
    }
}

#[test]
fn inspection_preserves_untrusted_metadata_but_does_not_bypass_expected_parent_policy() {
    let parent = inert_parent();
    let target = prepared(4);
    let mut bytes = encode(metadata(parent, parent.cursor, 4), &target);
    bytes[340..348].copy_from_slice(&8_u64.to_be_bytes());
    rehash(&mut bytes);
    let inspected = inspect_checkpoint_base(&bytes, &limits()).unwrap();
    assert_eq!(
        checkpoint_base_inspection_view(&inspected)
            .metadata
            .lookahead_height,
        8
    );
    assert!(matches!(
        preflight_checkpoint_base(&bytes, &target, parent.cursor, parent, &limits()),
        Err(CheckpointBaseError::InvalidMetadata)
    ));
    let mut wrong_parent = parent;
    wrong_parent.state_binding[0] ^= 1;
    assert!(matches!(
        preflight_checkpoint_base(&bytes, &target, parent.cursor, wrong_parent, &limits()),
        Err(CheckpointBaseError::ParentMismatch)
    ));
}
