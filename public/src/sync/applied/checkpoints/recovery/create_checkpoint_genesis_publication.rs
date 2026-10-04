// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{
    AppliedPublication,
    checkpoints::CheckpointAppliedError,
    publication::build_applied_markers,
    resources::{EstimatedWorkingPool, reserve_estimated_working},
    segmented::{SegmentedAppliedPosition, bind_local_state_version},
    state::applied_state_commit,
    types::ChargedAppliedState,
};
use eve_state::BOUNDED_STATE_CODEC_SCRATCH_BYTES;
use eve_storage::records::{
    OpaqueRecordRepository, opaque_record_bootstrap_cursor, segmented::SegmentedRecoveryAnchor,
};
use std::sync::Arc;

/// Private startup parent only. It is never published to RPC before complete recovery succeeds.
pub(super) fn create_checkpoint_genesis_publication(
    repository: &OpaqueRecordRepository,
    generation: Arc<ChargedAppliedState>,
    working: &Arc<EstimatedWorkingPool>,
) -> Result<Arc<AppliedPublication>, CheckpointAppliedError> {
    if applied_state_commit(&generation.state).target.height != 0 {
        return Err(CheckpointAppliedError::InvalidArtifactBinding);
    }
    let _scratch = reserve_estimated_working(working, BOUNDED_STATE_CODEC_SCRATCH_BYTES)
        .map_err(CheckpointAppliedError::Applied)?;
    let cursor = opaque_record_bootstrap_cursor(repository);
    let anchor = SegmentedRecoveryAnchor {
        height: 0,
        cursor,
        state_binding: bind_local_state_version(&applied_state_commit(&generation.state).target)
            .map_err(CheckpointAppliedError::Applied)?,
    };
    let markers =
        build_applied_markers(&generation.state, 0).map_err(CheckpointAppliedError::Applied)?;
    Ok(Arc::new(AppliedPublication {
        generation,
        markers,
        durable_cursor: cursor,
        admitted_cursor: cursor,
        storage_failed: false,
        segmented_position: Some(SegmentedAppliedPosition {
            applied: anchor,
            durable: anchor,
            last_acknowledged_physical: cursor,
            missing_from: None,
        }),
    }))
}
