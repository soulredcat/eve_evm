// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::history::HistorySnapshot;
impl HistorySnapshot<'_> {
    pub fn sequence(&self) -> u64 {
        self.database_sequence
    }
}
