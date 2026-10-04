// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    SegmentedRecoveryError, begin_segmented_recovery, required_segmented_recovery_reservation,
    scan_next_segmented_recovery, segmented_recovery_observation,
};
use crate::records::{
    OpaqueRecordIdentity, compare_and_append_opaque_records, development_opaque_record_budget,
    opaque_record_bootstrap_cursor, open_opaque_record_repository, read_opaque_record,
    segmented::{SegmentedCodecLimits, SegmentedRecoveryAnchor},
};

#[test]
fn actual_retained_row_failure_fences_head_and_latches_scanner_before_progress() {
    let temporary = tempfile::tempdir().unwrap();
    let budget = development_opaque_record_budget();
    let mut repository = open_opaque_record_repository(
        &temporary.path().join("records"),
        OpaqueRecordIdentity {
            genesis_hash: [1; 32],
            owner: [2; 32],
            domain: [3; 32],
        },
        budget,
    )
    .unwrap();
    let limits = SegmentedCodecLimits {
        maximum_logical_bytes: 384,
        maximum_chunk_bytes: 64,
        maximum_segments: 6,
        maximum_payload_bytes: 465,
    };
    let anchor = SegmentedRecoveryAnchor {
        height: 0,
        cursor: opaque_record_bootstrap_cursor(&repository),
        state_binding: [7; 32],
    };
    let reserved = required_segmented_recovery_reservation(&budget, &limits).unwrap();
    let mut scan = begin_segmented_recovery(&repository, anchor, &limits, reserved).unwrap();
    compare_and_append_opaque_records(&mut repository, anchor.cursor, &[vec![0x11]]).unwrap();
    // Deliberate corruption of this disposable test namespace, not a recovery action.
    let key = [b"eve/opaque/v1/record/".as_slice(), &1_u64.to_be_bytes()].concat();
    let mut writes = rocksdb::WriteOptions::default();
    writes.set_sync(true);
    repository.database.delete_opt(key, &writes).unwrap();
    assert!(read_opaque_record(&repository, 1).is_err());
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &repository, reserved),
        Err(SegmentedRecoveryError::StorageFailed)
    ));
    assert!(segmented_recovery_observation(&scan).failed);
    assert_eq!(segmented_recovery_observation(&scan).logical_anchor, anchor);
    assert!(matches!(
        scan_next_segmented_recovery(&mut scan, &repository, reserved),
        Err(SegmentedRecoveryError::FailedScanner)
    ));
}
