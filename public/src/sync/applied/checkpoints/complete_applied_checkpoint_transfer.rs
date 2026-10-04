// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ChargedCheckpointTransfer, ChargedCompletedCheckpoint, CheckpointAppliedError};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_storage::checkpoints::{complete_checkpoint_transfer, required_checkpoint_io_reservation};

pub fn complete_applied_checkpoint_transfer(
    transfer: ChargedCheckpointTransfer,
) -> Result<ChargedCompletedCheckpoint, CheckpointAppliedError> {
    let required = required_checkpoint_io_reservation(&transfer.limits.content)
        .map_err(CheckpointAppliedError::Storage)?;
    let _storage = crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
        &transfer.storage,
        required,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let _io = reserve_estimated_working(&transfer.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    let store = complete_checkpoint_transfer(transfer.transfer, required)
        .map_err(CheckpointAppliedError::Storage)?;
    Ok(ChargedCompletedCheckpoint {
        content_store: store,
        storage: transfer.storage,
        parent: transfer.parent,
        working: transfer.working,
        limits: transfer.limits,
        content_id: transfer.content_id,
        content: transfer.content,
        _metadata: transfer._metadata,
        _staging: transfer._staging,
    })
}
