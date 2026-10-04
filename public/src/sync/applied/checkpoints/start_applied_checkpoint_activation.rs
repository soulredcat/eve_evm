// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointAppliedError, PreparedAppliedCheckpoint,
    build_checkpoint_base_metadata::build_checkpoint_base_metadata,
    build_checkpoint_publication::build_checkpoint_publication, types::PendingCheckpointActivation,
    validate_checkpoint_activation_parent::validate_checkpoint_activation_parent,
};
use crate::{
    persistence::segmented::checkpoints::{
        checkpoint_record_cursor, seal_checkpoint_base_record, try_submit_checkpoint_base,
    },
    sync::applied::{AppliedOwner, state::applied_state_commit, types::AppliedBackend},
};
use eve_storage::records::segmented::checkpoints::{
    CHECKPOINT_BASE_MAX_PAYLOAD_BYTES, CheckpointBaseLimits,
};

/// The old RAM view remains published until the exact same-worker durable base acknowledgment.
pub fn start_applied_checkpoint_activation(
    owner: &mut AppliedOwner,
    prepared: PreparedAppliedCheckpoint,
) -> Result<(), CheckpointAppliedError> {
    validate_checkpoint_activation_parent(owner, &prepared)?;
    let metadata = build_checkpoint_base_metadata(owner, &prepared)?;
    let AppliedBackend::Segmented { worker, pool, .. } = &owner.backend else {
        return Err(CheckpointAppliedError::WrongMode);
    };
    let worker = worker.as_ref().ok_or(CheckpointAppliedError::Applied(
        crate::sync::applied::AppliedError::Closed,
    ))?;
    let record = seal_checkpoint_base_record(
        pool,
        metadata,
        &applied_state_commit(&prepared.generation.state).target,
        CheckpointBaseLimits {
            maximum_payload_bytes: CHECKPOINT_BASE_MAX_PAYLOAD_BYTES,
        },
    )
    .map_err(CheckpointAppliedError::Segmented)?;
    let publication = build_checkpoint_publication(&prepared, checkpoint_record_cursor(&record))?;
    // Allocate retained bookkeeping before the worker can write. Charges already surround it.
    let mut retained = Box::new(PendingCheckpointActivation {
        prepared,
        publication,
        record: record.clone(),
        ticket: None,
        acknowledged: None,
    });
    let ticket = try_submit_checkpoint_base(worker, owner.admitted_cursor, record)
        .map_err(|rejected| CheckpointAppliedError::Segmented(rejected.error))?;
    retained.ticket = Some(ticket);
    owner.checkpoint = Some(retained);
    Ok(())
}
