// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::SelectedState;
use eve_state::StateCommit;
impl SelectedState {
    pub(crate) fn commit(&self) -> &StateCommit {
        match self {
            Self::Current { view, .. } => view.commit(),
            Self::Owned { commit, .. } => commit,
        }
    }
}
