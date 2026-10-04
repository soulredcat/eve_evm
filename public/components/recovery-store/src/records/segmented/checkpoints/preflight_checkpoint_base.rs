// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointBaseError, CheckpointBaseLimits, CheckpointBasePreflight,
    PreparedCheckpointBaseTarget, checkpoint_base_inspection_view, inspect_checkpoint_base,
    validate_checkpoint_base_metadata::validate_checkpoint_base_metadata,
};
use crate::records::{OpaqueRecordCursor, segmented::SegmentedRecoveryAnchor};

/// Allocation-free local framing/hash check against a separately prepared canonical
/// target. It does not parse proof bytes or establish state correctness/finality.
pub fn preflight_checkpoint_base<'a>(
    bytes: &'a [u8],
    target: &PreparedCheckpointBaseTarget,
    expected_physical_parent: OpaqueRecordCursor,
    expected_logical_parent: SegmentedRecoveryAnchor,
    limits: &CheckpointBaseLimits,
) -> Result<CheckpointBasePreflight<'a>, CheckpointBaseError> {
    let inspected = inspect_checkpoint_base(bytes, limits).map_err(|error| match error {
        CheckpointBaseError::InvalidTarget => CheckpointBaseError::TargetMismatch,
        other => other,
    })?;
    let view = checkpoint_base_inspection_view(&inspected);
    if view.metadata.previous_opaque_cursor != expected_physical_parent
        || view.metadata.previous_logical_anchor != expected_logical_parent
    {
        return Err(CheckpointBaseError::ParentMismatch);
    }
    if view.security_profile != target.security_profile
        || view.target_version_bytes != target.encoded.as_ref()
    {
        return Err(CheckpointBaseError::TargetMismatch);
    }
    validate_checkpoint_base_metadata(&view.metadata, target)?;
    Ok(CheckpointBasePreflight { bytes, view })
}
