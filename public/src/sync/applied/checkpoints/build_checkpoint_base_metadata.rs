// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointAppliedError, PreparedAppliedCheckpoint};
use crate::sync::applied::{AppliedOwner, state::applied_state_commit};
use eve_storage::records::segmented::checkpoints::{CheckpointBaseMetadata, CheckpointBaseMode};

pub(super) fn build_checkpoint_base_metadata(
    owner: &AppliedOwner,
    prepared: &PreparedAppliedCheckpoint,
) -> Result<CheckpointBaseMetadata, CheckpointAppliedError> {
    let target = &applied_state_commit(&prepared.generation.state).target;
    let parent = prepared
        .artifacts
        .content
        .parent
        .segmented_position
        .ok_or(CheckpointAppliedError::WrongMode)?
        .durable;
    Ok(CheckpointBaseMetadata {
        mode: CheckpointBaseMode::AuthenticatedImport,
        previous_opaque_cursor: owner.admitted_cursor,
        previous_logical_anchor: parent,
        target_height: target.height,
        target_state_binding: prepared.target_binding,
        snapshot_manifest_id: prepared.artifacts.content.content_id,
        snapshot_body_hash: prepared.artifacts.content.content.body_sha256,
        proof_manifest_id: prepared.artifacts.proof_id,
        proof_root: prepared.artifacts.proofs.stream_sha256,
        proof_genesis_height: 0,
        execution_start: 1,
        execution_end: target.height,
        lookahead_height: target
            .height
            .checked_add(1)
            .ok_or(CheckpointAppliedError::InvalidArtifactBinding)?,
        retained_start: 0,
        retained_end: target.height,
    })
}
