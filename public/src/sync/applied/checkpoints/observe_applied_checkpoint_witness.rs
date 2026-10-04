// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ChargedCheckpointProofTransfer, CheckpointAppliedError};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_storage::checkpoints::{
    CheckpointChunkStatus,
    proofs::{observe_checkpoint_proof_witness, required_checkpoint_proof_io_reservation},
};

pub fn observe_applied_checkpoint_witness(
    transfer: &ChargedCheckpointProofTransfer,
    index: usize,
) -> Result<CheckpointChunkStatus, CheckpointAppliedError> {
    let required = required_checkpoint_proof_io_reservation(&transfer.content.limits.proofs)
        .map_err(CheckpointAppliedError::Storage)?;
    let _storage = crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
        &transfer.content.storage,
        required,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let _io = reserve_estimated_working(&transfer.content.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    observe_checkpoint_proof_witness(&transfer.transfer, index, required)
        .map_err(CheckpointAppliedError::Storage)
}
