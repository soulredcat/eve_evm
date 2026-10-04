// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AppliedStorageReadReservation, StorageAdmissionError, types::StorageAdmissionPool};
use std::sync::Arc;
pub(in crate::sync::applied) fn reserve_storage_read(
    pool: &Arc<StorageAdmissionPool>,
) -> Result<AppliedStorageReadReservation, StorageAdmissionError> {
    let mut accounting = pool
        .accounting
        .lock()
        .map_err(|_| StorageAdmissionError::AccountingUnavailable)?;
    let next = accounting
        .reads
        .checked_add(1)
        .ok_or(StorageAdmissionError::Overflow)?;
    if next > pool.reads_limit {
        return Err(StorageAdmissionError::ReadCapacity);
    }
    accounting.reads = next;
    accounting.peak_reads = accounting.peak_reads.max(next);
    Ok(AppliedStorageReadReservation {
        pool: Arc::clone(pool),
    })
}
