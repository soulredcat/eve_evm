// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::DiscoveredCheckpointBase;
use crate::sync::applied::{
    checkpoints::{CheckpointAppliedError, PreparedAppliedCheckpoint},
    state::applied_state_commit,
};
use eve_storage::records::segmented::checkpoints::checkpoint_base_membership_view;

pub(super) fn validate_reopened_checkpoint_base(
    base: &DiscoveredCheckpointBase,
    prepared: &PreparedAppliedCheckpoint,
) -> Result<(), CheckpointAppliedError> {
    let metadata = checkpoint_base_membership_view(&base.membership).metadata;
    let actual = &applied_state_commit(&prepared.generation.state).target;
    if actual != &base.version
        || actual.height != metadata.target_height
        || prepared.target_binding != metadata.target_state_binding
        || prepared.artifacts.content.content_id != metadata.snapshot_manifest_id
        || prepared.artifacts.content.content.body_sha256 != metadata.snapshot_body_hash
        || prepared.artifacts.proof_id != metadata.proof_manifest_id
        || prepared.artifacts.proofs.stream_sha256 != metadata.proof_root
        || metadata.proof_genesis_height != 0
        || metadata.execution_start != 1
        || metadata.execution_end != actual.height
        || metadata.lookahead_height
            != actual
                .height
                .checked_add(1)
                .ok_or(CheckpointAppliedError::InvalidArtifactBinding)?
        || metadata.retained_start != 0
        || metadata.retained_end != actual.height
    {
        return Err(CheckpointAppliedError::InvalidArtifactBinding);
    }
    Ok(())
}
