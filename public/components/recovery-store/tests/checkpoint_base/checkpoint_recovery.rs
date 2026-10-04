// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    checkpoint_recovery_support::{charge, codec, suffix},
    fixtures,
};
use eve_storage::records::segmented::{SegmentedRecoveryAnchor, checkpoints::*, recovery::*};

#[test]
fn explicit_actual_checkpoint_base_recovers_only_its_complete_suffix_after_reopen() {
    let directory = tempfile::tempdir().unwrap();
    let mut repository = fixtures::open(directory.path());
    let genesis = fixtures::bootstrap(&repository);
    let target = fixtures::prepared(4);
    let metadata = fixtures::metadata(genesis, genesis.cursor, 4);
    let base_cursor = fixtures::append(&mut repository, fixtures::encode(metadata, &target));
    let anchor = SegmentedRecoveryAnchor {
        height: 4,
        cursor: base_cursor,
        state_binding: metadata.target_state_binding,
    };
    let body = [0x43; 128];
    let next = suffix(&mut repository, anchor, &body);
    drop(repository);
    let repository = fixtures::open(directory.path());
    let membership = read_checkpoint_base_membership(
        &repository,
        base_cursor,
        metadata,
        &target,
        &fixtures::limits(),
        fixtures::membership_charge(&repository),
    )
    .unwrap();
    assert!(begin_segmented_recovery(&repository, anchor, &codec(), charge(&repository)).is_err());
    let mut scan = begin_segmented_checkpoint_recovery(
        &repository,
        &membership,
        &target,
        &codec(),
        charge(&repository),
    )
    .unwrap();
    assert_eq!(segmented_recovery_observation(&scan).logical_anchor, anchor);
    assert_eq!(
        segmented_recovery_observation(&scan).physical_cursor,
        base_cursor
    );
    let SegmentedRecoveryStep::Complete(bundle) =
        scan_next_segmented_recovery(&mut scan, &repository, charge(&repository)).unwrap()
    else {
        panic!("complete suffix required")
    };
    assert_eq!(segmented_recovery_bundle_bytes(&bundle), body);
    assert_eq!(segmented_recovery_bundle_anchor(&bundle), next);
    assert_eq!(segmented_recovery_bundle_capacity(&bundle), 128);
    assert_eq!(segmented_recovery_observation(&scan).logical_anchor, anchor);
    // Test-only integrity confirmation; no production execution/finality claim.
    accept_segmented_recovery(&mut scan, bundle, next, charge(&repository)).unwrap();
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &repository, charge(&repository)).unwrap(),
        SegmentedRecoveryStep::Exhausted
    ));
}

#[test]
fn checkpoint_initializer_rereads_same_base_and_refuses_missing_foreign_or_wrong_target() {
    let directory = tempfile::tempdir().unwrap();
    let mut repository = fixtures::open(directory.path());
    let parent = fixtures::bootstrap(&repository);
    let target = fixtures::prepared(4);
    let expected = fixtures::metadata(parent, parent.cursor, 4);
    let cursor = fixtures::append(&mut repository, fixtures::encode(expected, &target));
    let membership = read_checkpoint_base_membership(
        &repository,
        cursor,
        expected,
        &target,
        &fixtures::limits(),
        fixtures::membership_charge(&repository),
    )
    .unwrap();
    assert!(matches!(
        begin_segmented_checkpoint_recovery(
            &repository,
            &membership,
            &fixtures::prepared(5),
            &codec(),
            charge(&repository)
        ),
        Err(SegmentedRecoveryError::InvalidAnchor)
    ));
    assert!(matches!(
        begin_segmented_checkpoint_recovery(
            &repository,
            &membership,
            &target,
            &codec(),
            charge(&repository) - 1
        ),
        Err(SegmentedRecoveryError::ReservationTooSmall)
    ));
    let missing_directory = tempfile::tempdir().unwrap();
    let missing = fixtures::open(missing_directory.path());
    assert!(matches!(
        begin_segmented_checkpoint_recovery(
            &missing,
            &membership,
            &target,
            &codec(),
            charge(&missing)
        ),
        Err(SegmentedRecoveryError::InvalidAnchor)
    ));
    let foreign_directory = tempfile::tempdir().unwrap();
    let mut foreign = fixtures::open(foreign_directory.path());
    let mut changed = expected;
    changed.proof_manifest_id = [0x31; 32];
    fixtures::append(&mut foreign, fixtures::encode(changed, &target));
    assert!(matches!(
        begin_segmented_checkpoint_recovery(
            &foreign,
            &membership,
            &target,
            &codec(),
            charge(&foreign)
        ),
        Err(SegmentedRecoveryError::InvalidAnchor)
    ));
    let unchanged = checkpoint_base_membership_view(&membership).metadata;
    assert_eq!(unchanged.proof_manifest_id, expected.proof_manifest_id);
    assert_eq!(unchanged.proof_root, expected.proof_root);
    assert!(
        begin_segmented_checkpoint_recovery(
            &repository,
            &membership,
            &target,
            &codec(),
            charge(&repository)
        )
        .is_ok()
    );
}
