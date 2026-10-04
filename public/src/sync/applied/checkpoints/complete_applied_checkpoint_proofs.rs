// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ChargedCheckpointArtifacts, ChargedCheckpointProofTransfer, CheckpointAppliedError};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_storage::checkpoints::proofs::{
    complete_checkpoint_proof_transfer, required_checkpoint_proof_io_reservation,
};

pub fn complete_applied_checkpoint_proofs(
    transfer: ChargedCheckpointProofTransfer,
) -> Result<ChargedCheckpointArtifacts, CheckpointAppliedError> {
    let required = required_checkpoint_proof_io_reservation(&transfer.content.limits.proofs)
        .map_err(CheckpointAppliedError::Storage)?;
    let _storage = crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
        &transfer.content.storage,
        required,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let _io = reserve_estimated_working(&transfer.content.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    let proof_store = complete_checkpoint_proof_transfer(transfer.transfer, required)
        .map_err(CheckpointAppliedError::Storage)?;
    Ok(ChargedCheckpointArtifacts {
        content: transfer.content,
        proof_store,
        proof_id: transfer.proof_id,
        proofs: transfer.proofs,
        _metadata: transfer._metadata,
        _staging: transfer._staging,
    })
}
