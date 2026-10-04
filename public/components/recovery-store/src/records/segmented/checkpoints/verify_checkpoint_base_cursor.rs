// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointBaseError;
use crate::records::{
    OpaqueRecordCursor, OpaqueRecordRepository, opaque_record_bootstrap_cursor, read_opaque_record,
};

pub(super) fn verify_checkpoint_base_cursor(
    repository: &OpaqueRecordRepository,
    cursor: OpaqueRecordCursor,
) -> Result<(), CheckpointBaseError> {
    if cursor.sequence == 0 {
        return if cursor == opaque_record_bootstrap_cursor(repository) {
            Ok(())
        } else {
            Err(CheckpointBaseError::MembershipMismatch)
        };
    }
    let record = read_opaque_record(repository, cursor.sequence)
        .map_err(|_| CheckpointBaseError::StorageFailed)?
        .ok_or(CheckpointBaseError::MissingRecord)?;
    if record.content_hash != cursor.content_hash {
        return Err(CheckpointBaseError::MembershipMismatch);
    }
    Ok(())
}
