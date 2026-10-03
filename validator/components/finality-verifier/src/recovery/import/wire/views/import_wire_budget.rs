// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::StateBudget;

use super::super::ImportWirePreflight;

pub fn import_wire_budget(preflight: &ImportWirePreflight<'_>) -> StateBudget {
    preflight.budget
}
