// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointBaseError, CheckpointBaseMetadata, CheckpointBaseMode,
    read_checkpoint_base_array::read_checkpoint_base_array,
};
use crate::records::{OpaqueRecordCursor, segmented::SegmentedRecoveryAnchor};

pub(super) fn read_checkpoint_base_metadata(
    bytes: &[u8],
) -> Result<CheckpointBaseMetadata, CheckpointBaseError> {
    Ok(CheckpointBaseMetadata {
        mode: CheckpointBaseMode::AuthenticatedImport,
        previous_opaque_cursor: OpaqueRecordCursor {
            sequence: u64::from_be_bytes(read_checkpoint_base_array(bytes, 28)?),
            content_hash: read_checkpoint_base_array(bytes, 36)?,
        },
        previous_logical_anchor: SegmentedRecoveryAnchor {
            height: u64::from_be_bytes(read_checkpoint_base_array(bytes, 68)?),
            cursor: OpaqueRecordCursor {
                sequence: u64::from_be_bytes(read_checkpoint_base_array(bytes, 76)?),
                content_hash: read_checkpoint_base_array(bytes, 84)?,
            },
            state_binding: read_checkpoint_base_array(bytes, 116)?,
        },
        target_height: u64::from_be_bytes(read_checkpoint_base_array(bytes, 148)?),
        target_state_binding: read_checkpoint_base_array(bytes, 156)?,
        snapshot_manifest_id: read_checkpoint_base_array(bytes, 188)?,
        snapshot_body_hash: read_checkpoint_base_array(bytes, 220)?,
        proof_manifest_id: read_checkpoint_base_array(bytes, 252)?,
        proof_root: read_checkpoint_base_array(bytes, 284)?,
        proof_genesis_height: u64::from_be_bytes(read_checkpoint_base_array(bytes, 316)?),
        execution_start: u64::from_be_bytes(read_checkpoint_base_array(bytes, 324)?),
        execution_end: u64::from_be_bytes(read_checkpoint_base_array(bytes, 332)?),
        lookahead_height: u64::from_be_bytes(read_checkpoint_base_array(bytes, 340)?),
        retained_start: u64::from_be_bytes(read_checkpoint_base_array(bytes, 348)?),
        retained_end: u64::from_be_bytes(read_checkpoint_base_array(bytes, 356)?),
    })
}
