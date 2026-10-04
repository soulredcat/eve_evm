// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AppliedStorageObservation, StorageAdmissionError, types::StorageAdmissionPool};
pub(in crate::sync::applied) fn observe_storage_admission(
    pool: &StorageAdmissionPool,
) -> Result<AppliedStorageObservation, StorageAdmissionError> {
    let accounting = pool
        .accounting
        .lock()
        .map_err(|_| StorageAdmissionError::AccountingUnavailable)?;
    Ok(AppliedStorageObservation {
        active_reads: accounting.reads,
        peak_reads: accounting.peak_reads,
        read_limit: pool.reads_limit,
        reserved_staging_bytes: accounting.staging,
        peak_staging_bytes: accounting.peak_staging,
        staging_limit: pool.staging_limit,
    })
}
