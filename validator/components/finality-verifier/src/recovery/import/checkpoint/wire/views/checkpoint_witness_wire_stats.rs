// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{CheckpointWitnessWirePreflight, CheckpointWitnessWireStats};
pub fn checkpoint_witness_wire_stats(
    preflight: &CheckpointWitnessWirePreflight<'_>,
) -> CheckpointWitnessWireStats {
    preflight.stats
}
