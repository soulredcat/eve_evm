// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::CheckpointLimits;
use super::{
    CheckpointMessageError, CheckpointMessageLimits,
    validate_checkpoint_message_limits::validate_checkpoint_message_limits,
};
pub fn checkpoint_message_storage_limits(
    limits: &CheckpointMessageLimits,
    chunk_bytes: usize,
) -> Result<CheckpointLimits, CheckpointMessageError> {
    validate_checkpoint_message_limits(limits)?;
    if chunk_bytes == 0 || chunk_bytes > limits.maximum_chunk_bytes {
        return Err(CheckpointMessageError::BudgetExceeded);
    }
    Ok(CheckpointLimits {
        logical: limits.logical,
        maximum_body_bytes: limits.maximum_body_bytes,
        maximum_chunk_bytes: chunk_bytes,
        maximum_chunks: 4_096,
        maximum_manifest_bytes: limits.maximum_manifest_bytes,
    })
}
