// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::ReservationLease;

pub(super) fn release_handoff_reservation(lease: &ReservationLease) {
    // Admission/observation remain fail-closed after poisoning; cleanup still releases ownership.
    let mut accounting = lease
        .pool
        .accounting
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if accounting.leases.remove(&lease.id).is_some() {
        accounting.bytes -= lease.bytes;
    }
}
