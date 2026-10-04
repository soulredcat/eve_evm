// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointError, CheckpointLimits, MAXIMUM_CHECKPOINT_CHUNK_BYTES};

pub(super) fn validate_checkpoint_limits(limits: &CheckpointLimits) -> Result<(), CheckpointError> {
    eve_state::validate_state_budget(&limits.logical).map_err(CheckpointError::State)?;
    if limits.maximum_body_bytes == 0
        || limits.maximum_body_bytes > limits.logical.maximum_commit_bytes
        || limits.maximum_chunk_bytes == 0
        || limits.maximum_chunk_bytes > MAXIMUM_CHECKPOINT_CHUNK_BYTES
        || limits.maximum_chunks == 0
        || limits.maximum_chunks > 4_096
        || limits.maximum_manifest_bytes < super::types::HEADER_BYTES
        || limits.maximum_manifest_bytes > 262_144
    {
        return Err(CheckpointError::InvalidLimits);
    }
    Ok(())
}
