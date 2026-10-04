// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointMessageError, CheckpointRequest, CheckpointRequestKind,
    append_checkpoint_bytes::append_checkpoint_bytes, types::REQUEST_DOMAIN,
    validate_checkpoint_request::validate_checkpoint_request,
};
use eve_state::encode_state_version;
pub fn encode_checkpoint_request(
    request: &CheckpointRequest,
) -> Result<Vec<u8>, CheckpointMessageError> {
    validate_checkpoint_request(request)?;
    let genesis = encode_state_version(&request.genesis).map_err(CheckpointMessageError::State)?;
    if genesis.len() > 4_096 {
        return Err(CheckpointMessageError::BudgetExceeded);
    }
    let tag = match request.kind {
        CheckpointRequestKind::Manifest { .. } => 1,
        CheckpointRequestKind::Chunk { .. } => 2,
        CheckpointRequestKind::Execution => 3,
    };
    let mut bytes = Vec::with_capacity(4_096 + 128);
    bytes.extend_from_slice(REQUEST_DOMAIN);
    bytes.extend_from_slice(&[1, 0, tag]);
    append_checkpoint_bytes(&mut bytes, &genesis)?;
    bytes.extend_from_slice(&request.height.to_be_bytes());
    match request.kind {
        CheckpointRequestKind::Manifest { chunk_bytes } => {
            bytes.extend_from_slice(&chunk_bytes.to_be_bytes())
        }
        CheckpointRequestKind::Chunk {
            chunk_bytes,
            manifest_id,
            index,
        } => {
            bytes.extend_from_slice(&chunk_bytes.to_be_bytes());
            bytes.extend_from_slice(&manifest_id);
            bytes.extend_from_slice(&index.to_be_bytes());
        }
        CheckpointRequestKind::Execution => {}
    }
    Ok(bytes)
}
