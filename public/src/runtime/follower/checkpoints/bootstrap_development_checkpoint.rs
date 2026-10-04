// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    activate_downloaded_checkpoint::activate_downloaded_checkpoint,
    check_checkpoint_bootstrap_deadline::check_checkpoint_bootstrap_deadline,
    collect_checkpoint_proof_manifest::collect_checkpoint_proof_manifest,
    download_checkpoint_content::download_checkpoint_content,
    download_checkpoint_proofs::download_checkpoint_proofs,
    open_development_checkpoint_directories::open_development_checkpoint_directories,
    prepare_local_checkpoint_genesis::prepare_local_checkpoint_genesis,
    types::{CHECKPOINT_BOOTSTRAP_SECONDS, CheckpointDownloadContext},
};
use crate::sync::applied::{
    AppliedOwner, applied_owner_reader, applied_owner_state_budget,
    checkpoints::CheckpointRecoveryConfig,
};
use anyhow::{Context, Result};
use eve_state::DevelopmentGenesis;
use eve_storage::checkpoints::messages::CheckpointMessageLimits;
use std::{
    net::SocketAddr,
    path::Path,
    time::{Duration, Instant},
};

pub(in crate::runtime::follower) fn bootstrap_development_checkpoint(
    owner: &mut AppliedOwner,
    data: &Path,
    recovery: &CheckpointRecoveryConfig,
    genesis: &DevelopmentGenesis,
    height: u64,
    address: SocketAddr,
) -> Result<()> {
    let deadline = Instant::now()
        .checked_add(Duration::from_secs(CHECKPOINT_BOOTSTRAP_SECONDS))
        .context("checkpoint bootstrap deadline overflow")?;
    let directories = open_development_checkpoint_directories(data, true, Some(deadline))?
        .context("checkpoint directories missing")?;
    check_checkpoint_bootstrap_deadline(deadline)?;
    let reader = applied_owner_reader(owner);
    let budget = applied_owner_state_budget(owner);
    let local_version = prepare_local_checkpoint_genesis(&reader, genesis, &budget)?;
    check_checkpoint_bootstrap_deadline(deadline)?;
    let context = CheckpointDownloadContext {
        reader: &reader,
        genesis: &local_version.version,
        address,
        limits: recovery.limits,
        deadline,
        message: CheckpointMessageLimits {
            logical: budget,
            maximum_body_bytes: recovery.limits.content.maximum_body_bytes,
            maximum_manifest_bytes: recovery.limits.content.maximum_manifest_bytes,
            maximum_chunk_bytes: recovery.limits.content.maximum_chunk_bytes,
        },
    };
    let (content, manifest) =
        download_checkpoint_content(owner, &directories.content, height, &context)?;
    let proofs = collect_checkpoint_proof_manifest(&manifest, &context)?;
    // First-pass source response/HTTP charges are no longer needed during second-pass transfer.
    drop(manifest);
    let artifacts =
        download_checkpoint_proofs(content, &directories.proofs, proofs, height, &context)?;
    activate_downloaded_checkpoint(owner, artifacts, genesis, deadline)
}
