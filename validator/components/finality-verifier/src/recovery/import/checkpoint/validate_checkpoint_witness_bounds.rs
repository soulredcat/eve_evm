// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointError, CheckpointSession, CheckpointWitness};
use crate::recovery::bounds::{
    measure_execution_payload_bytes::measure_execution_payload_bytes,
    measure_native_frame_bytes::measure_native_frame_bytes,
    measure_transaction_list_bytes::measure_transaction_list_bytes,
};
use eve_state::encode_state_version;

pub(super) fn validate_checkpoint_witness_bounds(
    session: &CheckpointSession,
    witness: &CheckpointWitness,
) -> Result<(), CheckpointError> {
    let native = match witness {
        CheckpointWitness::Execution(execution) => &execution.native,
        CheckpointWitness::Lookahead(native) => native.as_ref(),
    };
    let native_bytes = measure_native_frame_bytes(&native.frame)
        .map_err(CheckpointError::Recovery)?
        .checked_add(
            measure_transaction_list_bytes(&native.transactions)
                .map_err(CheckpointError::Recovery)?,
        )
        .ok_or(CheckpointError::Overflow)?;
    let extra = match witness {
        CheckpointWitness::Execution(execution) => {
            measure_execution_payload_bytes(&execution.block)
                .map_err(CheckpointError::Recovery)?
                .checked_add(
                    encode_state_version(&execution.version)
                        .map_err(CheckpointError::State)?
                        .len(),
                )
                .ok_or(CheckpointError::Overflow)?
        }
        CheckpointWitness::Lookahead(_) => 0,
    };
    if native_bytes
        .checked_add(extra)
        .ok_or(CheckpointError::Overflow)?
        > session.limits.maximum_witness_bytes
    {
        return Err(CheckpointError::InvalidLimit);
    }
    Ok(())
}
