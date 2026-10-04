// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedCodecError, SegmentedCodecLimits, SegmentedMarkerMetadata,
    framing::encode_segmented_marker_header::encode_segmented_marker_header,
    validation::validate_segmented_marker::validate_segmented_marker,
};
use crate::records::OpaqueRecordCursor;
use sha2::{Digest, Sha256};

/// Standard SHA-256 of canonical marker metadata and ordered opaque references.
/// The hash proves no segment membership, successful sync or finality.
pub fn hash_segmented_marker(
    metadata: &SegmentedMarkerMetadata,
    references: &[OpaqueRecordCursor],
    limits: &SegmentedCodecLimits,
) -> Result<[u8; 32], SegmentedCodecError> {
    validate_segmented_marker(metadata, references, limits)?;
    let count = u32::try_from(references.len()).map_err(|_| SegmentedCodecError::LimitExceeded)?;
    let mut hash = Sha256::new();
    hash.update(encode_segmented_marker_header(metadata, count));
    for reference in references {
        hash.update(reference.sequence.to_be_bytes());
        hash.update(reference.content_hash);
    }
    Ok(hash.finalize().into())
}
