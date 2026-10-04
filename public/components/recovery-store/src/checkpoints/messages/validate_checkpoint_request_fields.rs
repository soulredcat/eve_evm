// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointMessageError, CheckpointRequestKind, MAXIMUM_CHECKPOINT_MESSAGE_CHUNK_BYTES,
};
pub(super) fn validate_checkpoint_request_fields(
    height: u64,
    kind: &CheckpointRequestKind,
) -> Result<(), CheckpointMessageError> {
    if height == 0 || height > i64::MAX as u64 {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    let width = match *kind {
        CheckpointRequestKind::Manifest { chunk_bytes } => chunk_bytes,
        CheckpointRequestKind::Chunk {
            chunk_bytes, index, ..
        } => {
            if index >= 4_096 {
                return Err(CheckpointMessageError::BudgetExceeded);
            }
            chunk_bytes
        }
        CheckpointRequestKind::Execution => return Ok(()),
    };
    if width == 0 || width as usize > MAXIMUM_CHECKPOINT_MESSAGE_CHUNK_BYTES {
        return Err(CheckpointMessageError::BudgetExceeded);
    }
    Ok(())
}
