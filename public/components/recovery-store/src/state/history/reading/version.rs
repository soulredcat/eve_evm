// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::history::HistorySnapshot;
use eve_state::StateVersion;
impl HistorySnapshot<'_> {
    pub fn version(&self) -> &StateVersion {
        &self.head
    }
}
