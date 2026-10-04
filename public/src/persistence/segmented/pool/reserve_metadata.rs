// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedError, SegmentedPartPool, types::MetadataLease};
use std::sync::Arc;
pub(in crate::persistence::segmented) fn reserve_metadata(
    pool: &Arc<SegmentedPartPool>,
    bytes: usize,
) -> Result<MetadataLease, SegmentedError> {
    let bytes = u64::try_from(bytes).map_err(|_| SegmentedError::Overflow)?;
    let mut state = pool
        .accounting
        .lock()
        .map_err(|_| SegmentedError::AccountingUnavailable)?;
    let next = state
        .metadata
        .checked_add(bytes)
        .ok_or(SegmentedError::Overflow)?;
    if next > pool.policy.maximum_metadata_bytes {
        return Err(SegmentedError::Capacity);
    }
    state.metadata = next;
    Ok(MetadataLease {
        pool: Arc::clone(pool),
        bytes,
    })
}
