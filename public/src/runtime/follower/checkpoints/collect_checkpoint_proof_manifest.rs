// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    check_checkpoint_bootstrap_deadline::check_checkpoint_bootstrap_deadline,
    checkpoint_rpc_before::checkpoint_rpc_before,
    reserve_checkpoint_ingress::reserve_checkpoint_ingress,
    types::{
        ChargedCheckpointProofManifest, CheckpointDownloadContext, CheckpointManifestDownload,
    },
};
use anyhow::{Context, Result, ensure};
use eve_storage::checkpoints::{
    messages::CheckpointResponse,
    proofs::{
        CheckpointProofKind, CheckpointProofReferenceInput, begin_checkpoint_proof_stream_hash,
        create_checkpoint_proof_manifest, finish_checkpoint_proof_stream_hash,
        required_checkpoint_proof_metadata_reservation, update_checkpoint_proof_stream_hash,
    },
};
use eve_sync_client::{
    CheckpointWitnessHeights, downloaded_checkpoint_response, downloaded_checkpoint_witness_wire,
    fetch_checkpoint_witness_before,
};
use sha2::{Digest, Sha256};

/// First pass keeps only bounded references, never the downloaded witness history.
pub(super) fn collect_checkpoint_proof_manifest(
    descriptor: &CheckpointManifestDownload,
    context: &CheckpointDownloadContext<'_>,
) -> Result<ChargedCheckpointProofManifest> {
    let CheckpointResponse::Manifest { target, .. } =
        downloaded_checkpoint_response(&descriptor.downloaded)
    else {
        anyhow::bail!("checkpoint manifest response expected");
    };
    let count = usize::try_from(
        target
            .height
            .checked_add(1)
            .context("checkpoint proof height overflow")?,
    )?;
    ensure!(
        count <= context.limits.proofs.maximum_files,
        "checkpoint proof reference limit"
    );
    let metadata = required_checkpoint_proof_metadata_reservation(&context.limits.proofs)
        .map_err(|error| anyhow::anyhow!("checkpoint proof metadata limit: {error:?}"))?;
    let required = count
        .checked_mul(std::mem::size_of::<CheckpointProofReferenceInput>())
        .and_then(|bytes| bytes.checked_add(metadata))
        .context("checkpoint proof metadata overflow")?;
    let metadata_working = crate::sync::applied::reserve_applied_working(context.reader, required)
        .map_err(|error| {
            anyhow::anyhow!("checkpoint proof metadata working reservation: {error:?}")
        })?;
    let metadata_staging =
        crate::sync::applied::reserve_applied_snapshot_staging(context.reader, required).map_err(
            |error| anyhow::anyhow!("checkpoint proof metadata staging reservation: {error:?}"),
        )?;
    let lease = super::ingress_reservation_types::CheckpointIngressReservation {
        _working: metadata_working,
        _staging: metadata_staging,
    };
    let mut references = Vec::new();
    references.try_reserve_exact(count)?;
    ensure!(
        references.capacity() == count,
        "checkpoint reference allocation exceeded reservation"
    );
    let mut stream = begin_checkpoint_proof_stream_hash(
        &descriptor.id,
        &descriptor.stats.body_sha256,
        target,
        &context.limits.proofs,
        metadata,
    )
    .map_err(|error| anyhow::anyhow!("checkpoint proof stream: {error:?}"))?;
    for index in 0..count {
        let height = u64::try_from(index)?
            .checked_add(1)
            .context("checkpoint witness height overflow")?;
        let downloaded = fetch_checkpoint_witness_before(
            checkpoint_rpc_before(context.address, context.deadline)?,
            context.genesis,
            CheckpointWitnessHeights {
                checkpoint: target.height,
                witness: height,
            },
            &context.limits.content.logical,
            context.limits.verification,
            &mut |bytes| reserve_checkpoint_ingress(context.reader, bytes),
            context.deadline,
        )?;
        let bytes = downloaded_checkpoint_witness_wire(&downloaded);
        let reference = CheckpointProofReferenceInput {
            height,
            kind: if index + 1 == count {
                CheckpointProofKind::ClosingLookahead
            } else {
                CheckpointProofKind::Execution
            },
            length: bytes.len(),
            sha256: Sha256::digest(bytes).into(),
        };
        update_checkpoint_proof_stream_hash(&mut stream, &reference, bytes)
            .map_err(|error| anyhow::anyhow!("checkpoint proof checksum stream: {error:?}"))?;
        references.push(reference);
        check_checkpoint_bootstrap_deadline(context.deadline)?;
    }
    let stream_hash = finish_checkpoint_proof_stream_hash(stream)
        .map_err(|error| anyhow::anyhow!("checkpoint proof stream finish: {error:?}"))?;
    let bytes = create_checkpoint_proof_manifest(
        &descriptor.id,
        &descriptor.stats.body_sha256,
        target,
        &references,
        &stream_hash,
        &context.limits.proofs,
        metadata,
    )
    .map_err(|error| anyhow::anyhow!("checkpoint proof manifest: {error:?}"))?;
    check_checkpoint_bootstrap_deadline(context.deadline)?;
    Ok(ChargedCheckpointProofManifest {
        bytes,
        _lease: lease,
    })
}
