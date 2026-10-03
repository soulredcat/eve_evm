// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedPublication;
use crate::sync::applied::state::applied_state_anchor;
use eve_finality_verifier::AuthenticatedApplicationAnchor;

pub fn applied_anchor(publication: &AppliedPublication) -> Option<&AuthenticatedApplicationAnchor> {
    applied_state_anchor(&publication.generation.state)
}
