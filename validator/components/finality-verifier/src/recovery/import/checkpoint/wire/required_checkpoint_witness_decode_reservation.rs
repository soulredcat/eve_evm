// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointWitnessWireError, CheckpointWitnessWirePreflight,
    estimate_checkpoint_witness_decode_charge::estimate_checkpoint_witness_decode_charge,
};

/// Numeric logical envelope only. Caller holds a real lease before decode through decoded object drop.
pub fn required_checkpoint_witness_decode_reservation(
    preflight: &CheckpointWitnessWirePreflight<'_>,
) -> Result<usize, CheckpointWitnessWireError> {
    estimate_checkpoint_witness_decode_charge(preflight.stats)
}
