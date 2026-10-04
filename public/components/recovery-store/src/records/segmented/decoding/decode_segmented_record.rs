// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedCodecError, SegmentedCommitMarker, SegmentedRecord, SegmentedRecordPreflight,
    SegmentedSegment, framing::read_segmented_reference::read_segmented_reference,
    types::BorrowedSegmentedRecord,
};

/// Decode only the same borrowed preflight. Fixed-width canonical framing and
/// hashes were checked before allocation; decoded contents confer no authority.
pub fn decode_segmented_record(
    preflight: &SegmentedRecordPreflight<'_>,
) -> Result<SegmentedRecord, SegmentedCodecError> {
    match &preflight.record {
        BorrowedSegmentedRecord::Segment(view) => {
            let mut data = Vec::new();
            data.try_reserve_exact(view.data.len())
                .map_err(|_| SegmentedCodecError::AllocationFailed)?;
            data.extend_from_slice(view.data);
            Ok(SegmentedRecord::Segment(SegmentedSegment {
                metadata: view.metadata,
                data,
                chunk_hash: view.chunk_hash,
            }))
        }
        BorrowedSegmentedRecord::CommitMarker(view) => {
            let mut references = Vec::new();
            references
                .try_reserve_exact(view.reference_count)
                .map_err(|_| SegmentedCodecError::AllocationFailed)?;
            for index in 0..view.reference_count {
                references.push(read_segmented_reference(view.references, index)?);
            }
            Ok(SegmentedRecord::CommitMarker(SegmentedCommitMarker {
                metadata: view.metadata,
                references,
                marker_hash: view.marker_hash,
            }))
        }
    }
}
