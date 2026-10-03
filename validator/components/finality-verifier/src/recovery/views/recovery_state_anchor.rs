// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{AuthenticatedApplicationAnchor, recovery::DevelopmentRecoveryState};

pub fn recovery_state_anchor(
    state: &DevelopmentRecoveryState,
) -> Option<&AuthenticatedApplicationAnchor> {
    state.anchor.as_ref()
}
