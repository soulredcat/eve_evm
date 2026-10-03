// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::prepare_empty_recovery;
use crate::sync::applied::{
    AppliedConfig, AppliedError,
    resources::{
        EstimatedWorkingPool, estimate_replay_charge, estimated_repository_read_charge,
        reserve_estimated_working, split_estimated_working,
    },
    types::ChargedRecoveryState,
};
use eve_evm::estimate_clone_reservation;
use eve_finality_verifier::{into_recovery_state, recovery_state_commit};
use eve_storage::records::{OpaqueRecordRepository, opaque_record_cursor, read_opaque_record};
use std::sync::Arc;

pub(in crate::sync::applied) fn recover_applied_prefix(
    config: &AppliedConfig,
    repository: &OpaqueRecordRepository,
    mut generation: Arc<ChargedRecoveryState>,
    working: &Arc<EstimatedWorkingPool>,
) -> Result<Arc<ChargedRecoveryState>, AppliedError> {
    let head = opaque_record_cursor(repository).map_err(|_| AppliedError::StorageUnavailable)?;
    for sequence in 1..=head.sequence {
        let oracle = estimate_clone_reservation(&recovery_state_commit(&generation.recovery).state)
            .map_err(|_| AppliedError::EstimatedCapacity)?;
        let charge = estimate_replay_charge(
            &config.state_budget,
            config.maximum_recovery_payload_bytes,
            oracle,
        )?;
        let total = charge
            .total
            .checked_add(estimated_repository_read_charge(&config.repository_budget)?)
            .ok_or(AppliedError::ArithmeticOverflow)?;
        let lease = reserve_estimated_working(working, total)?;
        let record = read_opaque_record(repository, sequence)
            .map_err(|_| AppliedError::StorageUnavailable)?
            .ok_or(AppliedError::InvalidDurablePrefix)?;
        if record.payload.is_empty() || record.payload.len() > config.maximum_recovery_payload_bytes
        {
            return Err(AppliedError::PayloadLimit);
        }
        let transition =
            prepare_empty_recovery(&generation, &record.payload, &config.state_budget, oracle)?;
        let recovery = into_recovery_state(transition);
        if recovery_state_commit(&recovery).target.height != sequence
            || record.sequence != sequence
            || (sequence == head.sequence && record.content_hash != head.content_hash)
        {
            return Err(AppliedError::InvalidDurablePrefix);
        }
        let (retained, transient) = split_estimated_working(lease, charge.retained)?;
        generation = Arc::new(ChargedRecoveryState {
            recovery,
            _lease: retained,
        });
        drop(record);
        drop(transient);
    }
    if recovery_state_commit(&generation.recovery).target.height != head.sequence {
        return Err(AppliedError::InvalidDurablePrefix);
    }
    Ok(generation)
}
