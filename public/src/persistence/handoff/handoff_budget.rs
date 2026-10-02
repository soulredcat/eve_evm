// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::HandoffPool;
use eve_node_policy::PublicBudget;

pub(in crate::persistence) fn handoff_budget(pool: &HandoffPool) -> PublicBudget {
    pool.budget
}
