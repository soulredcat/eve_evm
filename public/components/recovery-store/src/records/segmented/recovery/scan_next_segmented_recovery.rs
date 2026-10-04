// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    append_segmented_recovery_candidate::append_segmented_recovery_candidate,
    complete_segmented_recovery_candidate::complete_segmented_recovery_candidate,
    required_segmented_recovery_reservation::required_segmented_recovery_reservation,
    types::{SegmentedRecoveryError, SegmentedRecoveryScan, SegmentedRecoveryStep},
};
use crate::records::{
    OpaqueRecordCursor, OpaqueRecordRepository, opaque_record_budget, opaque_record_cursor,
    opaque_record_identity, read_opaque_record,
    segmented::{
        SEGMENTED_MAX_REFERENCES, preflight_segmented_record, segmented_marker_view,
        segmented_segment_view,
    },
};

/// Read at most seven rows, retaining at most one candidate body and six refs.
/// Complete is local integrity only; logical anchor awaits separate confirmation.
pub fn scan_next_segmented_recovery(
    scan: &mut SegmentedRecoveryScan,
    repository: &OpaqueRecordRepository,
    reserved_bytes: usize,
) -> Result<SegmentedRecoveryStep, SegmentedRecoveryError> {
    if scan.failed {
        return Err(SegmentedRecoveryError::FailedScanner);
    }
    if scan.pending.is_some() {
        return Err(SegmentedRecoveryError::ConfirmationRequired);
    }
    if scan.namespace != opaque_record_identity(repository)
        || scan.budget != opaque_record_budget(repository)
    {
        return Err(SegmentedRecoveryError::ForeignRepository);
    }
    if reserved_bytes < required_segmented_recovery_reservation(&scan.budget, &scan.limits)? {
        return Err(SegmentedRecoveryError::ReservationTooSmall);
    }
    let head = match opaque_record_cursor(repository) {
        Ok(head) => head,
        Err(_) => {
            scan.failed = true;
            return Err(SegmentedRecoveryError::StorageFailed);
        }
    };
    if scan.physical.sequence > head.sequence
        || (scan.physical.sequence == head.sequence && scan.physical != head)
    {
        scan.failed = true;
        return Err(SegmentedRecoveryError::WrongPhysicalParent);
    }
    for rows_scanned in 0..=SEGMENTED_MAX_REFERENCES {
        if scan.physical == head {
            return Ok(if scan.candidate.is_some() {
                SegmentedRecoveryStep::Incomplete { rows_scanned }
            } else {
                SegmentedRecoveryStep::Exhausted
            });
        }
        let sequence = scan
            .physical
            .sequence
            .checked_add(1)
            .ok_or(SegmentedRecoveryError::ArithmeticOverflow)?;
        let record = match read_opaque_record(repository, sequence) {
            Ok(Some(record)) => record,
            _ => {
                scan.failed = true;
                return Err(SegmentedRecoveryError::StorageFailed);
            }
        };
        if record.parent != scan.physical {
            scan.failed = true;
            return Err(SegmentedRecoveryError::WrongPhysicalParent);
        }
        let preflight = match preflight_segmented_record(&record.payload, &scan.limits) {
            Ok(value) => value,
            Err(error) => {
                scan.failed = true;
                return Err(SegmentedRecoveryError::Codec(error));
            }
        };
        let cursor = OpaqueRecordCursor {
            sequence: record.sequence,
            content_hash: record.content_hash,
        };
        if let Some(segment) = segmented_segment_view(&preflight) {
            if let Err(error) = append_segmented_recovery_candidate(scan, segment, cursor) {
                scan.failed = true;
                return Err(error);
            }
        } else if let Some(marker) = segmented_marker_view(&preflight) {
            let result = complete_segmented_recovery_candidate(scan, marker, cursor, record.parent);
            match result {
                Ok(bundle) => {
                    scan.physical = cursor;
                    return Ok(SegmentedRecoveryStep::Complete(Box::new(bundle)));
                }
                Err(error) => {
                    scan.failed = true;
                    return Err(error);
                }
            }
        } else {
            scan.failed = true;
            return Err(SegmentedRecoveryError::InvalidMarker);
        }
        scan.physical = cursor;
    }
    Ok(if scan.physical == head {
        SegmentedRecoveryStep::Incomplete {
            rows_scanned: SEGMENTED_MAX_REFERENCES + 1,
        }
    } else {
        SegmentedRecoveryStep::Progress {
            rows_scanned: SEGMENTED_MAX_REFERENCES + 1,
        }
    })
}
