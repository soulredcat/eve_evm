// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    create_checkpoint_genesis_publication::create_checkpoint_genesis_publication,
    find_latest_checkpoint_base::find_latest_checkpoint_base,
    recover_checkpoint_suffix::recover_checkpoint_suffix,
    reopen_checkpoint_artifacts::reopen_checkpoint_artifacts, types::RecoveredCheckpointPrefix,
    validate_reopened_checkpoint_base::validate_reopened_checkpoint_base,
};
use crate::sync::applied::{
    AppliedConfig,
    checkpoints::{CheckpointAppliedError, CheckpointRecoveryConfig, prepare_applied_checkpoint},
    resources::storage_admission::AppliedStorageResourcePools,
    segmented::recover_segmented_applied_prefix,
    types::ChargedAppliedState,
};
use eve_state::DevelopmentGenesis;
use eve_storage::records::{OpaqueRecordRepository, segmented::SegmentedCodecLimits};
use std::sync::Arc;

pub(in crate::sync::applied) fn recover_checkpoint_applied_prefix(
    config: &AppliedConfig,
    recovery: Option<&CheckpointRecoveryConfig>,
    repository: &OpaqueRecordRepository,
    codec: &SegmentedCodecLimits,
    genesis_generation: Arc<ChargedAppliedState>,
    resource_pools: &AppliedStorageResourcePools,
    local_genesis: &DevelopmentGenesis,
) -> Result<RecoveredCheckpointPrefix, CheckpointAppliedError> {
    let working = &resource_pools.working;
    let base = match recovery {
        Some(recovery) => {
            let _read = crate::sync::applied::resources::storage_admission::reserve_storage_read(
                &resource_pools.storage,
            )
            .map_err(
                crate::sync::applied::resources::storage_admission::map_storage_admission_error,
            )
            .map_err(CheckpointAppliedError::Applied)?;
            find_latest_checkpoint_base(repository, recovery.maximum_scan_records, working)?
        }
        None => None,
    };
    let Some(base) = base else {
        let _read = crate::sync::applied::resources::storage_admission::reserve_storage_read(
            &resource_pools.storage,
        )
        .map_err(crate::sync::applied::resources::storage_admission::map_storage_admission_error)
        .map_err(CheckpointAppliedError::Applied)?;
        let (generation, position) = recover_segmented_applied_prefix(
            config,
            repository,
            codec,
            genesis_generation,
            working,
        )
        .map_err(CheckpointAppliedError::Applied)?;
        return Ok(RecoveredCheckpointPrefix {
            generation,
            position,
            checkpoint_height: 0,
            base: None,
        });
    };
    let recovery = recovery.ok_or(CheckpointAppliedError::InvalidArtifactBinding)?;
    if recovery.limits.content.logical != config.state_budget {
        return Err(CheckpointAppliedError::InvalidArtifactBinding);
    }
    let parent = create_checkpoint_genesis_publication(repository, genesis_generation, working)?;
    let artifacts =
        reopen_checkpoint_artifacts(&base, recovery, parent, working, &resource_pools.storage)?;
    let prepared = prepare_applied_checkpoint(artifacts, local_genesis)?;
    validate_reopened_checkpoint_base(&base, &prepared)?;
    let checkpoint_height = base.version.height;
    let (generation, position) = recover_checkpoint_suffix(
        config,
        repository,
        codec,
        &base,
        Arc::clone(&prepared.generation),
        working,
    )?;
    drop(prepared);
    Ok(RecoveredCheckpointPrefix {
        generation,
        position,
        checkpoint_height,
        base: Some(base),
    })
}
