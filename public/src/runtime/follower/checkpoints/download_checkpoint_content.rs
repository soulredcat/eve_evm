// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    check_checkpoint_bootstrap_deadline::check_checkpoint_bootstrap_deadline,
    checkpoint_rpc_before::checkpoint_rpc_before,
    reserve_checkpoint_ingress::reserve_checkpoint_ingress,
    stage_checkpoint_content_chunk::stage_checkpoint_content_chunk,
    types::{CHECKPOINT_CHUNK_BYTES, CheckpointDownloadContext, CheckpointManifestDownload},
};
use crate::sync::applied::{
    AppliedOwner,
    checkpoints::{
        ChargedCompletedCheckpoint, begin_applied_checkpoint_transfer,
        complete_applied_checkpoint_transfer, repair_applied_checkpoint_content_pending,
    },
    reserve_applied_working,
};
use anyhow::{Result, ensure};
use eve_state::{BOUNDED_STATE_CODEC_SCRATCH_BYTES, encode_state_version};
use eve_storage::checkpoints::{
    checkpoint_manifest_id, checkpoint_manifest_stats,
    messages::{CheckpointRequest, CheckpointRequestKind, CheckpointResponse},
    preflight_checkpoint_manifest,
};
use eve_sync_client::{downloaded_checkpoint_response, fetch_checkpoint_response_before};
use std::fs::File;

pub(super) fn download_checkpoint_content(
    owner: &AppliedOwner,
    root: &File,
    height: u64,
    context: &CheckpointDownloadContext<'_>,
) -> Result<(ChargedCompletedCheckpoint, CheckpointManifestDownload)> {
    let deadline = context.deadline;
    let downloaded = fetch_checkpoint_response_before(
        checkpoint_rpc_before(context.address, deadline)?,
        &CheckpointRequest {
            genesis: context.genesis.clone(),
            height,
            kind: CheckpointRequestKind::Manifest {
                chunk_bytes: CHECKPOINT_CHUNK_BYTES as u32,
            },
        },
        &context.message,
        &mut |bytes| reserve_checkpoint_ingress(context.reader, bytes),
        deadline,
    )?;
    let CheckpointResponse::Manifest {
        target,
        manifest,
        manifest_id,
        ..
    } = downloaded_checkpoint_response(&downloaded)
    else {
        anyhow::bail!("checkpoint manifest response expected");
    };
    let _encoding = reserve_applied_working(context.reader, BOUNDED_STATE_CODEC_SCRATCH_BYTES)
        .map_err(|error| anyhow::anyhow!("checkpoint target encoding reservation: {error:?}"))?;
    let target_bytes = encode_state_version(target)
        .map_err(|error| anyhow::anyhow!("checkpoint target encoding: {error:?}"))?;
    let sealed = preflight_checkpoint_manifest(manifest, &target_bytes, &context.limits.content)
        .map_err(|error| anyhow::anyhow!("checkpoint manifest admission: {error:?}"))?;
    let id = checkpoint_manifest_id(&sealed);
    ensure!(id == *manifest_id, "checkpoint manifest identity changed");
    let stats = checkpoint_manifest_stats(&sealed);
    repair_applied_checkpoint_content_pending(owner, root, manifest, &target_bytes, context.limits)
        .map_err(|error| {
            anyhow::anyhow!("checkpoint pending content metadata refused: {error:?}")
        })?;
    check_checkpoint_bootstrap_deadline(deadline)?;
    let mut transfer =
        begin_applied_checkpoint_transfer(owner, root, manifest, &target_bytes, context.limits)
            .map_err(|error| anyhow::anyhow!("checkpoint content staging: {error:?}"))?;
    check_checkpoint_bootstrap_deadline(deadline)?;
    let descriptor = CheckpointManifestDownload {
        downloaded,
        id,
        stats,
    };
    for index in 0..stats.chunks {
        stage_checkpoint_content_chunk(&mut transfer, &descriptor, index, context)?;
    }
    let complete = complete_applied_checkpoint_transfer(transfer)
        .map_err(|error| anyhow::anyhow!("checkpoint content completion: {error:?}"))?;
    check_checkpoint_bootstrap_deadline(deadline)?;
    Ok((complete, descriptor))
}
