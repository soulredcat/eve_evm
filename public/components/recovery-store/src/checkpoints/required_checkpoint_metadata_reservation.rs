// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointLimits, validate_checkpoint_limits::validate_checkpoint_limits,
};

/// Caller holds a real lease through manifest copies, retained handles and directory iteration.
pub fn required_checkpoint_metadata_reservation(
    limits: &CheckpointLimits,
) -> Result<usize, CheckpointError> {
    validate_checkpoint_limits(limits)?;
    limits
        .maximum_manifest_bytes
        .checked_mul(3)
        .and_then(|bytes| {
            bytes.checked_add(65_536 + 8_192 + std::mem::size_of::<super::CheckpointTransfer>())
        })
        .ok_or(CheckpointError::ResourceReservation)
}
