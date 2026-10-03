// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::prepare_charged_generation;
use crate::sync::applied::{
    AppliedConfig, AppliedError,
    resources::{
        EstimatedWorkingPool, estimated_repository_read_charge, reserve_estimated_working,
    },
    state::applied_state_commit,
    types::ChargedAppliedState,
};
use eve_storage::records::{OpaqueRecordRepository, opaque_record_cursor, read_opaque_record};
use std::sync::Arc;

pub(in crate::sync::applied) fn recover_applied_prefix(
    config: &AppliedConfig,
    repository: &OpaqueRecordRepository,
    mut generation: Arc<ChargedAppliedState>,
    working: &Arc<EstimatedWorkingPool>,
) -> Result<Arc<ChargedAppliedState>, AppliedError> {
    let head = opaque_record_cursor(repository).map_err(|_| AppliedError::StorageUnavailable)?;
    for sequence in 1..=head.sequence {
        let read_lease = reserve_estimated_working(
            working,
            estimated_repository_read_charge(&config.repository_budget)?,
        )?;
        let record = read_opaque_record(repository, sequence)
            .map_err(|_| AppliedError::StorageUnavailable)?
            .ok_or(AppliedError::InvalidDurablePrefix)?;
        if record.payload.is_empty() || record.payload.len() > config.maximum_recovery_payload_bytes
        {
            return Err(AppliedError::PayloadLimit);
        }
        let prepared = prepare_charged_generation(&generation, &record.payload, config, working)?;
        if applied_state_commit(&prepared.state).target.height != sequence
            || record.sequence != sequence
            || (sequence == head.sequence && record.content_hash != head.content_hash)
        {
            return Err(AppliedError::InvalidDurablePrefix);
        }
        generation = prepared;
        drop(record);
        drop(read_lease);
    }
    if applied_state_commit(&generation.state).target.height != head.sequence {
        return Err(AppliedError::InvalidDurablePrefix);
    }
    Ok(generation)
}
