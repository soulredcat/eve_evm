// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{CheckpointExecutionWitness, CheckpointWitness};
use super::{
    CheckpointWitnessWireError, CheckpointWitnessWireKind, CheckpointWitnessWirePreflight,
    encode_checkpoint_witness_wire, required_checkpoint_witness_decode_reservation,
};
use crate::recovery::decoding::decode_native_data_frame::decode_native_data_frame;
use eve_state::{decode_block_payload, decode_state_version};

/// Decode the sealed same bytes/frozen policy only. No proof validation or state authority is created.
pub fn decode_checkpoint_witness_wire(
    preflight: &CheckpointWitnessWirePreflight<'_>,
    reserved_bytes: usize,
) -> Result<CheckpointWitness, CheckpointWitnessWireError> {
    if reserved_bytes < required_checkpoint_witness_decode_reservation(preflight)? {
        return Err(CheckpointWitnessWireError::ReservationTooSmall);
    }
    let native =
        decode_native_data_frame(preflight.native).map_err(CheckpointWitnessWireError::Recovery)?;
    let witness = match preflight.kind {
        CheckpointWitnessWireKind::Execution => {
            CheckpointWitness::Execution(Box::new(CheckpointExecutionWitness {
                native,
                version: decode_state_version(
                    preflight
                        .version
                        .ok_or(CheckpointWitnessWireError::MalformedEncoding)?,
                )
                .map_err(CheckpointWitnessWireError::State)?,
                block: decode_block_payload(
                    preflight
                        .execution
                        .ok_or(CheckpointWitnessWireError::MalformedEncoding)?,
                    &preflight.budget,
                )
                .map_err(CheckpointWitnessWireError::State)?,
            }))
        }
        CheckpointWitnessWireKind::Lookahead => CheckpointWitness::Lookahead(Box::new(native)),
    };
    if encode_checkpoint_witness_wire(&witness, &preflight.budget, preflight.limits)?
        != preflight.bytes
    {
        return Err(CheckpointWitnessWireError::NonCanonicalEncoding);
    }
    Ok(witness)
}
