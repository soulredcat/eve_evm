// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::CheckpointWitnessWirePreflight;
use eve_state::StateBudget;
pub fn checkpoint_witness_wire_budget(
    preflight: &CheckpointWitnessWirePreflight<'_>,
) -> StateBudget {
    preflight.budget
}
