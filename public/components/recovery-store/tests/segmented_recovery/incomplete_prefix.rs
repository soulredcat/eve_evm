// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_storage::records::{
    opaque_record_cursor, read_opaque_record,
    segmented::recovery::{
        SegmentedRecoveryStep, begin_segmented_recovery, scan_next_segmented_recovery,
        segmented_recovery_bundle_bytes, segmented_recovery_observation,
    },
};

#[test]
fn canonical_contiguous_suffix_resumes_across_actual_reopen_and_live_append() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("records");
    let mut repository = fixtures::open(&path);
    let parent = fixtures::anchor(&repository);
    let body = vec![0x31; 192];
    let identity = fixtures::identity(parent, &body);
    let mut references = fixtures::append_segments(&mut repository, identity, &body, &[0]);
    drop(repository);
    let mut repository = fixtures::open(&path);
    let charge = fixtures::charge(&repository);
    let mut scan =
        begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &repository, charge).unwrap(),
        SegmentedRecoveryStep::Incomplete { rows_scanned: 1 }
    ));
    assert_eq!(segmented_recovery_observation(&scan).received_segments, 1);
    references.extend(fixtures::append_segments(
        &mut repository,
        identity,
        &body,
        &[1, 2],
    ));
    fixtures::append(
        &mut repository,
        fixtures::marker(identity, references, [8; 32]),
    );
    let SegmentedRecoveryStep::Complete(bundle) =
        scan_next_segmented_recovery(&mut scan, &repository, charge).unwrap()
    else {
        panic!("resumed bundle required")
    };
    assert_eq!(segmented_recovery_bundle_bytes(&bundle), body);
}

#[test]
fn explicit_index_zero_restart_keeps_incomplete_orphan_rows_and_discards_only_ram_candidate() {
    let temporary = tempfile::tempdir().unwrap();
    let mut repository = fixtures::open(&temporary.path().join("records"));
    let parent = fixtures::anchor(&repository);
    let orphan = [0x31; 192];
    let orphan_identity = fixtures::identity(parent, &orphan);
    fixtures::append_segments(&mut repository, orphan_identity, &orphan, &[0, 1]);
    let wanted = [0x41; 128];
    fixtures::append_bundle(&mut repository, parent, &wanted, [8; 32]);
    let head = opaque_record_cursor(&repository).unwrap();
    let charge = fixtures::charge(&repository);
    let mut scan =
        begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
    let SegmentedRecoveryStep::Complete(bundle) =
        scan_next_segmented_recovery(&mut scan, &repository, charge).unwrap()
    else {
        panic!("explicit new-start bundle required")
    };
    assert_eq!(segmented_recovery_bundle_bytes(&bundle), wanted);
    assert_eq!(segmented_recovery_observation(&scan).orphan_segments, 2);
    assert!(read_opaque_record(&repository, 1).unwrap().is_some());
    assert!(read_opaque_record(&repository, 2).unwrap().is_some());
    assert_eq!(opaque_record_cursor(&repository).unwrap(), head);
}

#[test]
fn each_scan_step_reads_at_most_seven_rows_without_accumulating_history() {
    let temporary = tempfile::tempdir().unwrap();
    let mut repository = fixtures::open(&temporary.path().join("records"));
    let parent = fixtures::anchor(&repository);
    let body = [0x31; 128];
    let identity = fixtures::identity(parent, &body);
    for _ in 0..8 {
        fixtures::append_segments(&mut repository, identity, &body, &[0]);
    }
    let charge = fixtures::charge(&repository);
    let mut scan =
        begin_segmented_recovery(&repository, parent, &fixtures::limits(), charge).unwrap();
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &repository, charge).unwrap(),
        SegmentedRecoveryStep::Progress { rows_scanned: 7 }
    ));
    assert_eq!(
        segmented_recovery_observation(&scan)
            .physical_cursor
            .sequence,
        7
    );
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &repository, charge).unwrap(),
        SegmentedRecoveryStep::Incomplete { rows_scanned: 1 }
    ));
    assert_eq!(segmented_recovery_observation(&scan).orphan_segments, 7);
    assert_eq!(segmented_recovery_observation(&scan).received_segments, 1);
}
