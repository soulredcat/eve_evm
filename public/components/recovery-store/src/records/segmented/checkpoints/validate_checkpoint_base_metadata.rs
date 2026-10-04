// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointBaseError, CheckpointBaseMetadata, PreparedCheckpointBaseTarget};

pub(super) fn validate_checkpoint_base_metadata(
    metadata: &CheckpointBaseMetadata,
    target: &PreparedCheckpointBaseTarget,
) -> Result<(), CheckpointBaseError> {
    let parent = metadata.previous_logical_anchor;
    let physical = metadata.previous_opaque_cursor;
    let hashes = [
        physical.content_hash,
        parent.cursor.content_hash,
        parent.state_binding,
        metadata.target_state_binding,
        metadata.snapshot_manifest_id,
        metadata.snapshot_body_hash,
        metadata.proof_manifest_id,
        metadata.proof_root,
    ];
    if hashes.contains(&[0; 32])
        || parent.cursor.sequence > physical.sequence
        || (parent.cursor.sequence == physical.sequence && parent.cursor != physical)
        || (parent.height == 0 && parent.cursor.sequence != 0)
        || (parent.height > 0 && parent.cursor.sequence == 0)
        || metadata.target_height <= parent.height
        || metadata.target_height != target.height
        || metadata.proof_genesis_height != 0
        || metadata.execution_start != 1
        || metadata.execution_end != metadata.target_height
        || metadata.retained_start > metadata.retained_end
        || metadata.retained_end > metadata.target_height
    {
        return Err(CheckpointBaseError::InvalidMetadata);
    }
    let lookahead = metadata
        .target_height
        .checked_add(1)
        .ok_or(CheckpointBaseError::ArithmeticOverflow)?;
    physical
        .sequence
        .checked_add(1)
        .ok_or(CheckpointBaseError::ArithmeticOverflow)?;
    if metadata.lookahead_height != lookahead {
        return Err(CheckpointBaseError::InvalidMetadata);
    }
    Ok(())
}
