// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointMessageError, CheckpointMessageLimits, MAXIMUM_CHECKPOINT_MESSAGE_BODY_BYTES,
    MAXIMUM_CHECKPOINT_MESSAGE_CHUNK_BYTES, MAXIMUM_CHECKPOINT_MESSAGE_MANIFEST_BYTES,
};
pub(super) fn validate_checkpoint_message_limits(
    limits: &CheckpointMessageLimits,
) -> Result<(), CheckpointMessageError> {
    eve_state::validate_state_budget(&limits.logical).map_err(CheckpointMessageError::State)?;
    if limits.maximum_body_bytes == 0
        || limits.maximum_body_bytes > MAXIMUM_CHECKPOINT_MESSAGE_BODY_BYTES
        || limits.maximum_body_bytes > limits.logical.maximum_commit_bytes
        || limits.maximum_manifest_bytes == 0
        || limits.maximum_manifest_bytes > MAXIMUM_CHECKPOINT_MESSAGE_MANIFEST_BYTES
        || limits.maximum_chunk_bytes == 0
        || limits.maximum_chunk_bytes > MAXIMUM_CHECKPOINT_MESSAGE_CHUNK_BYTES
    {
        return Err(CheckpointMessageError::BudgetExceeded);
    }
    Ok(())
}
