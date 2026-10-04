// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ChargedCheckpointProofTransfer, CheckpointAppliedError};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_storage::checkpoints::proofs::{
    required_checkpoint_proof_io_reservation, write_checkpoint_proof_witness,
};

/// The caller's raw witness lease and our local IO lease overlap throughout file synchronization.
pub fn write_applied_checkpoint_witness(
    transfer: &mut ChargedCheckpointProofTransfer,
    index: usize,
    bytes: &[u8],
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
    write_checkpoint_proof_witness(&mut transfer.transfer, index, bytes, required)
        .map_err(CheckpointAppliedError::Storage)
}
