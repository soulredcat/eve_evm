// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{Candidate, SegmentedRecoveryError, SegmentedRecoveryScan};
use crate::records::{
    OpaqueRecordCursor,
    segmented::{SEGMENTED_MAX_REFERENCES, SegmentedSegmentView},
};

/// Index zero alone authorizes local restart. Old physical rows remain untouched.
pub(super) fn append_segmented_recovery_candidate(
    scan: &mut SegmentedRecoveryScan,
    segment: SegmentedSegmentView<'_>,
    cursor: OpaqueRecordCursor,
) -> Result<(), SegmentedRecoveryError> {
    if segment.metadata.identity.parent != scan.anchor {
        return Err(SegmentedRecoveryError::InvalidSuffix);
    }
    if segment.metadata.index == 0 {
        if let Some(candidate) = scan.candidate {
            scan.orphan_segments = scan
                .orphan_segments
                .checked_add(candidate.received as u64)
                .ok_or(SegmentedRecoveryError::ArithmeticOverflow)?;
        }
        scan.body.clear();
        scan.candidate = Some(Candidate {
            identity: segment.metadata.identity,
            references: [OpaqueRecordCursor::default(); SEGMENTED_MAX_REFERENCES],
            received: 0,
            expected: segment.metadata.count as usize,
        });
    }
    let candidate = scan
        .candidate
        .as_mut()
        .ok_or(SegmentedRecoveryError::InvalidSuffix)?;
    if candidate.identity != segment.metadata.identity
        || candidate.received != segment.metadata.index as usize
        || candidate.expected != segment.metadata.count as usize
        || scan.body.len() as u64 != segment.metadata.offset
        || candidate.received >= SEGMENTED_MAX_REFERENCES
    {
        return Err(SegmentedRecoveryError::InvalidSuffix);
    }
    let length = scan
        .body
        .len()
        .checked_add(segment.data.len())
        .ok_or(SegmentedRecoveryError::ArithmeticOverflow)?;
    if length > scan.limits.maximum_logical_bytes || length > scan.body.capacity() {
        return Err(SegmentedRecoveryError::InvalidSuffix);
    }
    scan.body.extend_from_slice(segment.data);
    candidate.references[candidate.received] = cursor;
    candidate.received = candidate
        .received
        .checked_add(1)
        .ok_or(SegmentedRecoveryError::ArithmeticOverflow)?;
    Ok(())
}
