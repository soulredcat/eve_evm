// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        SegmentedCodecError, SegmentedCodecLimits, SegmentedRecordKind, SegmentedRecordPreflight,
        framing::read_segmented_base::read_segmented_base, types::BorrowedSegmentedRecord,
        validation::validate_segmented_codec_limits::validate_segmented_codec_limits,
    },
    preflight_segmented_marker::preflight_segmented_marker,
    preflight_segmented_segment::preflight_segmented_segment,
};

/// Borrowed width/count/hash validation BEFORE decoded Vec allocation. This
/// proves only canonical local record integrity, never finality or successful sync.
pub fn preflight_segmented_record<'a>(
    bytes: &'a [u8],
    limits: &SegmentedCodecLimits,
) -> Result<SegmentedRecordPreflight<'a>, SegmentedCodecError> {
    validate_segmented_codec_limits(limits)?;
    if bytes.len() > limits.maximum_payload_bytes {
        return Err(SegmentedCodecError::LimitExceeded);
    }
    let base = read_segmented_base(bytes)?;
    let (record, stats) = match base.kind {
        SegmentedRecordKind::Segment => {
            let (view, stats) = preflight_segmented_segment(bytes, base, limits)?;
            (BorrowedSegmentedRecord::Segment(view), stats)
        }
        SegmentedRecordKind::CommitMarker => {
            let (view, stats) = preflight_segmented_marker(bytes, base, limits)?;
            (BorrowedSegmentedRecord::CommitMarker(view), stats)
        }
    };
    Ok(SegmentedRecordPreflight {
        bytes,
        limits: *limits,
        record,
        stats,
    })
}
