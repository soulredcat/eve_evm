// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CHECKPOINT_BASE_MAX_PAYLOAD_BYTES, CheckpointBaseError, CheckpointBaseLimits,
    types::HEADER_BYTES,
};

pub(super) fn validate_checkpoint_base_limits(
    limits: &CheckpointBaseLimits,
) -> Result<(), CheckpointBaseError> {
    if limits.maximum_payload_bytes <= HEADER_BYTES + 32
        || limits.maximum_payload_bytes > CHECKPOINT_BASE_MAX_PAYLOAD_BYTES
    {
        return Err(CheckpointBaseError::InvalidLimits);
    }
    Ok(())
}
