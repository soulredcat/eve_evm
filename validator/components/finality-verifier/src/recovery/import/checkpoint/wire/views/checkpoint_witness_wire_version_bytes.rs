// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::CheckpointWitnessWirePreflight;
pub fn checkpoint_witness_wire_version_bytes<'a>(
    preflight: &CheckpointWitnessWirePreflight<'a>,
) -> Option<&'a [u8]> {
    preflight.version
}
