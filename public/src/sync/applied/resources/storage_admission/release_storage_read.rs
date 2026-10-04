// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::StorageAdmissionPool;
pub(super) fn release_storage_read(pool: &StorageAdmissionPool) {
    let mut accounting = pool
        .accounting
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    accounting.reads = accounting
        .reads
        .checked_sub(1)
        .expect("one exclusively owned storage read lease");
}
