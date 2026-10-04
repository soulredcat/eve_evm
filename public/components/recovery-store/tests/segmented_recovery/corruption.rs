// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_storage::records::{
    opaque_record_cursor, read_opaque_record,
    segmented::recovery::{
        SegmentedRecoveryError, begin_segmented_recovery, scan_next_segmented_recovery,
        segmented_recovery_observation,
    },
};

#[test]
fn rehashed_chunks_and_marker_cannot_substitute_for_full_logical_body_identity() {
    let temporary = tempfile::tempdir().unwrap();
    let mut repository = fixtures::open(&temporary.path().join("records"));
    let parent = fixtures::anchor(&repository);
    let body = [0x31; 128];
    let mut identity = fixtures::identity(parent, &body);
    identity.logical_id = [0x41; 32];
    let references = fixtures::append_segments(&mut repository, identity, &body, &[0, 1]);
    let marker = fixtures::append(
        &mut repository,
        fixtures::marker(identity, references, [8; 32]),
    );
    let charge = fixtures::charge(&repository);
    let mut scan =
        begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &repository, charge),
        Err(SegmentedRecoveryError::LogicalHashMismatch)
    ));
    assert_eq!(segmented_recovery_observation(&scan).logical_anchor, parent);
    assert_eq!(opaque_record_cursor(&repository).unwrap(), marker);
    let supplied = eve_storage::records::segmented::SegmentedRecoveryAnchor {
        height: 1,
        cursor: marker,
        state_binding: [8; 32],
    };
    assert!(matches!(
        begin_segmented_recovery(&repository, supplied, &fixtures::limits(), charge),
        Err(SegmentedRecoveryError::LogicalHashMismatch)
    ));
}

#[test]
fn forged_actual_reference_hash_and_marker_position_fail_without_deleting_rows() {
    for misplaced in [false, true] {
        let temporary = tempfile::tempdir().unwrap();
        let mut repository = fixtures::open(&temporary.path().join("records"));
        let parent = fixtures::anchor(&repository);
        let body = [0x31; 128];
        let identity = fixtures::identity(parent, &body);
        let mut references = fixtures::append_segments(&mut repository, identity, &body, &[0, 1]);
        if misplaced {
            fixtures::append_segments(&mut repository, identity, &body, &[0]);
        } else {
            references[0].content_hash = [0x55; 32];
        }
        fixtures::append(
            &mut repository,
            fixtures::marker(identity, references, [8; 32]),
        );
        let head = opaque_record_cursor(&repository).unwrap();
        let charge = fixtures::charge(&repository);
        let mut scan =
            begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
        assert!(matches!(
            scan_next_segmented_recovery(&mut scan, &repository, charge),
            Err(SegmentedRecoveryError::InvalidMarker)
        ));
        assert_eq!(opaque_record_cursor(&repository).unwrap(), head);
        assert!(read_opaque_record(&repository, 1).unwrap().is_some());
    }
}

#[test]
fn malformed_payload_or_noncontiguous_suffix_latches_failure_instead_of_resetting() {
    for malformed in [false, true] {
        let temporary = tempfile::tempdir().unwrap();
        let mut repository = fixtures::open(&temporary.path().join("records"));
        let parent = fixtures::anchor(&repository);
        let body = [0x31; 192];
        let identity = fixtures::identity(parent, &body);
        fixtures::append_segments(&mut repository, identity, &body, &[0]);
        if malformed {
            let mut payload = fixtures::segment(identity, &body, 1);
            *payload.last_mut().unwrap() ^= 1;
            fixtures::append(&mut repository, payload);
        } else {
            fixtures::append_segments(&mut repository, identity, &body, &[2]);
        }
        let charge = fixtures::charge(&repository);
        let mut scan =
            begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
        assert!(scan_next_segmented_recovery(&mut scan, &repository, charge).is_err());
        assert!(segmented_recovery_observation(&scan).failed);
        fixtures::append_segments(&mut repository, identity, &body, &[0]);
        assert!(matches!(
            scan_next_segmented_recovery(&mut scan, &repository, charge),
            Err(SegmentedRecoveryError::FailedScanner)
        ));
        assert_eq!(segmented_recovery_observation(&scan).logical_anchor, parent);
        assert_eq!(opaque_record_cursor(&repository).unwrap().sequence, 3);
    }
}
