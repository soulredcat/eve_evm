// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::MasterFollower;
use eve_state::StateCommit;

/// Borrow the actual private imported generation; callers cannot shed its retained charge.
pub fn current_master_commit(follower: &MasterFollower) -> &StateCommit {
    eve_finality_verifier::imported_state_commit(&follower.current.state)
}
