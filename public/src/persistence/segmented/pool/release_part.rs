// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::PartLease;
pub(super) fn release_part(lease: &PartLease) {
    let mut state = lease
        .pool
        .accounting
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if state.slots[lease.slot].is_some_and(|slot| slot.id == lease.id) {
        let slot = state.slots[lease.slot]
            .take()
            .expect("checked part ownership");
        state.bytes -= slot.bytes;
    }
}
