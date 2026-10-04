// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_storage::records::{
    opaque_record_cursor, open_opaque_record_repository,
    segmented::recovery::{
        SegmentedRecoveryError, begin_segmented_recovery, required_segmented_recovery_reservation,
        scan_next_segmented_recovery, segmented_recovery_observation,
    },
};

#[test]
fn exact_reservation_passes_one_byte_less_rejects_before_owned_buffer_or_read() {
    let temporary = tempfile::tempdir().unwrap();
    let repository = fixtures::open(&temporary.path().join("records"));
    let parent = fixtures::anchor(&repository);
    let charge = fixtures::charge(&repository);
    assert!(matches!(
        begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge - 1),
        Err(SegmentedRecoveryError::ReservationTooSmall)
    ));
    let mut scan =
        begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &repository, charge - 1),
        Err(SegmentedRecoveryError::ReservationTooSmall)
    ));
    assert_eq!(
        segmented_recovery_observation(&scan).physical_cursor,
        parent.cursor
    );
    assert_eq!(opaque_record_cursor(&repository).unwrap(), parent.cursor);
    let mut overflowing = fixtures::budget();
    overflowing.maximum_read_bytes = usize::MAX;
    assert_eq!(
        required_segmented_recovery_reservation(&overflowing, &fixtures::limits()),
        Err(SegmentedRecoveryError::ArithmeticOverflow)
    );
}

#[test]
fn wrong_namespace_or_anchor_is_rejected_without_changing_verified_parent() {
    let temporary = tempfile::tempdir().unwrap();
    let repository = fixtures::open(&temporary.path().join("records"));
    let parent = fixtures::anchor(&repository);
    let charge = fixtures::charge(&repository);
    let mut scan =
        begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
    let mut other_identity = fixtures::namespace();
    other_identity.domain[0] ^= 1;
    let foreign = open_opaque_record_repository(
        &temporary.path().join("foreign"),
        other_identity,
        fixtures::budget(),
    )
    .unwrap();
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &foreign, charge),
        Err(SegmentedRecoveryError::ForeignRepository)
    ));
    let mut wrong_anchor = parent;
    wrong_anchor.cursor.content_hash[0] ^= 1;
    assert!(matches!(
        begin_segmented_recovery(&repository, wrong_anchor, &fixtures::limits(), charge),
        Err(SegmentedRecoveryError::InvalidAnchor)
    ));
    assert_eq!(segmented_recovery_observation(&scan).logical_anchor, parent);
}
