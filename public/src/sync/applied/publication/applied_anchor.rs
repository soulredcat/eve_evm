// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedPublication;
use eve_finality_verifier::{AuthenticatedApplicationAnchor, recovery_state_anchor};

pub fn applied_anchor(publication: &AppliedPublication) -> Option<&AuthenticatedApplicationAnchor> {
    recovery_state_anchor(&publication.generation.recovery)
}
