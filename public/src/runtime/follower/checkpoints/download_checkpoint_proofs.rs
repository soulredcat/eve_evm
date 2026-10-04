// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    check_checkpoint_bootstrap_deadline::check_checkpoint_bootstrap_deadline,
    stage_checkpoint_proof_witness::stage_checkpoint_proof_witness,
    types::{ChargedCheckpointProofManifest, CheckpointDownloadContext},
};
use crate::sync::applied::checkpoints::{
    ChargedCheckpointArtifacts, ChargedCompletedCheckpoint, begin_applied_checkpoint_proofs,
    complete_applied_checkpoint_proofs, repair_applied_checkpoint_proof_pending,
};
use anyhow::{Context, Result};
use std::fs::File;

/// Second pass writes each missing witness against immutable first-pass length and SHA references.
pub(super) fn download_checkpoint_proofs(
    content: ChargedCompletedCheckpoint,
    root: &File,
    manifest: ChargedCheckpointProofManifest,
    height: u64,
    context: &CheckpointDownloadContext<'_>,
) -> Result<ChargedCheckpointArtifacts> {
    repair_applied_checkpoint_proof_pending(&content, root, &manifest.bytes)
        .map_err(|error| anyhow::anyhow!("checkpoint pending proof metadata refused: {error:?}"))?;
    check_checkpoint_bootstrap_deadline(context.deadline)?;
    let mut transfer = begin_applied_checkpoint_proofs(content, root, &manifest.bytes)
        .map_err(|error| anyhow::anyhow!("checkpoint proof staging: {error:?}"))?;
    check_checkpoint_bootstrap_deadline(context.deadline)?;
    let count = usize::try_from(
        height
            .checked_add(1)
            .context("checkpoint proof height overflow")?,
    )?;
    for index in 0..count {
        stage_checkpoint_proof_witness(&mut transfer, height, index, context)?;
    }
    let complete = complete_applied_checkpoint_proofs(transfer)
        .map_err(|error| anyhow::anyhow!("checkpoint proof completion: {error:?}"))?;
    check_checkpoint_bootstrap_deadline(context.deadline)?;
    Ok(complete)
}
