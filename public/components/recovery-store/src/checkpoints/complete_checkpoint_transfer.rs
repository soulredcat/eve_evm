// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointTransfer, CompletedCheckpointStore,
    checkpoint_completion_bytes::checkpoint_completion_bytes,
    publish_checkpoint_metadata::publish_checkpoint_metadata, required_checkpoint_io_reservation,
    validate_checkpoint_chunks::validate_checkpoint_chunks,
};

pub fn complete_checkpoint_transfer(
    transfer: CheckpointTransfer,
    reserved_io: usize,
) -> Result<CompletedCheckpointStore, CheckpointError> {
    if reserved_io < required_checkpoint_io_reservation(&transfer.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    validate_checkpoint_chunks(&transfer)?;
    let bytes = checkpoint_completion_bytes(&transfer.summary);
    publish_checkpoint_metadata(
        &transfer.directory,
        "complete.bin",
        "complete.pending",
        &bytes,
    )?;
    Ok(CompletedCheckpointStore { transfer })
}
