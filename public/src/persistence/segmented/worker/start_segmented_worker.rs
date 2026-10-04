// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::verify_startup_marker_membership::verify_startup_marker_membership;
use super::{
    super::{
        SegmentedError, SegmentedPartPool, SegmentedWorker,
        pool::{acquire_segmented_worker, estimate_segmented_metadata, reserve_metadata},
        types::{Admission, BATCHES, WorkerState},
    },
    run_segmented_worker::run_segmented_worker,
};
use eve_storage::records::segmented::SegmentedRecoveryAnchor;
use eve_storage::records::{
    OpaqueRecordRepository, opaque_record_bootstrap_cursor, opaque_record_budget,
    opaque_record_cursor, opaque_record_identity,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64},
    mpsc,
};
pub fn start_segmented_worker(
    repository: OpaqueRecordRepository,
    pool: Arc<SegmentedPartPool>,
    logical_parent: SegmentedRecoveryAnchor,
) -> Result<SegmentedWorker, SegmentedError> {
    let worker_lifetime = acquire_segmented_worker(&pool)?;
    if opaque_record_identity(&repository) != pool.namespace
        || opaque_record_budget(&repository) != pool.repository
    {
        return Err(SegmentedError::ForeignPool);
    }
    let cursor = opaque_record_cursor(&repository).map_err(|_| SegmentedError::StorageFailed)?;
    if logical_parent.cursor.sequence > cursor.sequence
        || logical_parent.state_binding == [0; 32]
        || logical_parent.cursor.content_hash == [0; 32]
        || (logical_parent.height == 0) != (logical_parent.cursor.sequence == 0)
        || (logical_parent.cursor.sequence == cursor.sequence && logical_parent.cursor != cursor)
    {
        return Err(SegmentedError::WrongCursor);
    }
    let (_, _, metadata_bytes) = estimate_segmented_metadata();
    let metadata = reserve_metadata(&pool, metadata_bytes)?;
    let state = Arc::new(WorkerState {
        pool,
        admission: Mutex::new(Admission {
            cursor,
            logical: logical_parent,
            slots: std::array::from_fn(|_| None),
        }),
        failed: AtomicBool::new(false),
        scratch: AtomicU64::new(0),
        _metadata: metadata,
        #[cfg(test)]
        pause: Mutex::new(None),
        _worker_lifetime: worker_lifetime,
    });
    if logical_parent.height == 0 {
        if logical_parent.cursor != opaque_record_bootstrap_cursor(&repository) {
            return Err(SegmentedError::WrongCursor);
        }
    } else {
        verify_startup_marker_membership(&repository, &state, logical_parent)?;
    }
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
