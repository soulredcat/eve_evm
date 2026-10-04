// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        SegmentedError,
        types::{Request, WorkerState},
    },
    persist_segmented_batch::persist_segmented_batch,
};
use eve_storage::records::OpaqueRecordRepository;
use std::sync::{Arc, atomic::Ordering, mpsc::Receiver};
pub(super) fn run_segmented_worker(
    mut repository: OpaqueRecordRepository,
    state: Arc<WorkerState>,
    receiver: Receiver<Request>,
) -> OpaqueRecordRepository {
    for request in receiver {
        let result = if state.failed.load(Ordering::Acquire) {
            Err(SegmentedError::StorageFailed)
        } else {
            persist_segmented_batch(&mut repository, &state, &request)
        };
        let result = match result {
            Ok(ack) => match state.admission.lock() {
                Ok(mut admission) => match admission.slots[request.slot].as_mut() {
                    Some(slot) if slot.batch.0.id == request.batch.0.id => {
                        slot.complete = Some(ack);
                        Ok(ack)
                    }
                    _ => Err(SegmentedError::AckMismatch),
                },
                Err(_) => Err(SegmentedError::AccountingUnavailable),
            },
            Err(error) => Err(error),
        };
        if result.is_err() {
            state.failed.store(true, Ordering::Release);
        }
        // State slots retain the full batch through failed/panicked/cancelled deliveries.
        let _ = request.reply.try_send(result);
    }
    repository
}
