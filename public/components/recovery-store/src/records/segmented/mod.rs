// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Pure bounded local segmented-record integrity. No finality, I/O or logical publication.
mod decoding;
mod encoding;
mod framing;
mod hashing;
mod preflight;
mod types;
mod validation;
mod views;

pub use decoding::decode_segmented_record;
pub use encoding::{
    encode_segmented_marker, encode_segmented_segment, write_segmented_marker,
    write_segmented_segment,
};
pub use hashing::{hash_segmented_chunk, hash_segmented_marker};
pub use preflight::preflight_segmented_record;
pub use types::{
    SEGMENTED_MARKER_HEADER_BYTES, SEGMENTED_MARKER_OVERHEAD_BYTES, SEGMENTED_MAX_CHUNK_BYTES,
    SEGMENTED_MAX_MARKER_BYTES, SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES, SEGMENTED_MAX_REFERENCES,
    SEGMENTED_SCHEMA_VERSION, SEGMENTED_SEGMENT_HEADER_BYTES, SEGMENTED_SEGMENT_OVERHEAD_BYTES,
    SegmentedCodecError, SegmentedCodecLimits, SegmentedCommitMarker, SegmentedLogicalIdentity,
    SegmentedMarkerMetadata, SegmentedMarkerView, SegmentedRecord, SegmentedRecordKind,
    SegmentedRecordPreflight, SegmentedRecordStats, SegmentedRecoveryAnchor, SegmentedRecoveryMode,
    SegmentedSegment, SegmentedSegmentMetadata, SegmentedSegmentView,
};
pub use validation::validate_segmented_codec_limits::validate_segmented_codec_limits;
pub use views::{
    match_segmented_identity, segmented_marker_reference_at, segmented_marker_view,
    segmented_record_bytes, segmented_record_limits, segmented_record_stats,
    segmented_segment_view,
};
