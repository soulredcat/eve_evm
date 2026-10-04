// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    check_checkpoint_bootstrap_deadline::check_checkpoint_bootstrap_deadline,
    checkpoint_rpc_before::checkpoint_rpc_before,
    reserve_checkpoint_ingress::reserve_checkpoint_ingress,
    types::{CHECKPOINT_CHUNK_BYTES, CheckpointDownloadContext, CheckpointManifestDownload},
};
use crate::sync::applied::checkpoints::{
    ChargedCheckpointTransfer, CheckpointAppliedError, observe_applied_checkpoint_chunk,
    repair_applied_checkpoint_chunk, write_applied_checkpoint_chunk,
};
use anyhow::{Result, ensure};
use eve_storage::checkpoints::{
    CheckpointChunkStatus, CheckpointError,
    messages::{CheckpointRequest, CheckpointRequestKind, CheckpointResponse},
};
use eve_sync_client::{downloaded_checkpoint_response, fetch_checkpoint_response_before};

pub(super) fn stage_checkpoint_content_chunk(
    transfer: &mut ChargedCheckpointTransfer,
    descriptor: &CheckpointManifestDownload,
    index: usize,
    context: &CheckpointDownloadContext<'_>,
) -> Result<()> {
    let deadline = context.deadline;
    check_checkpoint_bootstrap_deadline(deadline)?;
    match observe_applied_checkpoint_chunk(transfer, index) {
        Ok(CheckpointChunkStatus::Present) => return check_checkpoint_bootstrap_deadline(deadline),
        Ok(CheckpointChunkStatus::Missing) => {}
        Ok(CheckpointChunkStatus::Corrupt)
        | Err(CheckpointAppliedError::Storage(CheckpointError::UnsafeEntry)) => {
            repair_applied_checkpoint_chunk(transfer, index)
                .map_err(|error| anyhow::anyhow!("checkpoint staging repair: {error:?}"))?;
            check_checkpoint_bootstrap_deadline(deadline)?;
        }
        Err(error) => anyhow::bail!("checkpoint chunk observation: {error:?}"),
    }
    let CheckpointResponse::Manifest { target, .. } =
        downloaded_checkpoint_response(&descriptor.downloaded)
    else {
        anyhow::bail!("checkpoint manifest response expected");
    };
    let response = fetch_checkpoint_response_before(
        checkpoint_rpc_before(context.address, deadline)?,
        &CheckpointRequest {
            genesis: context.genesis.clone(),
            height: target.height,
            kind: CheckpointRequestKind::Chunk {
                chunk_bytes: CHECKPOINT_CHUNK_BYTES as u32,
                manifest_id: descriptor.id,
                index: u32::try_from(index)?,
            },
        },
        &context.message,
        &mut |bytes| reserve_checkpoint_ingress(context.reader, bytes),
        deadline,
    )?;
    let CheckpointResponse::Chunk {
        target: actual,
        body_sha256,
        total_length,
        data,
        ..
    } = downloaded_checkpoint_response(&response)
    else {
        anyhow::bail!("checkpoint chunk response expected");
    };
    ensure!(
        actual == target
            && *body_sha256 == descriptor.stats.body_sha256
            && usize::try_from(*total_length)? == descriptor.stats.body_bytes,
        "checkpoint chunk manifest binding changed"
    );
    write_applied_checkpoint_chunk(transfer, index, data)
        .map_err(|error| anyhow::anyhow!("checkpoint chunk sync: {error:?}"))?;
    check_checkpoint_bootstrap_deadline(deadline)
}
