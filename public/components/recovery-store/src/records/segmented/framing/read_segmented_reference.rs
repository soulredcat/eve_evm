// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{super::SegmentedCodecError, read_segmented_array::read_segmented_array};
use crate::records::OpaqueRecordCursor;

pub(in crate::records::segmented) fn read_segmented_reference(
    bytes: &[u8],
    index: usize,
) -> Result<OpaqueRecordCursor, SegmentedCodecError> {
    let offset = index
        .checked_mul(40)
        .ok_or(SegmentedCodecError::InvalidReferences)?;
    Ok(OpaqueRecordCursor {
        sequence: u64::from_be_bytes(read_segmented_array(bytes, offset)?),
        content_hash: read_segmented_array(bytes, offset + 8)?,
    })
}
