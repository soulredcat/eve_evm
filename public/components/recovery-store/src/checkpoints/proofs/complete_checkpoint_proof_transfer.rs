// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofTransfer, CompletedCheckpointProofStore,
    checkpoint_proof_completion_bytes::checkpoint_proof_completion_bytes,
    required_checkpoint_proof_io_reservation,
    validate_checkpoint_proof_witnesses::validate_checkpoint_proof_witnesses,
};
use crate::checkpoints::{
    CheckpointError, publish_checkpoint_metadata::publish_checkpoint_metadata,
};

pub fn complete_checkpoint_proof_transfer(
    transfer: CheckpointProofTransfer,
    reserved_io: usize,
) -> Result<CompletedCheckpointProofStore, CheckpointError> {
    if reserved_io < required_checkpoint_proof_io_reservation(&transfer.limits)? {
        return Err(CheckpointError::ResourceReservation);
    }
    validate_checkpoint_proof_witnesses(&transfer)?;
    publish_checkpoint_metadata(
        &transfer.directory,
        "complete.bin",
        "complete.pending",
        &checkpoint_proof_completion_bytes(&transfer.summary),
    )?;
    Ok(CompletedCheckpointProofStore { transfer })
}
