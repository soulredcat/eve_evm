// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::StateSnapshot;
use eve_state::StateVersion;

impl StateSnapshot<'_> {
    pub fn version(&self) -> &StateVersion {
        &self.version
    }
}
