// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::EstimatedWorkingLease;

pub(super) fn release_estimated_working(lease: &EstimatedWorkingLease) {
    let mut accounting = lease
        .pool
        .accounting
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    accounting.bytes -= lease.bytes;
}
