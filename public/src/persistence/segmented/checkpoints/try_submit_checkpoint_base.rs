// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedError, SegmentedWorker, pool::observe_segmented_parts, types::WorkerRequest,
};
use super::{
    CheckpointTicket, RejectedCheckpointRecord, SealedCheckpointRecord,
    types::{AdmittedCheckpoint, CheckpointRequest},
};
use eve_storage::records::{OpaqueRecordCursor, segmented::SegmentedRecoveryAnchor};
use std::{
    sync::{
        Arc,
        atomic::Ordering,
        mpsc::{self, TrySendError},
    },
    time::Duration,
};

/// Single checkpoint request shares the existing sole WAL owner and blocks batches.
pub fn try_submit_checkpoint_base(
    worker: &SegmentedWorker,
    expected: OpaqueRecordCursor,
    record: SealedCheckpointRecord,
) -> Result<CheckpointTicket, RejectedCheckpointRecord> {
    let reject = |error, record| RejectedCheckpointRecord { error, record };
    if !Arc::ptr_eq(&worker.state.pool, &record.0._metadata.pool) {
        return Err(reject(SegmentedError::ForeignPool, record));
    }
    if worker.state.failed.load(Ordering::Acquire) {
        return Err(reject(SegmentedError::StorageFailed, record));
    }
    let mut admission = match worker.state.admission.lock() {
        Ok(value) => value,
        Err(_) => return Err(reject(SegmentedError::AccountingUnavailable, record)),
    };
    let age = match observe_segmented_parts(&worker.state.pool) {
        Ok(value) => value.oldest_age.max(record.0.created.elapsed()),
        Err(error) => return Err(reject(error, record)),
    };
    if age > Duration::from_millis(worker.state.pool.policy.maximum_queue_age_ms) {
        return Err(reject(SegmentedError::QueueAged, record));
    }
    if admission
        .checkpoint
        .as_ref()
        .is_some_and(|slot| slot.record.0.id == record.0.id)
    {
        return Err(reject(SegmentedError::AlreadySubmitted, record));
    }
    if admission.checkpoint.is_some() || admission.slots.iter().any(Option::is_some) {
        return Err(reject(SegmentedError::QueueFull, record));
    }
    let metadata = record.0.metadata;
    if expected != admission.cursor
        || expected != metadata.previous_opaque_cursor
        || metadata.previous_logical_anchor != admission.logical
        || metadata.target_height <= admission.logical.height
    {
        return Err(reject(SegmentedError::WrongCursor, record));
    }
    let (reply, receiver) = mpsc::sync_channel(1);
    admission.checkpoint = Some(AdmittedCheckpoint {
        record: record.clone(),
        acknowledged: expected,
        complete: None,
    });
    match worker
        .sender
        .try_send(WorkerRequest::Checkpoint(CheckpointRequest {
            record,
            reply,
        })) {
        Ok(()) => {
            let retained = admission
                .checkpoint
                .as_ref()
                .expect("reserved checkpoint admission");
            let id = retained.record.0.id;
            let next = retained.record.0.cursor;
            admission.cursor = next;
            admission.logical = SegmentedRecoveryAnchor {
                height: metadata.target_height,
                cursor: next,
                state_binding: metadata.target_state_binding,
            };
            Ok(CheckpointTicket {
                receiver,
                state: Arc::clone(&worker.state),
                id,
            })
        }
        Err(error) => {
            let retained = admission.checkpoint.take();
            drop(admission);
            drop(retained);
            let (error, request) = match error {
                TrySendError::Full(request) => (SegmentedError::QueueFull, request),
                TrySendError::Disconnected(request) => (SegmentedError::Closed, request),
            };
            let WorkerRequest::Checkpoint(request) = request else {
                unreachable!("checkpoint submission returned its same request")
            };
            Err(reject(error, request.record))
        }
    }
}
