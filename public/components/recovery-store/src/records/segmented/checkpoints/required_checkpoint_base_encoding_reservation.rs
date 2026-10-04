// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CHECKPOINT_BASE_MAX_VERSION_BYTES, CheckpointBaseError, CheckpointBaseLimits,
    validate_checkpoint_base_limits::validate_checkpoint_base_limits,
};

/// Canonical version codec scratch, retained target, payload and fixed control envelope.
/// Numeric admission does not acquire a lease or demonstrate allocator/RSS bounds.
pub fn required_checkpoint_base_encoding_reservation(
    limits: &CheckpointBaseLimits,
) -> Result<usize, CheckpointBaseError> {
    validate_checkpoint_base_limits(limits)?;
    CHECKPOINT_BASE_MAX_VERSION_BYTES
        .checked_mul(8)
        .and_then(|bytes| bytes.checked_add(limits.maximum_payload_bytes))
        .and_then(|bytes| bytes.checked_add(4_096))
        .ok_or(CheckpointBaseError::ArithmeticOverflow)
}
