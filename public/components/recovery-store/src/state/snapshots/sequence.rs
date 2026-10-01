// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::StateSnapshot;

impl StateSnapshot<'_> {
    pub fn sequence(&self) -> u64 {
        self.database_sequence
    }
}
