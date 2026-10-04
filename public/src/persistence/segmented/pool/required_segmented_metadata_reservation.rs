// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedError, types::BATCHES};
use super::estimate_segmented_metadata;

/// Concrete pool, all retained batch descriptors, and one worker; not RSS.
pub fn required_segmented_metadata_reservation() -> Result<usize, SegmentedError> {
    let (pool, batch, worker) = estimate_segmented_metadata();
    batch
        .checked_mul(BATCHES)
        .and_then(|bytes| bytes.checked_add(pool))
        .and_then(|bytes| bytes.checked_add(worker))
        .ok_or(SegmentedError::Overflow)
}
