// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    AppliedConfig, AppliedError,
    resources::{
        estimate_pending_metadata, estimate_replay_charge, estimated_clone_ceiling,
        estimated_repository_read_charge,
    },
};
use eve_node_policy::validate_public_budget;
use eve_storage::records::validate_opaque_record_budget;

pub(super) fn validate_applied_configuration(config: &AppliedConfig) -> Result<(), AppliedError> {
    validate_public_budget(config.public_budget).map_err(|_| AppliedError::InvalidConfiguration)?;
    validate_opaque_record_budget(&config.repository_budget)
        .map_err(|_| AppliedError::InvalidConfiguration)?;
    let state = config.state_budget;
    if [
        state.maximum_accounts,
        state.maximum_storage_slots,
        state.maximum_codes,
        state.maximum_code_bytes,
        state.maximum_total_code_bytes,
        state.maximum_system_records,
        state.maximum_system_bytes,
        state.maximum_block_hashes,
        state.maximum_journal_operations,
        state.maximum_journal_bytes,
        state.maximum_state_bytes,
        state.maximum_commit_bytes,
    ]
    .contains(&0)
        || config.maximum_recovery_payload_bytes == 0
        || config.worker_scratch_limit == 0
        || state.maximum_code_bytes > 24_576
        || state.maximum_code_bytes > state.maximum_total_code_bytes
        || state.maximum_total_code_bytes > state.maximum_state_bytes
        || state.maximum_system_bytes > state.maximum_state_bytes
        || state.maximum_state_bytes > state.maximum_commit_bytes
    {
        return Err(AppliedError::InvalidConfiguration);
    }
    let encoded = config
        .maximum_recovery_payload_bytes
        .checked_add(88)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let payload = u64::try_from(config.maximum_recovery_payload_bytes)
        .map_err(|_| AppliedError::ArithmeticOverflow)?;
    if payload > config.public_budget.maximum_record_bytes
        || payload > config.public_budget.maximum_batch_bytes
        || encoded > config.repository_budget.maximum_record_bytes
        || encoded > config.repository_budget.maximum_read_bytes
    {
        return Err(AppliedError::InvalidConfiguration);
    }
    let charge = estimate_replay_charge(
        &state,
        config.maximum_recovery_payload_bytes,
        estimated_clone_ceiling(&state)?,
    )?;
    let queue_count = usize::try_from(config.public_budget.queue_batches)
        .map_err(|_| AppliedError::ArithmeticOverflow)?;
    let metadata = estimate_pending_metadata(queue_count)?
        .checked_mul(2)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let with_parent = charge
        .total
        .checked_add(charge.retained)
        .and_then(|bytes| {
            bytes.checked_add(estimated_repository_read_charge(&config.repository_budget).ok()?)
        })
        .and_then(|bytes| bytes.checked_add(metadata))
        .ok_or(AppliedError::ArithmeticOverflow)?;
    if u64::try_from(with_parent).map_err(|_| AppliedError::ArithmeticOverflow)?
        > config.public_budget.maximum_working_state_bytes
    {
        return Err(AppliedError::EstimatedCapacity);
    }
    Ok(())
}
