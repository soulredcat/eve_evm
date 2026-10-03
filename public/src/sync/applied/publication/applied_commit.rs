// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedPublication;
use crate::sync::applied::state::applied_state_commit;
use eve_state::StateCommit;

pub fn applied_commit(publication: &AppliedPublication) -> &StateCommit {
    applied_state_commit(&publication.generation.state)
}
