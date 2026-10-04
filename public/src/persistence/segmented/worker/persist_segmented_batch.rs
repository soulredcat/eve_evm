// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        SegmentedError, SegmentedLogicalAck,
        types::{Request, WorkerState},
    },
    append_segmented_record::append_segmented_record,
};
use eve_storage::records::{OpaqueRecordDisposition, OpaqueRecordRepository};
use std::sync::Arc;

/// Sequential real one-record syncs, then marker only after every exact data acknowledgement.
pub(super) fn persist_segmented_batch(
    repository: &mut OpaqueRecordRepository,
    state: &Arc<WorkerState>,
    request: &Request,
) -> Result<SegmentedLogicalAck, SegmentedError> {
    let batch = &request.batch.0;
    let mut cursor = batch.expected;
    let mut database_sequence = 0;
    for index in 0..=batch.plan.layout.segment_count {
        let (part, offset, planned) = if index < batch.plan.layout.segment_count {
            (
                index / batch.plan.layout.per_part,
                index % batch.plan.layout.per_part,
                batch.references[index],
            )
        } else {
            (batch.plan.layout.part_count - 1, 0, batch.marker_cursor)
        };
        let payload = &batch.parts[part]
            .as_ref()
            .ok_or(SegmentedError::InvalidPlan)?
            .buffers[offset];
        let ack = append_segmented_record(repository, state, cursor, payload, index)?;
        if ack.disposition != OpaqueRecordDisposition::NewlySynced
            || ack.appended != planned
            || ack.store_head != planned
            || ack.database_sequence <= database_sequence
        {
            return Err(SegmentedError::AckMismatch);
        }
        cursor = planned;
        database_sequence = ack.database_sequence;
        let mut admission = state
            .admission
            .lock()
            .map_err(|_| SegmentedError::AccountingUnavailable)?;
        let retained = admission.slots[request.slot]
            .as_mut()
            .ok_or(SegmentedError::AckMismatch)?;
        if retained.batch.0.id != batch.id {
            return Err(SegmentedError::AckMismatch);
        }
        retained.acknowledged = cursor;
    }
    Ok(SegmentedLogicalAck {
        identity: batch.plan.marker.identity,
        target_state_binding: batch.plan.marker.target_state_binding,
        marker_cursor: cursor,
        references: batch.references,
        segment_count: batch.plan.layout.segment_count,
        database_sequence,
    })
}
