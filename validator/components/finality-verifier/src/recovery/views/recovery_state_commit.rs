// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::StateCommit;

use crate::recovery::DevelopmentRecoveryState;

pub fn recovery_state_commit(state: &DevelopmentRecoveryState) -> &StateCommit {
    &state.commit
}
