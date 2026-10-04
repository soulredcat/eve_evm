// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use eve_state::encode_state_version;
use eve_storage::checkpoints::{
    checkpoint_manifest_stats,
    messages::{
        CheckpointMessageLimits, CheckpointRequest, CheckpointRequestKind, CheckpointResponse,
        checkpoint_message_storage_limits,
    },
    preflight_checkpoint_manifest,
};

/// Configured equality and requested framing only; returned history is still untrusted.
pub(super) fn validate_checkpoint_response(
    request: &CheckpointRequest,
    response: &CheckpointResponse,
    limits: &CheckpointMessageLimits,
) -> Result<()> {
    let target = match response {
        CheckpointResponse::Manifest { target, .. }
        | CheckpointResponse::Chunk { target, .. }
        | CheckpointResponse::Execution { target, .. } => target,
    };
    ensure!(target.height == request.height, "SYNC_CHECKPOINT_HEIGHT");
    ensure!(
        target.identity == request.genesis.identity,
        "SYNC_CHECKPOINT_IDENTITY"
    );
    match (&request.kind, response) {
        (
            CheckpointRequestKind::Manifest { chunk_bytes },
            CheckpointResponse::Manifest {
                manifest, target, ..
            },
        ) => {
            let storage_limits = checkpoint_message_storage_limits(limits, *chunk_bytes as usize)
                .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_LIMITS"))?;
            let target_bytes = encode_state_version(target)
                .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_VERSION"))?;
            let preflight = preflight_checkpoint_manifest(manifest, &target_bytes, &storage_limits)
                .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_MANIFEST"))?;
            ensure!(
                checkpoint_manifest_stats(&preflight).chunk_bytes == *chunk_bytes as usize,
                "SYNC_CHECKPOINT_CHUNK_WIDTH"
            );
        }
        (
            CheckpointRequestKind::Chunk {
                chunk_bytes,
                manifest_id,
                index,
            },
            CheckpointResponse::Chunk {
                chunk_bytes: width,
                manifest_id: id,
                index: actual,
                ..
            },
        ) => {
            ensure!(
                chunk_bytes == width && manifest_id == id && index == actual,
                "SYNC_CHECKPOINT_CHUNK_IDENTITY"
            );
        }
        (CheckpointRequestKind::Execution, CheckpointResponse::Execution { target, block }) => {
            ensure!(
                block.header.number == target.height,
                "SYNC_CHECKPOINT_EXECUTION_HEIGHT"
            );
        }
        _ => anyhow::bail!("SYNC_CHECKPOINT_KIND"),
    }
    Ok(())
}
