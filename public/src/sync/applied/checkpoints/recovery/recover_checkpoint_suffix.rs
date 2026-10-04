// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::DiscoveredCheckpointBase;
use crate::sync::applied::{
    AppliedConfig, AppliedError,
    checkpoints::CheckpointAppliedError,
    recovery::prepare_charged_import,
    resources::{EstimatedWorkingPool, reserve_estimated_working},
    segmented::{SegmentedAppliedPosition, bind_local_state_version},
    state::{AppliedState, applied_state_commit},
    types::ChargedAppliedState,
};
use eve_state::BOUNDED_STATE_CODEC_SCRATCH_BYTES;
use eve_storage::records::{
    OpaqueRecordRepository,
    segmented::{
        SegmentedCodecLimits,
        recovery::{
            SegmentedRecoveryStep, accept_segmented_recovery, begin_segmented_checkpoint_recovery,
            required_segmented_checkpoint_recovery_reservation, scan_next_segmented_recovery,
            segmented_recovery_bundle_anchor, segmented_recovery_bundle_bytes,
            segmented_recovery_observation,
        },
    },
};
use std::sync::Arc;

/// Every suffix still passes canonical V2 H/H+1 import; a typed local base never bypasses it.
pub(super) fn recover_checkpoint_suffix(
    config: &AppliedConfig,
    repository: &OpaqueRecordRepository,
    codec: &SegmentedCodecLimits,
    base: &DiscoveredCheckpointBase,
    mut generation: Arc<ChargedAppliedState>,
    working: &Arc<EstimatedWorkingPool>,
) -> Result<(Arc<ChargedAppliedState>, SegmentedAppliedPosition), CheckpointAppliedError> {
    let _sizing = reserve_estimated_working(working, BOUNDED_STATE_CODEC_SCRATCH_BYTES)
        .map_err(CheckpointAppliedError::Applied)?;
    let required =
        required_segmented_checkpoint_recovery_reservation(&config.repository_budget, codec)
            .map_err(CheckpointAppliedError::Recovery)?;
    let _scan_lease =
        reserve_estimated_working(working, required).map_err(CheckpointAppliedError::Applied)?;
    let mut scan = begin_segmented_checkpoint_recovery(
        repository,
        &base.membership,
        &base.target,
        codec,
        required,
    )
    .map_err(CheckpointAppliedError::Recovery)?;
    let mut missing_from = None;
    loop {
        match scan_next_segmented_recovery(&mut scan, repository, required)
            .map_err(CheckpointAppliedError::Recovery)?
        {
            SegmentedRecoveryStep::Progress { .. } => {}
            SegmentedRecoveryStep::Complete(bundle) => {
                let AppliedState::AuthenticatedImport(parent) = &generation.state else {
                    return Err(CheckpointAppliedError::WrongMode);
                };
                let prepared = prepare_charged_import(
                    parent,
                    segmented_recovery_bundle_bytes(&bundle),
                    &config.state_budget,
                    working,
                    true,
                )
                .map_err(CheckpointAppliedError::Applied)?;
                let target = segmented_recovery_bundle_anchor(&bundle);
                if target.height != applied_state_commit(&prepared.state).target.height
                    || target.state_binding
                        != bind_local_state_version(&applied_state_commit(&prepared.state).target)
                            .map_err(CheckpointAppliedError::Applied)?
                {
                    return Err(CheckpointAppliedError::Applied(
                        AppliedError::InvalidDurablePrefix,
                    ));
                }
                accept_segmented_recovery(&mut scan, bundle, target, required)
                    .map_err(|rejected| CheckpointAppliedError::Recovery(rejected.error))?;
                generation = prepared;
            }
            SegmentedRecoveryStep::Incomplete { .. } => {
                missing_from = segmented_recovery_observation(&scan)
                    .logical_anchor
                    .height
                    .checked_add(1);
                break;
            }
            SegmentedRecoveryStep::Exhausted => break,
        }
    }
    let observation = segmented_recovery_observation(&scan);
    Ok((
        generation,
        SegmentedAppliedPosition {
            applied: observation.logical_anchor,
            durable: observation.logical_anchor,
            last_acknowledged_physical: observation.physical_cursor,
            missing_from,
        },
    ))
}
