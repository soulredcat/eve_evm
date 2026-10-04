// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ChargedCheckpointTransfer, CheckpointAppliedError};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_storage::checkpoints::{
    CheckpointChunkStatus, observe_checkpoint_chunk, required_checkpoint_io_reservation,
};

pub fn observe_applied_checkpoint_chunk(
    transfer: &ChargedCheckpointTransfer,
    index: usize,
) -> Result<CheckpointChunkStatus, CheckpointAppliedError> {
    let required = required_checkpoint_io_reservation(&transfer.limits.content)
        .map_err(CheckpointAppliedError::Storage)?;
    let _storage = crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
        &transfer.storage,
        required,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let _io = reserve_estimated_working(&transfer.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    observe_checkpoint_chunk(&transfer.transfer, index, required)
        .map_err(CheckpointAppliedError::Storage)
}
