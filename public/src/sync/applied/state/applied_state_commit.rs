// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AppliedState;
use eve_finality_verifier::{imported_state_commit, recovery_state_commit};
use eve_state::StateCommit;

pub(in crate::sync::applied) fn applied_state_commit(state: &AppliedState) -> &StateCommit {
    match state {
        AppliedState::EmptyReplay(state) => recovery_state_commit(state),
        AppliedState::AuthenticatedImport(state) => imported_state_commit(state),
    }
}
