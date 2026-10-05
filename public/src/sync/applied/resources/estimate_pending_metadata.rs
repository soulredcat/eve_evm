// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedError, types::PendingRecord};

/// Count-derived logical envelope: actual slot size plus 8KiB for bounded version strings,
/// reply channels/IDs and allocation bookkeeping, plus 4KiB fixed queue/publication allowance.
/// This is a conservative estimate, not allocator/RSS measurement.
pub(in crate::sync::applied) fn estimate_pending_metadata(
    count: usize,
) -> Result<usize, AppliedError> {
    let per_record = std::mem::size_of::<PendingRecord>()
        .checked_add(8_192)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    count
        .checked_mul(per_record)
        .and_then(|bytes| bytes.checked_add(4_096))
        .ok_or(AppliedError::ArithmeticOverflow)
}
