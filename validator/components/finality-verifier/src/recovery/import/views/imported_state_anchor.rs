// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{AuthenticatedApplicationAnchor, recovery::import::ImportedState};

pub fn imported_state_anchor(state: &ImportedState) -> Option<&AuthenticatedApplicationAnchor> {
    state.anchor.as_ref()
}
