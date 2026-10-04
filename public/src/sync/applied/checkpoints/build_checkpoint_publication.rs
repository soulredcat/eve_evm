// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointAppliedError, PreparedAppliedCheckpoint};
use crate::sync::applied::{
    AppliedPublication, publication::build_applied_markers, segmented::SegmentedAppliedPosition,
    state::applied_state_commit,
};
use eve_node_policy::{CheckpointHeight, validate_watermarks};
use eve_storage::records::{OpaqueRecordCursor, segmented::SegmentedRecoveryAnchor};
use std::sync::Arc;

/// Allocate coherent publication bookkeeping before the base is submitted to storage.
pub(super) fn build_checkpoint_publication(
    prepared: &PreparedAppliedCheckpoint,
    cursor: OpaqueRecordCursor,
) -> Result<Arc<AppliedPublication>, CheckpointAppliedError> {
    let height = applied_state_commit(&prepared.generation.state)
        .target
        .height;
    let mut markers = build_applied_markers(&prepared.generation.state, height)
        .map_err(CheckpointAppliedError::Applied)?;
    markers.checkpoint = CheckpointHeight(height);
    markers.authenticated_snapshot_height = height;
    validate_watermarks(markers).map_err(|_| CheckpointAppliedError::InvalidArtifactBinding)?;
    let anchor = SegmentedRecoveryAnchor {
        height,
        cursor,
        state_binding: prepared.target_binding,
    };
    Ok(Arc::new(AppliedPublication {
        generation: Arc::clone(&prepared.generation),
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
