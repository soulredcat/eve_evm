// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AppliedState;
use eve_finality_verifier::{
    AuthenticatedApplicationAnchor, imported_state_anchor, recovery_state_anchor,
};

pub(in crate::sync::applied) fn applied_state_anchor(
    state: &AppliedState,
) -> Option<&AuthenticatedApplicationAnchor> {
    match state {
        AppliedState::EmptyReplay(state) => recovery_state_anchor(state),
        AppliedState::AuthenticatedImport(state) => imported_state_anchor(state),
    }
}
