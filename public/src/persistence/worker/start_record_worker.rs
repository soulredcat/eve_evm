// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    RecordWorker, RecordWorkerError, run_record_worker::run_record_worker, types::WorkerState,
    validate_worker_resources::validate_worker_resources,
};
use crate::persistence::handoff::{HandoffPool, handoff_budget};
use eve_storage::records::{OpaqueRecordRepository, opaque_record_budget};
use std::{
    collections::BTreeSet,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64},
        mpsc,
    },
    thread,
};

pub fn start_record_worker(
    repository: OpaqueRecordRepository,
    pool: Arc<HandoffPool>,
    scratch_limit: u64,
) -> Result<RecordWorker, RecordWorkerError> {
    let public = handoff_budget(&pool);
    let repository_budget = opaque_record_budget(&repository);
    validate_worker_resources(public, repository_budget, scratch_limit)?;
    let capacity = usize::try_from(public.queue_batches)
        .map_err(|_| RecordWorkerError::InvalidConfiguration)?;
    let (sender, receiver) = mpsc::sync_channel(capacity);
    let state = Arc::new(WorkerState {
        pool,
        repository_budget,
        scratch_limit,
        active_scratch: AtomicU64::new(0),
        failed: AtomicBool::new(false),
        submitted: Mutex::new(BTreeSet::new()),
        #[cfg(test)]
        pause: std::sync::Mutex::new(None),
    });
    let worker_state = Arc::clone(&state);
    let thread = thread::Builder::new()
        .name("eve-public-record-writer".to_owned())
        .spawn(move || run_record_worker(repository, receiver, worker_state))
        .map_err(|_| RecordWorkerError::SpawnFailed)?;
    Ok(RecordWorker {
        sender,
        thread,
        state,
    })
}
