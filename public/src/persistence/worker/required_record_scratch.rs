// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::RecordWorkerError;
use eve_storage::records::OpaqueRecordBudget;

/// Conservative logical staging envelope for the current opaque repository implementation.
pub(super) fn required_record_scratch(
    payload_bytes: usize,
    budget: OpaqueRecordBudget,
) -> Result<u64, RecordWorkerError> {
    let payload = u64::try_from(payload_bytes).map_err(|_| RecordWorkerError::ScratchLimit)?;
    let batch =
        u64::try_from(budget.maximum_batch_bytes).map_err(|_| RecordWorkerError::ScratchLimit)?;
    let read =
        u64::try_from(budget.maximum_read_bytes).map_err(|_| RecordWorkerError::ScratchLimit)?;
    payload
        .checked_mul(2)
        .and_then(|bytes| bytes.checked_add(88))
        .and_then(|bytes| bytes.checked_add(batch))
        .and_then(|bytes| bytes.checked_add(read.checked_mul(2)?))
        .and_then(|bytes| bytes.checked_add(4096))
        .ok_or(RecordWorkerError::ScratchLimit)
}
