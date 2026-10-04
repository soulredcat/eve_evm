// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofLimits, validate_checkpoint_proof_limits::validate_checkpoint_proof_limits,
};
use crate::checkpoints::CheckpointError;

pub fn required_checkpoint_proof_metadata_reservation(
    limits: &CheckpointProofLimits,
) -> Result<usize, CheckpointError> {
    validate_checkpoint_proof_limits(limits)?;
    limits
        .maximum_manifest_bytes
        .checked_mul(3)
        .and_then(|bytes| {
            bytes
                .checked_add(65_536 + 8_192 + std::mem::size_of::<super::CheckpointProofTransfer>())
        })
        .ok_or(CheckpointError::ResourceReservation)
}
