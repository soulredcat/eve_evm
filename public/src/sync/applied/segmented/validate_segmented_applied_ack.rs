// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    AppliedError,
    types::{PendingPayload, PendingRecord},
};
use crate::persistence::segmented::{
    SegmentedLogicalAck, segmented_batch_marker_cursor, segmented_batch_references,
};
use eve_storage::records::OpaqueRecordCursor;

pub(in crate::sync::applied) fn validate_segmented_applied_ack(
    pending: &PendingRecord,
    ack: SegmentedLogicalAck,
    durable_cursor: OpaqueRecordCursor,
    durable_height: u64,
    database_sequence: u64,
) -> Result<(), AppliedError> {
    let PendingPayload::Segmented {
        payload,
        identity,
        target_binding,
        ..
    } = &pending.payload
    else {
        return Err(AppliedError::WrongMode);
    };
    let references = segmented_batch_references(payload);
    if ack.segment_count == 0
        || ack.segment_count != references.len()
        || ack.segment_count > ack.references.len()
    {
        return Err(AppliedError::UnexpectedAcknowledgement);
    }
    if pending.parent != durable_cursor
        || identity.parent.height != durable_height
        || durable_height.checked_add(1) != Some(pending.target.height)
        || ack.identity != *identity
        || ack.identity.target_height != pending.target.height
        || ack.target_state_binding != *target_binding
        || ack.marker_cursor != pending.target_cursor
        || ack.marker_cursor != segmented_batch_marker_cursor(payload)
        || ack.references[..ack.segment_count] != *references
        || ack.database_sequence <= database_sequence
    {
        return Err(AppliedError::UnexpectedAcknowledgement);
    }
    Ok(())
}
