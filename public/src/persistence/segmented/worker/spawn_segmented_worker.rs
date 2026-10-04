// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        SegmentedError, SegmentedWorker,
        types::{BATCHES, WorkerState},
    },
    run_segmented_worker::run_segmented_worker,
};
use eve_storage::records::OpaqueRecordRepository;
use std::sync::{Arc, mpsc};
pub(in crate::persistence::segmented) fn spawn_segmented_worker(
    repository: OpaqueRecordRepository,
    state: Arc<WorkerState>,
) -> Result<SegmentedWorker, SegmentedError> {
    let (sender, receiver) = mpsc::sync_channel(BATCHES);
    let worker_state = Arc::clone(&state);
    let thread = std::thread::Builder::new()
        .name("eve-public-segment-writer".into())
        .spawn(move || run_segmented_worker(repository, worker_state, receiver))
        .map_err(|_| SegmentedError::SpawnFailed)?;
    Ok(SegmentedWorker {
        sender,
        state,
        thread,
    })
}
