// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::StateCommit;

use crate::recovery::import::ImportedState;

/// Complete local representation. Only EVE_APP_V1 fields are certificate-authenticated.
pub fn imported_state_commit(state: &ImportedState) -> &StateCommit {
    &state.commit
}
