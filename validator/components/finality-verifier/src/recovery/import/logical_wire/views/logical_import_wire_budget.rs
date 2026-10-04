// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::LogicalImportWirePreflight;
use eve_state::StateBudget;

pub fn logical_import_wire_budget(preflight: &LogicalImportWirePreflight<'_>) -> StateBudget {
    preflight.budget
}
