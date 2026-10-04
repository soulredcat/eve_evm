// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateCommitPreflight;
use crate::StateBudget;
pub fn state_commit_preflight_budget(preflight: &StateCommitPreflight<'_>) -> StateBudget {
    preflight.budget
}
