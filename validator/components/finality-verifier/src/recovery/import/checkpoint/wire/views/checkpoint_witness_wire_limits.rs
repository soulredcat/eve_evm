// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::super::CheckpointLimits;
use super::super::CheckpointWitnessWirePreflight;
pub fn checkpoint_witness_wire_limits(
    preflight: &CheckpointWitnessWirePreflight<'_>,
) -> CheckpointLimits {
    preflight.limits
}
