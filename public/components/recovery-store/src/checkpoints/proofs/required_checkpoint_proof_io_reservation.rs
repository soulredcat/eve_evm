// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofLimits, validate_checkpoint_proof_limits::validate_checkpoint_proof_limits,
};
use crate::checkpoints::CheckpointError;

/// One streaming 64 KiB scratch plus bounded metadata; incoming blob bytes remain caller-charged.
pub fn required_checkpoint_proof_io_reservation(
    limits: &CheckpointProofLimits,
) -> Result<usize, CheckpointError> {
    validate_checkpoint_proof_limits(limits)?;
    Ok(65_536 + 4_096)
}
