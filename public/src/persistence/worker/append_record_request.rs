// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    RecordWorkerError,
    required_record_scratch::required_record_scratch,
    types::{RecordRequest, WorkerState},
};
use crate::persistence::handoff::{borrow_recovery_record, recovery_payload_bytes};
use eve_storage::records::{
    OpaqueRecordAck, OpaqueRecordRepository, compare_and_append_opaque_records,
};
use std::sync::atomic::Ordering;

pub(super) fn append_record_request(
    repository: &mut OpaqueRecordRepository,
    state: &WorkerState,
    request: &RecordRequest,
) -> Result<OpaqueRecordAck, RecordWorkerError> {
    if state.failed.load(Ordering::Acquire) {
        return Err(RecordWorkerError::StorageFailed);
    }
    let scratch = required_record_scratch(
        recovery_payload_bytes(&request.payload).len(),
        state.repository_budget,
    )?;
    if scratch > state.scratch_limit {
        return Err(RecordWorkerError::ScratchLimit);
    }
    state.active_scratch.store(scratch, Ordering::Release);
    #[cfg(test)]
    super::tests::pause_before_append(state);
    let result = compare_and_append_opaque_records(
        repository,
        request.expected,
        borrow_recovery_record(&request.payload),
    );
    state.active_scratch.store(0, Ordering::Release);
    result.map_err(|_| {
        state.failed.store(true, Ordering::Release);
        RecordWorkerError::StorageFailed
    })
}
