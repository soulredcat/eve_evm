// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::DiscoveredCheckpointBase;
use crate::sync::applied::{
    checkpoints::CheckpointAppliedError,
    resources::{EstimatedWorkingPool, reserve_estimated_working},
};
use eve_storage::records::{
    OpaqueRecord, OpaqueRecordCursor, OpaqueRecordRepository, opaque_record_budget,
    segmented::checkpoints::{
        CHECKPOINT_BASE_MAX_PAYLOAD_BYTES, CheckpointBaseLimits, checkpoint_base_inspection_view,
        inspect_checkpoint_base, prepare_checkpoint_base_target, read_checkpoint_base_membership,
        required_checkpoint_base_encoding_reservation,
        required_checkpoint_base_membership_reservation,
    },
};
use std::sync::Arc;

pub(super) fn read_discovered_checkpoint_base(
    repository: &OpaqueRecordRepository,
    row: &OpaqueRecord,
    working: &Arc<EstimatedWorkingPool>,
) -> Result<DiscoveredCheckpointBase, CheckpointAppliedError> {
    let limits = CheckpointBaseLimits {
        maximum_payload_bytes: CHECKPOINT_BASE_MAX_PAYLOAD_BYTES,
    };
    let inspect =
        inspect_checkpoint_base(&row.payload, &limits).map_err(CheckpointAppliedError::Base)?;
    let view = checkpoint_base_inspection_view(&inspect);
    let membership_required =
        required_checkpoint_base_membership_reservation(&opaque_record_budget(repository), &limits)
            .map_err(CheckpointAppliedError::Base)?;
    let encoding_required = required_checkpoint_base_encoding_reservation(&limits)
        .map_err(CheckpointAppliedError::Base)?;
    let required = membership_required
        .checked_add(encoding_required)
        .and_then(|bytes| bytes.checked_add(65_536))
        .ok_or(CheckpointAppliedError::InvalidArtifactBinding)?;
    let lease =
        reserve_estimated_working(working, required).map_err(CheckpointAppliedError::Applied)?;
    let version = eve_state::decode_state_version(view.target_version_bytes)
        .map_err(CheckpointAppliedError::State)?;
    let target = prepare_checkpoint_base_target(&version, &limits, encoding_required)
        .map_err(CheckpointAppliedError::Base)?;
    let cursor = OpaqueRecordCursor {
        sequence: row.sequence,
        content_hash: row.content_hash,
    };
    let membership = read_checkpoint_base_membership(
        repository,
        cursor,
        view.metadata,
        &target,
        &limits,
        membership_required,
    )
    .map_err(CheckpointAppliedError::Base)?;
    Ok(DiscoveredCheckpointBase {
        membership,
        target,
        version,
        _lease: lease,
    })
}
