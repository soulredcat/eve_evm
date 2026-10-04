// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::*;
use eve_state::{decode_state_version, encode_state_version};
use eve_storage::records::segmented::checkpoints::*;

#[test]
fn canonical_target_and_fixed_metadata_round_trip_without_finality_authority() {
    let parent = inert_parent();
    let target = prepared(4);
    let metadata = metadata(parent, parent.cursor, 4);
    let bytes = encode(metadata, &target);
    let checked =
        preflight_checkpoint_base(&bytes, &target, parent.cursor, parent, &limits()).unwrap();
    let view = checkpoint_base_view(&checked);
    assert_eq!(view.metadata, metadata);
    assert_eq!(
        view.security_profile,
        version(4).identity.security_profile as u8
    );
    assert_eq!(
        view.target_version_bytes,
        encode_state_version(&version(4)).unwrap().as_ref()
    );
    assert_eq!(
        decode_state_version(view.target_version_bytes).unwrap(),
        version(4)
    );
    assert_eq!(checkpoint_base_bytes(&checked), bytes);
    assert_eq!(
        prepared_checkpoint_base_target_bytes(&target),
        view.target_version_bytes
    );
}

#[test]
fn truncated_trailing_unknown_schema_mode_and_bad_checksum_are_rejected() {
    let parent = inert_parent();
    let target = prepared(4);
    let bytes = encode(metadata(parent, parent.cursor, 4), &target);
    for length in [0, 21, 364, bytes.len() - 1] {
        assert!(
            preflight_checkpoint_base(&bytes[..length], &target, parent.cursor, parent, &limits())
                .is_err()
        );
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(matches!(
        preflight_checkpoint_base(&trailing, &target, parent.cursor, parent, &limits()),
        Err(CheckpointBaseError::MalformedEncoding)
    ));
    for (offset, expected) in [
        (23, CheckpointBaseError::UnsupportedVersion),
        (24, CheckpointBaseError::UnsupportedMode),
    ] {
        let mut changed = bytes.clone();
        changed[offset] = 7;
        rehash(&mut changed);
        assert!(
            matches!(preflight_checkpoint_base(&changed, &target, parent.cursor, parent, &limits()), Err(error) if error == expected)
        );
    }
    let mut changed = bytes;
    changed[220] ^= 1;
    assert!(matches!(
        preflight_checkpoint_base(&changed, &target, parent.cursor, parent, &limits()),
        Err(CheckpointBaseError::HashMismatch)
    ));
}

#[test]
fn rehashed_foreign_target_profile_and_parents_still_reject() {
    let parent = inert_parent();
    let target = prepared(4);
    let bytes = encode(metadata(parent, parent.cursor, 4), &target);
    let foreign = prepared(5);
    assert!(matches!(
        preflight_checkpoint_base(&bytes, &foreign, parent.cursor, parent, &limits()),
        Err(CheckpointBaseError::TargetMismatch)
    ));
    for offset in [25, 364] {
        let mut changed = bytes.clone();
        changed[offset] ^= 1;
        rehash(&mut changed);
        assert!(matches!(
            preflight_checkpoint_base(&changed, &target, parent.cursor, parent, &limits()),
            Err(CheckpointBaseError::TargetMismatch)
        ));
    }
    for offset in [36, 84, 116] {
        let mut changed = bytes.clone();
        changed[offset] ^= 1;
        rehash(&mut changed);
        assert!(matches!(
            preflight_checkpoint_base(&changed, &target, parent.cursor, parent, &limits()),
            Err(CheckpointBaseError::ParentMismatch)
        ));
    }
}

#[test]
fn bounded_version_length_and_exact_target_height_proof_ranges_reject() {
    let parent = inert_parent();
    let target = prepared(4);
    let bytes = encode(metadata(parent, parent.cursor, 4), &target);
    for length in [0_u16, 4_097, u16::MAX] {
        let mut changed = bytes.clone();
        changed[26..28].copy_from_slice(&length.to_be_bytes());
        assert!(matches!(
            preflight_checkpoint_base(&changed, &target, parent.cursor, parent, &limits()),
            Err(CheckpointBaseError::LimitExceeded)
        ));
    }
    for offset in [148, 316, 324, 332, 340, 348, 356] {
        let mut changed = bytes.clone();
        changed[offset..offset + 8].copy_from_slice(&10_u64.to_be_bytes());
        rehash(&mut changed);
        assert!(matches!(
            preflight_checkpoint_base(&changed, &target, parent.cursor, parent, &limits()),
            Err(CheckpointBaseError::InvalidMetadata)
        ));
    }
}

#[test]
fn zero_hashes_invalid_logical_anchor_and_height_overflow_reject() {
    let parent = inert_parent();
    let target = prepared(4);
    let original = metadata(parent, parent.cursor, 4);
    let mut changed = original;
    changed.proof_root = [0; 32];
    assert!(matches!(
        encode_checkpoint_base(changed, &target, &limits(), reservation()),
        Err(CheckpointBaseError::InvalidMetadata)
    ));
    changed = original;
    changed.previous_logical_anchor.height = 1;
    assert!(matches!(
        encode_checkpoint_base(changed, &target, &limits(), reservation()),
        Err(CheckpointBaseError::InvalidMetadata)
    ));
    changed = original;
    changed.previous_opaque_cursor.sequence = u64::MAX;
    assert!(matches!(
        encode_checkpoint_base(changed, &target, &limits(), reservation()),
        Err(CheckpointBaseError::ArithmeticOverflow)
    ));
    let maximum_target = prepared(u64::MAX);
    let maximum = metadata(parent, parent.cursor, u64::MAX);
    assert!(matches!(
        encode_checkpoint_base(maximum, &maximum_target, &limits(), reservation()),
        Err(CheckpointBaseError::ArithmeticOverflow)
    ));
}
