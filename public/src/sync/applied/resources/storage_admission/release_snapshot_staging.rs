// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::StorageAdmissionPool;
pub(super) fn release_snapshot_staging(pool: &StorageAdmissionPool, bytes: u64) {
    let mut accounting = pool
        .accounting
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    accounting.staging = accounting
        .staging
        .checked_sub(bytes)
        .expect("exclusively owned staging lease");
}
