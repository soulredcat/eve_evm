// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{CheckpointLimits, CheckpointWitness};
use super::{
    CheckpointWitnessWireError, types::DOMAIN,
    validate_checkpoint_witness_wire_limits::validate_checkpoint_witness_wire_limits,
};
use crate::recovery::bounds::{
    measure_execution_payload_bytes::measure_execution_payload_bytes,
    measure_native_frame_bytes::measure_native_frame_bytes,
    measure_transaction_list_bytes::measure_transaction_list_bytes,
    types::{MAXIMUM_EXECUTION_DATA_BYTES, MAXIMUM_VERSION_BYTES},
};
use eve_state::{StateBudget, encode_state_version};

/// Exact aggregate admission before large component buffers; bounded version encoding still needs caller scratch.
pub fn measure_checkpoint_witness_wire(
    witness: &CheckpointWitness,
    budget: &StateBudget,
    limits: CheckpointLimits,
) -> Result<usize, CheckpointWitnessWireError> {
    validate_checkpoint_witness_wire_limits(budget, limits)?;
    let native = match witness {
        CheckpointWitness::Execution(execution) => &execution.native,
        CheckpointWitness::Lookahead(native) => native.as_ref(),
    };
    if native.transactions.len() > budget.maximum_journal_operations {
        return Err(CheckpointWitnessWireError::BudgetExceeded);
    }
    let native_bytes = measure_native_frame_bytes(&native.frame)
        .map_err(CheckpointWitnessWireError::Recovery)?
        .checked_add(
            measure_transaction_list_bytes(&native.transactions)
                .map_err(CheckpointWitnessWireError::Recovery)?,
        )
        .and_then(|length| length.checked_add(4))
        .ok_or(CheckpointWitnessWireError::ArithmeticOverflow)?;
    let mut total = DOMAIN
        .len()
        .checked_add(1 + 4)
        .and_then(|length| length.checked_add(native_bytes))
        .ok_or(CheckpointWitnessWireError::ArithmeticOverflow)?;
    if let CheckpointWitness::Execution(execution) = witness {
        let version =
            encode_state_version(&execution.version).map_err(CheckpointWitnessWireError::State)?;
        let block = measure_execution_payload_bytes(&execution.block)
            .map_err(CheckpointWitnessWireError::Recovery)?;
        if version.len() > MAXIMUM_VERSION_BYTES
            || block > MAXIMUM_EXECUTION_DATA_BYTES
            || block > budget.maximum_commit_bytes
            || execution.block.transactions.len() > budget.maximum_journal_operations
        {
            return Err(CheckpointWitnessWireError::BudgetExceeded);
        }
        total = total
            .checked_add(8)
            .and_then(|length| length.checked_add(version.len()))
            .and_then(|length| length.checked_add(block))
            .ok_or(CheckpointWitnessWireError::ArithmeticOverflow)?;
    }
    if total > limits.maximum_witness_bytes {
        return Err(CheckpointWitnessWireError::BudgetExceeded);
    }
    Ok(total)
}
