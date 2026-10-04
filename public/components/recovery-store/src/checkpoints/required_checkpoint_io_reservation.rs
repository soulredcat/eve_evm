// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointLimits, validate_checkpoint_limits::validate_checkpoint_limits,
};

/// Conservative logical IO/scratch charge; incoming chunk storage is a separate caller charge.
pub fn required_checkpoint_io_reservation(
    limits: &CheckpointLimits,
) -> Result<usize, CheckpointError> {
    validate_checkpoint_limits(limits)?;
    limits
        .maximum_chunk_bytes
        .checked_add(65_536 + 4_096)
        .ok_or(CheckpointError::ResourceReservation)
}
