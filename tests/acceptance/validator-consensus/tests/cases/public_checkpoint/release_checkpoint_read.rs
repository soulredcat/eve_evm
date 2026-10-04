// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::CheckpointReadLease;

pub(super) fn release_checkpoint_read(lease: &CheckpointReadLease<'_>) {
    lease.budget.used.set(
        lease
            .budget
            .used
            .get()
            .checked_sub(lease.bytes)
            .expect("owned checkpoint read lease"),
    );
}
