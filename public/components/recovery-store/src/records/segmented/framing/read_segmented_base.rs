// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        SEGMENTED_SCHEMA_VERSION, SegmentedCodecError, SegmentedRecordKind,
        SegmentedRecoveryAnchor, SegmentedRecoveryMode,
        types::{SEGMENTED_DOMAIN, SegmentedBase},
    },
    read_segmented_array::read_segmented_array,
};
use crate::records::OpaqueRecordCursor;

pub(in crate::records::segmented) fn read_segmented_base(
    bytes: &[u8],
) -> Result<SegmentedBase, SegmentedCodecError> {
    if bytes.get(..25) != Some(SEGMENTED_DOMAIN) {
        return Err(SegmentedCodecError::MalformedEncoding);
    }
    let kind = match read_segmented_array::<1>(bytes, 25)?[0] {
        1 => SegmentedRecordKind::Segment,
        2 => SegmentedRecordKind::CommitMarker,
        _ => return Err(SegmentedCodecError::MalformedEncoding),
    };
    if u16::from_be_bytes(read_segmented_array(bytes, 26)?) != SEGMENTED_SCHEMA_VERSION {
        return Err(SegmentedCodecError::UnsupportedVersion);
    }
    let mode = match read_segmented_array::<1>(bytes, 28)?[0] {
        1 => SegmentedRecoveryMode::AuthenticatedImport,
        _ => return Err(SegmentedCodecError::UnsupportedMode),
    };
    Ok(SegmentedBase {
        kind,
        mode,
        logical_id: read_segmented_array(bytes, 29)?,
        parent: SegmentedRecoveryAnchor {
            height: u64::from_be_bytes(read_segmented_array(bytes, 61)?),
            cursor: OpaqueRecordCursor {
                sequence: u64::from_be_bytes(read_segmented_array(bytes, 69)?),
                content_hash: read_segmented_array(bytes, 77)?,
            },
            state_binding: read_segmented_array(bytes, 109)?,
        },
        target_height: u64::from_be_bytes(read_segmented_array(bytes, 141)?),
    })
}
