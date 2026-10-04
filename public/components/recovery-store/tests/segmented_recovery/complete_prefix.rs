// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_storage::records::{
    opaque_record_cursor,
    segmented::recovery::{
        SegmentedRecoveryError, SegmentedRecoveryStep, accept_segmented_recovery,
        begin_segmented_recovery, scan_next_segmented_recovery, segmented_recovery_bundle_anchor,
        segmented_recovery_bundle_bytes, segmented_recovery_bundle_capacity,
        segmented_recovery_bundle_references, segmented_recovery_observation,
    },
};

#[test]
fn actual_six_segment_sync_reopen_recovers_complete_local_body_without_auto_acceptance() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("records");
    let mut repository = fixtures::open(&path);
    let parent = fixtures::anchor(&repository);
    let body = vec![0x31; 384];
    let target = fixtures::append_bundle(&mut repository, parent, &body, [8; 32]);
    drop(repository);
    let repository = fixtures::open(&path);
    let charge = fixtures::charge(&repository);
    let mut scan =
        begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
    let SegmentedRecoveryStep::Complete(bundle) =
        scan_next_segmented_recovery(&mut scan, &repository, charge).unwrap()
    else {
        panic!("complete seven-row bundle required")
    };
    assert_eq!(segmented_recovery_bundle_bytes(&bundle), body);
    assert_eq!(segmented_recovery_bundle_capacity(&bundle), 384);
    assert_eq!(segmented_recovery_bundle_references(&bundle).len(), 6);
    assert_eq!(segmented_recovery_bundle_anchor(&bundle), target);
    let observation = segmented_recovery_observation(&scan);
    assert_eq!(observation.logical_anchor, parent);
    assert_eq!(observation.physical_cursor, target.cursor);
    assert!(observation.awaiting_confirmation);
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &repository, charge),
        Err(SegmentedRecoveryError::ConfirmationRequired)
    ));
    // Test-only local confirmation; no native/EVM verification claim is made.
    accept_segmented_recovery(&mut scan, bundle, target, charge).unwrap();
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &repository, charge).unwrap(),
        SegmentedRecoveryStep::Exhausted
    ));
    assert_eq!(opaque_record_cursor(&repository).unwrap(), target.cursor);
}

#[test]
fn nonzero_verified_anchor_rechecks_actual_marker_refs_and_full_body_before_next_height() {
    let temporary = tempfile::tempdir().unwrap();
    let mut repository = fixtures::open(&temporary.path().join("records"));
    let genesis = fixtures::anchor(&repository);
    let parent = fixtures::append_bundle(&mut repository, genesis, &[0x31; 384], [8; 32]);
    let target = fixtures::append_bundle(&mut repository, parent, &[0x41; 128], [9; 32]);
    let charge = fixtures::charge(&repository);
    let mut scan =
        begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
    let SegmentedRecoveryStep::Complete(bundle) =
        scan_next_segmented_recovery(&mut scan, &repository, charge).unwrap()
    else {
        panic!("second complete bundle required")
    };
    assert_eq!(segmented_recovery_bundle_bytes(&bundle), &[0x41; 128]);
    assert_eq!(segmented_recovery_bundle_anchor(&bundle), target);
    assert_eq!(segmented_recovery_observation(&scan).logical_anchor, parent);
}

#[test]
fn rejected_confirmation_returns_owned_body_and_same_vec_is_reused_after_acceptance() {
    let temporary = tempfile::tempdir().unwrap();
    let mut repository = fixtures::open(&temporary.path().join("records"));
    let parent = fixtures::anchor(&repository);
    let first = fixtures::append_bundle(&mut repository, parent, &[0x31; 128], [8; 32]);
    let second = fixtures::append_bundle(&mut repository, first, &[0x41; 192], [9; 32]);
    let charge = fixtures::charge(&repository);
    let mut scan =
        begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
    let SegmentedRecoveryStep::Complete(bundle) =
        scan_next_segmented_recovery(&mut scan, &repository, charge).unwrap()
    else {
        panic!("first bundle required")
    };
    let pointer = segmented_recovery_bundle_bytes(&bundle).as_ptr();
    let mut wrong = first;
    wrong.state_binding[0] ^= 1;
    let rejected = accept_segmented_recovery(&mut scan, bundle, wrong, charge).unwrap_err();
    assert_eq!(rejected.error, SegmentedRecoveryError::WrongConfirmation);
    assert_eq!(segmented_recovery_observation(&scan).logical_anchor, parent);
    assert_eq!(
        segmented_recovery_bundle_bytes(&rejected.bundle).as_ptr(),
        pointer
    );
    accept_segmented_recovery(&mut scan, rejected.bundle, first, charge).unwrap();
    let SegmentedRecoveryStep::Complete(bundle) =
        scan_next_segmented_recovery(&mut scan, &repository, charge).unwrap()
    else {
        panic!("second bundle required")
    };
    assert_eq!(segmented_recovery_bundle_bytes(&bundle).as_ptr(), pointer);
    assert_eq!(segmented_recovery_bundle_anchor(&bundle), second);
    assert_eq!(segmented_recovery_bundle_bytes(&bundle), &[0x41; 192]);
}
