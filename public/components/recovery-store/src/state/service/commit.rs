// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::ImmutableStateView;
use eve_state::StateCommit;

impl ImmutableStateView {
    pub fn commit(&self) -> &StateCommit {
        &self.commit
    }
}
