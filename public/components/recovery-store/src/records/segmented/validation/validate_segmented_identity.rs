// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedCodecError, SegmentedCodecLimits, SegmentedLogicalIdentity};

pub(in crate::records::segmented) fn validate_segmented_identity(
    identity: &SegmentedLogicalIdentity,
    limits: &SegmentedCodecLimits,
) -> Result<usize, SegmentedCodecError> {
    if identity.logical_id == [0; 32]
        || identity.parent.state_binding == [0; 32]
        || identity.parent.cursor.content_hash == [0; 32]
        || (identity.parent.height == 0) != (identity.parent.cursor.sequence == 0)
        || identity.parent.height.checked_add(1) != Some(identity.target_height)
    {
        return Err(SegmentedCodecError::InvalidIdentity);
    }
    let length =
        usize::try_from(identity.total_length).map_err(|_| SegmentedCodecError::LimitExceeded)?;
    if length == 0 || length > limits.maximum_logical_bytes {
        return Err(SegmentedCodecError::LimitExceeded);
    }
    let count = length.div_ceil(limits.maximum_chunk_bytes);
    if count > limits.maximum_segments {
        return Err(SegmentedCodecError::LimitExceeded);
    }
    Ok(count)
}
