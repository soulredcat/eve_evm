// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ChargedCheckpointTransfer, CheckpointAppliedError};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_storage::checkpoints::{
    repair_invalid_checkpoint_chunk, required_checkpoint_io_reservation,
};

/// Explicitly repair one rechecked invalid unpublished staging entry; valid/completed data refuse.
pub fn repair_applied_checkpoint_chunk(
    transfer: &mut ChargedCheckpointTransfer,
    index: usize,
) -> Result<(), CheckpointAppliedError> {
    let required = required_checkpoint_io_reservation(&transfer.limits.content)
        .map_err(CheckpointAppliedError::Storage)?;
    let _storage = crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
        &transfer.storage,
        required,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let _io = reserve_estimated_working(&transfer.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    repair_invalid_checkpoint_chunk(&mut transfer.transfer, index, required)
        .map_err(CheckpointAppliedError::Storage)
}
