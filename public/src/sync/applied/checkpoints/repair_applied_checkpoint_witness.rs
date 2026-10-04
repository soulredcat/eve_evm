// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ChargedCheckpointProofTransfer, CheckpointAppliedError};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_storage::checkpoints::proofs::{
    repair_invalid_checkpoint_proof_witness, required_checkpoint_proof_io_reservation,
};

/// The storage primitive rechecks exclusive ownership, completion absence and exact invalid entry.
pub fn repair_applied_checkpoint_witness(
    transfer: &mut ChargedCheckpointProofTransfer,
    index: usize,
) -> Result<(), CheckpointAppliedError> {
    let required = required_checkpoint_proof_io_reservation(&transfer.content.limits.proofs)
        .map_err(CheckpointAppliedError::Storage)?;
    let _storage = crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
        &transfer.content.storage,
        required,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let _io = reserve_estimated_working(&transfer.content.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    repair_invalid_checkpoint_proof_witness(&mut transfer.transfer, index, required)
        .map_err(CheckpointAppliedError::Storage)
}
