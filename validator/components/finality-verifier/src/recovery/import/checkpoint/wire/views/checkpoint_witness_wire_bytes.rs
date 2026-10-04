// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::CheckpointWitnessWirePreflight;
pub fn checkpoint_witness_wire_bytes<'a>(
    preflight: &CheckpointWitnessWirePreflight<'a>,
) -> &'a [u8] {
    preflight.bytes
}
