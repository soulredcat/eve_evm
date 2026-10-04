// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedOwner;
use eve_state::StateBudget;

pub fn applied_owner_state_budget(owner: &AppliedOwner) -> StateBudget {
    owner.config.state_budget
}
