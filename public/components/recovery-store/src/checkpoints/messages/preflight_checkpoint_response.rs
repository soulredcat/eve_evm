// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointMessageError, CheckpointMessageLimits, CheckpointResponseKind,
    CheckpointResponsePreflight, CheckpointResponseStats, checkpoint_message_storage_limits,
    take_checkpoint_bytes::take_checkpoint_bytes, take_checkpoint_field::take_checkpoint_field,
    take_checkpoint_prefix::take_checkpoint_prefix, types::RESPONSE_DOMAIN,
    validate_checkpoint_chunk_message::validate_checkpoint_chunk_message,
    validate_checkpoint_message_limits::validate_checkpoint_message_limits,
};
use eve_state::preflight_state_version;
use sha2::{Digest, Sha256};

/// Borrowed bound scan. Canonical BlockPayload semantic decode occurs only after the caller reservation.
pub fn preflight_checkpoint_response<'a>(
    bytes: &'a [u8],
    limits: &CheckpointMessageLimits,
) -> Result<CheckpointResponsePreflight<'a>, CheckpointMessageError> {
    validate_checkpoint_message_limits(limits)?;
    if bytes.len()
        > limits
            .maximum_body_bytes
            .checked_add(16_384)
            .ok_or(CheckpointMessageError::ArithmeticOverflow)?
    {
        return Err(CheckpointMessageError::BudgetExceeded);
    }
    let mut remaining = bytes;
    let tag = take_checkpoint_prefix(&mut remaining, RESPONSE_DOMAIN)?;
    let kind = match tag {
        1 => CheckpointResponseKind::Manifest,
        2 => CheckpointResponseKind::Chunk,
        3 => CheckpointResponseKind::Execution,
        _ => return Err(CheckpointMessageError::MalformedEncoding),
    };
    let target = take_checkpoint_bytes(&mut remaining, 4_096)?;
    let target_network_bytes =
        preflight_state_version(target).map_err(CheckpointMessageError::State)?;
    let mut tip = None;
    let mut tip_network_bytes = 0;
    let mut manifest_id = None;
    let mut body_hash = None;
    let mut total = 0;
    let mut index = 0;
    let mut width = 0;
    let payload = match kind {
        CheckpointResponseKind::Manifest => {
            let encoded_tip = take_checkpoint_bytes(&mut remaining, 4_096)?;
            tip_network_bytes =
                preflight_state_version(encoded_tip).map_err(CheckpointMessageError::State)?;
            tip = Some(encoded_tip);
            let id = take_checkpoint_field(&mut remaining)?;
            let payload = take_checkpoint_bytes(&mut remaining, limits.maximum_manifest_bytes)?;
            if <[u8; 32]>::from(Sha256::digest(payload)) != id {
                return Err(CheckpointMessageError::ManifestInvalid);
            }
            let storage_limits =
                checkpoint_message_storage_limits(limits, limits.maximum_chunk_bytes)?;
            super::super::preflight_checkpoint_manifest(payload, target, &storage_limits)
                .map_err(|_| CheckpointMessageError::ManifestInvalid)?;
            manifest_id = Some(id);
            payload
        }
        CheckpointResponseKind::Chunk => {
            manifest_id = Some(take_checkpoint_field(&mut remaining)?);
            body_hash = Some(take_checkpoint_field(&mut remaining)?);
            total = u64::from_be_bytes(take_checkpoint_field(&mut remaining)?);
            index = u32::from_be_bytes(take_checkpoint_field(&mut remaining)?);
            width = u32::from_be_bytes(take_checkpoint_field(&mut remaining)?);
            let payload = take_checkpoint_bytes(&mut remaining, limits.maximum_chunk_bytes)?;
            validate_checkpoint_chunk_message(total, index, width, payload.len(), limits)?;
            payload
        }
        CheckpointResponseKind::Execution => take_checkpoint_bytes(
            &mut remaining,
            limits
                .logical
                .maximum_commit_bytes
                .min(8_400_896)
                .min(limits.maximum_body_bytes),
        )?,
    };
    if !remaining.is_empty() {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    Ok(CheckpointResponsePreflight {
        bytes,
        limits: *limits,
        kind,
        target,
        tip,
        manifest_id,
        body_hash,
        total,
        index,
        width,
        payload,
        stats: CheckpointResponseStats {
            encoded_bytes: bytes.len(),
            target_network_bytes,
            tip_network_bytes,
            payload_bytes: payload.len(),
        },
    })
}
