// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    RecordTicket, RecordWorker, RecordWorkerError, RejectedRecord,
    required_record_scratch::required_record_scratch, types::RecordRequest,
};
use crate::persistence::handoff::{
    RecoveryPayload, payload_belongs_to_pool, payload_reservation_id, recovery_payload_bytes,
};
use eve_storage::records::OpaqueRecordCursor;
use std::sync::{
    atomic::Ordering,
    mpsc::{self, TrySendError},
};

/// Immediate bounded admission; callers retain payload ownership on every rejection.
pub fn try_submit_record(
    worker: &RecordWorker,
    expected: OpaqueRecordCursor,
    payload: RecoveryPayload,
) -> Result<RecordTicket, RejectedRecord> {
    let failure = if !payload_belongs_to_pool(&payload, &worker.state.pool) {
        Some(RecordWorkerError::ForeignPool)
    } else if worker.state.failed.load(Ordering::Acquire) {
        Some(RecordWorkerError::StorageFailed)
    } else {
        let length = recovery_payload_bytes(&payload).len();
        if length
            .checked_add(88)
            .is_none_or(|encoded| encoded > worker.state.repository_budget.maximum_record_bytes)
        {
            Some(RecordWorkerError::RecordLimit)
        } else {
            match required_record_scratch(length, worker.state.repository_budget) {
                Ok(bytes) if bytes <= worker.state.scratch_limit => None,
                _ => Some(RecordWorkerError::ScratchLimit),
            }
        }
    };
    if let Some(error) = failure {
        return Err(RejectedRecord { error, payload });
    }
    let id = payload_reservation_id(&payload);
    let mut submitted = match worker.state.submitted.lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(RejectedRecord {
                error: RecordWorkerError::AdmissionUnavailable,
                payload,
            });
        }
    };
    if !submitted.insert(id) {
        return Err(RejectedRecord {
            error: RecordWorkerError::AlreadySubmitted,
            payload,
        });
    }
    let (reply, receiver) = mpsc::sync_channel(1);
    let request = RecordRequest {
        expected,
        payload,
        reply,
    };
    match worker.sender.try_send(request) {
        Ok(()) => Ok(RecordTicket { receiver }),
        Err(TrySendError::Full(request)) => {
            submitted.remove(&id);
            Err(RejectedRecord {
                error: RecordWorkerError::QueueFull,
                payload: request.payload,
            })
        }
        Err(TrySendError::Disconnected(request)) => {
            submitted.remove(&id);
            Err(RejectedRecord {
                error: RecordWorkerError::Closed,
                payload: request.payload,
            })
        }
    }
}
