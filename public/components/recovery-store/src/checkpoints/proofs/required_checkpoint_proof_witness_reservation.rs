// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CompletedCheckpointProofStore,
    read_checkpoint_proof_reference::read_checkpoint_proof_reference,
    required_checkpoint_proof_io_reservation,
};
use crate::checkpoints::CheckpointError;

pub fn required_checkpoint_proof_witness_reservation(
    store: &CompletedCheckpointProofStore,
    index: usize,
) -> Result<usize, CheckpointError> {
    read_checkpoint_proof_reference(&store.transfer, index)?
        .length
        .checked_add(required_checkpoint_proof_io_reservation(
            &store.transfer.limits,
        )?)
        .ok_or(CheckpointError::ResourceReservation)
}
