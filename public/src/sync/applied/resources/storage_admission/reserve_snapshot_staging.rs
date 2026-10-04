// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    AppliedSnapshotStagingReservation, StorageAdmissionError, types::StorageAdmissionPool,
};
use std::sync::Arc;
pub(in crate::sync::applied) fn reserve_snapshot_staging(
    pool: &Arc<StorageAdmissionPool>,
    bytes: usize,
) -> Result<AppliedSnapshotStagingReservation, StorageAdmissionError> {
    let bytes = u64::try_from(bytes).map_err(|_| StorageAdmissionError::Overflow)?;
    let mut accounting = pool
        .accounting
        .lock()
        .map_err(|_| StorageAdmissionError::AccountingUnavailable)?;
    let next = accounting
        .staging
        .checked_add(bytes)
        .ok_or(StorageAdmissionError::Overflow)?;
    if next > pool.staging_limit {
        return Err(StorageAdmissionError::StagingCapacity);
    }
    accounting.staging = next;
    accounting.peak_staging = accounting.peak_staging.max(next);
    Ok(AppliedSnapshotStagingReservation {
        pool: Arc::clone(pool),
        bytes,
    })
}
