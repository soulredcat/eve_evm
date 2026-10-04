// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{CheckpointWitnessWireKind, CheckpointWitnessWirePreflight};
pub fn checkpoint_witness_wire_kind(
    preflight: &CheckpointWitnessWirePreflight<'_>,
) -> CheckpointWitnessWireKind {
    preflight.kind
}
