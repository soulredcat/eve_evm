// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod error_types;
mod identity_types;
mod limit_types;
mod preflight_types;
mod record_types;

pub use error_types::SegmentedCodecError;
pub(super) use identity_types::SegmentedBase;
pub use identity_types::{
    SegmentedLogicalIdentity, SegmentedMarkerMetadata, SegmentedRecoveryAnchor,
    SegmentedRecoveryMode, SegmentedSegmentMetadata,
};
pub(super) use limit_types::{SEGMENTED_BASE_BYTES, SEGMENTED_DOMAIN};
pub use limit_types::{
    SEGMENTED_MARKER_HEADER_BYTES, SEGMENTED_MARKER_OVERHEAD_BYTES, SEGMENTED_MAX_CHUNK_BYTES,
    SEGMENTED_MAX_MARKER_BYTES, SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES, SEGMENTED_MAX_REFERENCES,
    SEGMENTED_SCHEMA_VERSION, SEGMENTED_SEGMENT_HEADER_BYTES, SEGMENTED_SEGMENT_OVERHEAD_BYTES,
    SegmentedCodecLimits,
};
pub(super) use preflight_types::BorrowedSegmentedRecord;
pub use preflight_types::{
    SegmentedMarkerView, SegmentedRecordPreflight, SegmentedRecordStats, SegmentedSegmentView,
};
pub use record_types::{
    SegmentedCommitMarker, SegmentedRecord, SegmentedRecordKind, SegmentedSegment,
};
