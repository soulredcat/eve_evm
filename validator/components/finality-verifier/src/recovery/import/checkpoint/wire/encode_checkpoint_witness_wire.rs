// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{CheckpointLimits, CheckpointWitness};
use super::{
    CheckpointWitnessWireError, measure_checkpoint_witness_wire, preflight_checkpoint_witness_wire,
    types::DOMAIN,
};
use crate::recovery::encoding::{
    append_length_prefixed::append_length_prefixed,
    encode_native_data_frame::encode_native_data_frame,
};
use eve_state::{StateBudget, encode_block_payload, encode_state_version};

/// Caller charges input, canonical codec scratch, component buffers and final output separately.
pub fn encode_checkpoint_witness_wire(
    witness: &CheckpointWitness,
    budget: &StateBudget,
    limits: CheckpointLimits,
) -> Result<Vec<u8>, CheckpointWitnessWireError> {
    let size = measure_checkpoint_witness_wire(witness, budget, limits)?;
    let (kind, native) = match witness {
        CheckpointWitness::Execution(execution) => (1_u8, &execution.native),
        CheckpointWitness::Lookahead(native) => (2_u8, native.as_ref()),
    };
    let native_bytes =
        encode_native_data_frame(native).map_err(CheckpointWitnessWireError::Recovery)?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(size)
        .map_err(|_| CheckpointWitnessWireError::AllocationFailed)?;
    output.extend_from_slice(DOMAIN);
    output.push(kind);
    append_length_prefixed(&mut output, &native_bytes)
        .map_err(CheckpointWitnessWireError::Recovery)?;
    if let CheckpointWitness::Execution(execution) = witness {
        let version =
            encode_state_version(&execution.version).map_err(CheckpointWitnessWireError::State)?;
        let block = encode_block_payload(&execution.block, budget)
            .map_err(CheckpointWitnessWireError::State)?;
        append_length_prefixed(&mut output, &version)
            .map_err(CheckpointWitnessWireError::Recovery)?;
        append_length_prefixed(&mut output, &block)
            .map_err(CheckpointWitnessWireError::Recovery)?;
    }
    if output.len() != size {
        return Err(CheckpointWitnessWireError::NonCanonicalEncoding);
    }
    preflight_checkpoint_witness_wire(&output, budget, limits)?;
    Ok(output)
}
