// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedError, types::WorkerState};
use super::{persist_checkpoint_base::persist_checkpoint_base, types::CheckpointRequest};
use eve_storage::records::OpaqueRecordRepository;
use std::sync::{Arc, atomic::Ordering};
pub(in crate::persistence::segmented) fn dispatch_checkpoint_request(
    repository: &mut OpaqueRecordRepository,
    state: &Arc<WorkerState>,
    request: CheckpointRequest,
) {
    let result = if state.failed.load(Ordering::Acquire) {
        Err(SegmentedError::StorageFailed)
    } else {
        persist_checkpoint_base(repository, state, &request)
    };
    let result = match result {
        Ok(ack) => match state.admission.lock() {
            Ok(mut admission) => match admission.checkpoint.as_mut() {
                Some(slot) if slot.record.0.id == request.record.0.id => {
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
    // The admission retains the immutable charged record even when delivery is cancelled.
    let _ = request.reply.try_send(result);
}
