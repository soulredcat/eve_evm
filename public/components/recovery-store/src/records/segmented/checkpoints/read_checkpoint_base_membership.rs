// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointBaseError, CheckpointBaseLimits, CheckpointBaseMembership, CheckpointBaseMetadata,
    PreparedCheckpointBaseTarget, checkpoint_base_view, preflight_checkpoint_base,
    required_checkpoint_base_membership_reservation,
    verify_checkpoint_base_cursor::verify_checkpoint_base_cursor,
};
use crate::records::{
    OpaqueRecordCursor, OpaqueRecordRepository, opaque_record_budget, read_opaque_record,
};

/// Read the actual durable row; a supplied height/hash or prospective cursor is
/// insufficient. Storage integrity cannot authenticate referenced artifacts.
pub fn read_checkpoint_base_membership(
    repository: &OpaqueRecordRepository,
    cursor: OpaqueRecordCursor,
    expected: CheckpointBaseMetadata,
    target: &PreparedCheckpointBaseTarget,
    limits: &CheckpointBaseLimits,
    reserved_bytes: usize,
) -> Result<CheckpointBaseMembership, CheckpointBaseError> {
    let required =
        required_checkpoint_base_membership_reservation(&opaque_record_budget(repository), limits)?;
    if reserved_bytes < required {
        return Err(CheckpointBaseError::InsufficientReservation);
    }
    if cursor.sequence == 0
        || expected.previous_opaque_cursor.sequence.checked_add(1) != Some(cursor.sequence)
    {
        return Err(CheckpointBaseError::MembershipMismatch);
    }
    verify_checkpoint_base_cursor(repository, expected.previous_opaque_cursor)?;
    if expected.previous_logical_anchor.cursor != expected.previous_opaque_cursor {
        verify_checkpoint_base_cursor(repository, expected.previous_logical_anchor.cursor)?;
    }
    let record = read_opaque_record(repository, cursor.sequence)
        .map_err(|_| CheckpointBaseError::StorageFailed)?
        .ok_or(CheckpointBaseError::MissingRecord)?;
    if record.content_hash != cursor.content_hash
        || record.parent != expected.previous_opaque_cursor
    {
        return Err(CheckpointBaseError::MembershipMismatch);
    }
    let checked = preflight_checkpoint_base(
        &record.payload,
        target,
        expected.previous_opaque_cursor,
        expected.previous_logical_anchor,
        limits,
    )?;
    let view = checkpoint_base_view(&checked);
    if view.metadata != expected {
        return Err(CheckpointBaseError::MembershipMismatch);
    }
    let security_profile = view.security_profile;
    Ok(CheckpointBaseMembership {
        record,
        metadata: expected,
        security_profile,
    })
}
