// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    AppliedConfig, AppliedError,
    recovery::prepare_charged_import,
    resources::{EstimatedWorkingPool, reserve_estimated_working},
    state::{AppliedState, applied_state_commit},
    types::ChargedAppliedState,
};
use super::{SegmentedAppliedPosition, bind_local_state_version};
use eve_state::BOUNDED_STATE_CODEC_SCRATCH_BYTES;
use eve_storage::records::{
    OpaqueRecordRepository, opaque_record_bootstrap_cursor,
    segmented::{
        SegmentedCodecLimits, SegmentedRecoveryAnchor,
        recovery::{
            SegmentedRecoveryStep, accept_segmented_recovery, begin_segmented_recovery,
            required_segmented_recovery_reservation, scan_next_segmented_recovery,
            segmented_recovery_bundle_anchor, segmented_recovery_bundle_bytes,
            segmented_recovery_observation,
        },
    },
};
use std::sync::Arc;

pub(super) fn recover_segmented_applied_prefix(
    config: &AppliedConfig,
    repository: &OpaqueRecordRepository,
    codec: &SegmentedCodecLimits,
    mut generation: Arc<ChargedAppliedState>,
    working: &Arc<EstimatedWorkingPool>,
) -> Result<(Arc<ChargedAppliedState>, SegmentedAppliedPosition), AppliedError> {
    let sizing = reserve_estimated_working(working, BOUNDED_STATE_CODEC_SCRATCH_BYTES)?;
    let binding = bind_local_state_version(&applied_state_commit(&generation.state).target)?;
    let anchor = SegmentedRecoveryAnchor {
        height: 0,
        cursor: opaque_record_bootstrap_cursor(repository),
        state_binding: binding,
    };
    let required = required_segmented_recovery_reservation(&config.repository_budget, codec)
        .map_err(AppliedError::SegmentedRecovery)?;
    let scan_lease = reserve_estimated_working(working, required)?;
    let mut scan = begin_segmented_recovery(repository, anchor, codec, required)
        .map_err(AppliedError::SegmentedRecovery)?;
    let mut missing_from = None;
    loop {
        match scan_next_segmented_recovery(&mut scan, repository, required)
            .map_err(AppliedError::SegmentedRecovery)?
        {
            SegmentedRecoveryStep::Progress { .. } => {}
            SegmentedRecoveryStep::Complete(bundle) => {
                let AppliedState::AuthenticatedImport(parent) = &generation.state else {
                    return Err(AppliedError::WrongMode);
                };
                let prepared = prepare_charged_import(
                    parent,
                    segmented_recovery_bundle_bytes(&bundle),
                    &config.state_budget,
                    working,
                    true,
                )?;
                let target = segmented_recovery_bundle_anchor(&bundle);
                if target.height != applied_state_commit(&prepared.state).target.height
                    || target.state_binding
                        != bind_local_state_version(&applied_state_commit(&prepared.state).target)?
                {
                    return Err(AppliedError::InvalidDurablePrefix);
                }
                accept_segmented_recovery(&mut scan, bundle, target, required)
                    .map_err(|rejected| AppliedError::SegmentedRecovery(rejected.error))?;
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
    let position = SegmentedAppliedPosition {
        applied: observation.logical_anchor,
        durable: observation.logical_anchor,
        last_acknowledged_physical: observation.physical_cursor,
        missing_from,
    };
    drop(scan);
    drop(scan_lease);
    drop(sizing);
    Ok((generation, position))
}
