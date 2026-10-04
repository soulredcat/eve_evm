// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::SegmentedError;
use eve_storage::records::OpaqueRecordBudget;
pub(in crate::persistence::segmented) fn required_segment_scratch(
    payload_bytes: usize,
    budget: OpaqueRecordBudget,
) -> Result<u64, SegmentedError> {
    (payload_bytes as u64)
        .checked_mul(2)
        .and_then(|bytes| bytes.checked_add(88))
        .and_then(|bytes| bytes.checked_add(budget.maximum_batch_bytes as u64))
        .and_then(|bytes| bytes.checked_add((budget.maximum_read_bytes as u64).checked_mul(2)?))
        .and_then(|bytes| bytes.checked_add(4_096))
        .ok_or(SegmentedError::Overflow)
}
