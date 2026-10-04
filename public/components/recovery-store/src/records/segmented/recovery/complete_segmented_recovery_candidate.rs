// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{
    Completion, RecoveredSegmentedBundle, SegmentedRecoveryError, SegmentedRecoveryScan,
};
use crate::records::{
    OpaqueRecordCursor,
    segmented::{
        SegmentedMarkerView, SegmentedRecoveryAnchor, hash_segmented_logical_body,
        segmented_marker_reference_at,
    },
};

pub(super) fn complete_segmented_recovery_candidate(
    scan: &mut SegmentedRecoveryScan,
    marker: SegmentedMarkerView<'_>,
    cursor: OpaqueRecordCursor,
    physical_parent: OpaqueRecordCursor,
) -> Result<RecoveredSegmentedBundle, SegmentedRecoveryError> {
    let candidate = scan
        .candidate
        .ok_or(SegmentedRecoveryError::InvalidMarker)?;
    if candidate.identity != marker.metadata.identity
        || candidate.received != candidate.expected
        || marker.reference_count != candidate.expected
        || scan.body.len() as u64 != marker.metadata.identity.total_length
    {
        return Err(SegmentedRecoveryError::InvalidMarker);
    }
    for index in 0..marker.reference_count {
        if segmented_marker_reference_at(&marker, index).map_err(SegmentedRecoveryError::Codec)?
            != candidate.references[index]
        {
            return Err(SegmentedRecoveryError::InvalidMarker);
        }
    }
    if candidate.references[candidate.received - 1] != physical_parent
        || physical_parent.sequence.checked_add(1) != Some(cursor.sequence)
    {
        return Err(SegmentedRecoveryError::InvalidMarker);
    }
    if hash_segmented_logical_body(scan.body.as_slice()) != candidate.identity.logical_id {
        return Err(SegmentedRecoveryError::LogicalHashMismatch);
    }
    let completion = Completion {
        identity: candidate.identity,
        target: SegmentedRecoveryAnchor {
            height: candidate.identity.target_height,
            cursor,
            state_binding: marker.metadata.target_state_binding,
        },
        references: candidate.references,
        count: candidate.received,
    };
    scan.pending = Some(completion);
    scan.candidate = None;
    Ok(RecoveredSegmentedBundle {
        namespace: scan.namespace,
        completion,
        body: std::mem::take(&mut scan.body),
    })
}
