// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AppliedState;
use crate::sync::applied::AppliedMode;

pub(in crate::sync::applied) fn applied_state_mode(state: &AppliedState) -> AppliedMode {
    match state {
        AppliedState::EmptyReplay(_) => AppliedMode::EmptyReplay,
        AppliedState::AuthenticatedImport(_) => AppliedMode::AuthenticatedImport,
    }
}
