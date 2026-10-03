// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    RejectedSegmentedBatch, SealedSegmentedBatch, SegmentedError, SegmentedTicket, SegmentedWorker,
    pool::observe_segmented_parts,
    types::{AdmittedBatch, Request},
};
use eve_storage::records::OpaqueRecordCursor;
use eve_storage::records::segmented::SegmentedRecoveryAnchor;
use std::sync::{
    Arc,
    atomic::Ordering,
    mpsc::{self, TrySendError},
};
use std::time::Duration;

pub fn try_submit_segmented_batch(
    worker: &SegmentedWorker,
    expected: OpaqueRecordCursor,
    batch: SealedSegmentedBatch,
) -> Result<SegmentedTicket, RejectedSegmentedBatch> {
    let reject = |error, batch| RejectedSegmentedBatch { error, batch };
    if !Arc::ptr_eq(&worker.state.pool, &batch.0._metadata.pool) {
        return Err(reject(SegmentedError::ForeignPool, batch));
    }
    if worker.state.failed.load(Ordering::Acquire) {
        return Err(reject(SegmentedError::StorageFailed, batch));
    }
    let observation = match observe_segmented_parts(&worker.state.pool) {
        Ok(value) => value,
        Err(error) => return Err(reject(error, batch)),
    };
    if observation.oldest_age > Duration::from_millis(worker.state.pool.policy.maximum_queue_age_ms)
    {
        return Err(reject(SegmentedError::QueueAged, batch));
    }
    let mut admission = match worker.state.admission.lock() {
        Ok(value) => value,
        Err(_) => return Err(reject(SegmentedError::AccountingUnavailable, batch)),
    };
    let final_age = match observe_segmented_parts(&worker.state.pool) {
        Ok(value) => value.oldest_age,
        Err(error) => return Err(reject(error, batch)),
    };
    if final_age > Duration::from_millis(worker.state.pool.policy.maximum_queue_age_ms) {
        return Err(reject(SegmentedError::QueueAged, batch));
    }
    if admission
        .slots
        .iter()
        .flatten()
        .any(|slot| slot.batch.0.id == batch.0.id)
    {
        return Err(reject(SegmentedError::AlreadySubmitted, batch));
    }
    if admission.slots.iter().flatten().count() as u64
        >= worker.state.pool.policy.maximum_logical_lag_blocks
    {
        return Err(reject(SegmentedError::LogicalLag, batch));
    }
    if expected != batch.0.expected
        || expected != admission.cursor
        || batch.0.plan.marker.identity.parent != admission.logical
    {
        return Err(reject(SegmentedError::WrongCursor, batch));
    }
    let Some(slot) = admission.slots.iter().position(Option::is_none) else {
        return Err(reject(SegmentedError::QueueFull, batch));
    };
    let (reply, receiver) = mpsc::sync_channel(1);
    admission.slots[slot] = Some(AdmittedBatch {
        batch: batch.clone(),
        acknowledged: expected,
        complete: None,
    });
    let request = Request { batch, slot, reply };
    match worker.sender.try_send(request) {
        Ok(()) => {
            let retained = &admission.slots[slot]
                .as_ref()
                .expect("reserved admitted slot")
                .batch;
            let id = retained.0.id;
            let next = retained.0.marker_cursor;
            let logical = SegmentedRecoveryAnchor {
                height: retained.0.plan.marker.identity.target_height,
                cursor: next,
                state_binding: retained.0.plan.marker.target_state_binding,
            };
            admission.cursor = next;
            admission.logical = logical;
            Ok(SegmentedTicket {
                receiver,
                state: Arc::clone(&worker.state),
                slot,
                id,
            })
        }
        Err(error) => {
            let retained = admission.slots[slot].take();
            drop(admission);
            drop(retained);
            let (error, request) = match error {
                TrySendError::Full(request) => (SegmentedError::QueueFull, request),
                TrySendError::Disconnected(request) => (SegmentedError::Closed, request),
            };
            Err(reject(error, request.batch))
        }
    }
}
