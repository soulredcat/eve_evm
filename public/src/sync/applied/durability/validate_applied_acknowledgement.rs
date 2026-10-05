// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedError, types::PendingRecord};
use eve_storage::records::{OpaqueRecordAck, OpaqueRecordCursor, OpaqueRecordDisposition};

/// A ticket advances only the exact next already verified recovery record.
pub(super) fn validate_applied_acknowledgement(
    pending: &PendingRecord,
    ack: OpaqueRecordAck,
    durable_cursor: OpaqueRecordCursor,
    durable_height: u64,
    database_sequence: u64,
) -> Result<(), AppliedError> {
    if pending.parent != durable_cursor
        || pending.target_cursor != ack.appended
        || ack.store_head != ack.appended
        || ack.disposition != OpaqueRecordDisposition::NewlySynced
        || durable_height.checked_add(1) != Some(pending.target.height)
        || durable_cursor.sequence.checked_add(1) != Some(ack.appended.sequence)
        || ack.database_sequence <= database_sequence
    {
        return Err(AppliedError::UnexpectedAcknowledgement);
    }
    Ok(())
}
