// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AuthenticatedCheckpoint;
use eve_state::StateCommit;
pub fn checkpoint_commit(checkpoint: &AuthenticatedCheckpoint) -> &StateCommit {
    &checkpoint.commit
}
