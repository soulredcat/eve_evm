// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ChargedCheckpointTransfer, CheckpointAppliedError};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_storage::checkpoints::{required_checkpoint_io_reservation, write_checkpoint_chunk};

/// The caller retains its ingress lease while this independently charged sync operation runs.
pub fn write_applied_checkpoint_chunk(
    transfer: &mut ChargedCheckpointTransfer,
    index: usize,
    bytes: &[u8],
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
    write_checkpoint_chunk(&mut transfer.transfer, index, bytes, required)
        .map_err(CheckpointAppliedError::Storage)
}
