// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::release_history_snapshot_lease::release_history_snapshot_lease;
use crate::state::history::HistorySnapshot;
impl std::ops::Drop for HistorySnapshot<'_> {
    fn drop(&mut self) {
        release_history_snapshot_lease(&self.leases);
    }
}
