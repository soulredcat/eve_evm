// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedError, types::WorkerState, worker::append_segmented_record::append_segmented_record,
};
use super::{CheckpointAck, types::CheckpointRequest};
use eve_storage::records::{OpaqueRecordDisposition, OpaqueRecordRepository};
use std::sync::Arc;

/// Only a real newly-synced opaque ACK can advance the retained checkpoint tail.
pub(in crate::persistence::segmented) fn persist_checkpoint_base(
    repository: &mut OpaqueRecordRepository,
    state: &Arc<WorkerState>,
    request: &CheckpointRequest,
) -> Result<CheckpointAck, SegmentedError> {
    let record = &request.record.0;
    let ack = append_segmented_record(
        repository,
        state,
        record.metadata.previous_opaque_cursor,
        &record.payload,
        0,
    )?;
    if ack.disposition != OpaqueRecordDisposition::NewlySynced
        || ack.appended != record.cursor
        || ack.store_head != record.cursor
        || ack.database_sequence == 0
    {
        return Err(SegmentedError::AckMismatch);
    }
    let mut admission = state
        .admission
        .lock()
        .map_err(|_| SegmentedError::AccountingUnavailable)?;
    let retained = admission
        .checkpoint
        .as_mut()
        .ok_or(SegmentedError::AckMismatch)?;
    if retained.record.0.id != record.id {
        return Err(SegmentedError::AckMismatch);
    }
    retained.acknowledged = ack.appended;
    Ok(CheckpointAck {
        metadata: record.metadata,
        cursor: ack.appended,
        database_sequence: ack.database_sequence,
    })
}
