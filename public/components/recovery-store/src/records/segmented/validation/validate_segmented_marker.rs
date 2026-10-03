// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{SegmentedCodecError, SegmentedCodecLimits, SegmentedMarkerMetadata},
    validate_segmented_references::validate_segmented_references,
};
use crate::records::OpaqueRecordCursor;

/// Reference shape only; actual segment membership and sync belong to the orchestrator.
pub(in crate::records::segmented) fn validate_segmented_marker(
    metadata: &SegmentedMarkerMetadata,
    references: &[OpaqueRecordCursor],
    limits: &SegmentedCodecLimits,
) -> Result<(), SegmentedCodecError> {
    validate_segmented_references(
        metadata,
        references.len(),
        references.iter().copied().map(Ok),
        limits,
    )
}
