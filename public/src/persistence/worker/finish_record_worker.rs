// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RecordWorker, RecordWorkerError};
use eve_storage::records::OpaqueRecordRepository;
use std::sync::atomic::Ordering;

/// Consume the sole sender, drain admitted requests and join outside applied-state locks.
pub fn finish_record_worker(
    worker: RecordWorker,
) -> Result<OpaqueRecordRepository, RecordWorkerError> {
    drop(worker.sender);
    let repository = worker
        .thread
        .join()
        .map_err(|_| RecordWorkerError::WorkerPanicked)?;
    if worker.state.failed.load(Ordering::Acquire) {
        return Err(RecordWorkerError::StorageFailed);
    }
    Ok(repository)
}
