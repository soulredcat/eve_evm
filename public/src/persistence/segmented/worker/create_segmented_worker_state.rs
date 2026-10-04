// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedError, SegmentedPartPool,
    pool::{estimate_segmented_metadata, reserve_metadata},
    types::{Admission, WorkerLifetimeLease, WorkerState},
};
use eve_storage::records::{OpaqueRecordCursor, segmented::SegmentedRecoveryAnchor};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64},
};
pub(in crate::persistence::segmented) fn create_segmented_worker_state(
    pool: Arc<SegmentedPartPool>,
    cursor: OpaqueRecordCursor,
    logical: SegmentedRecoveryAnchor,
    worker_lifetime: WorkerLifetimeLease,
) -> Result<Arc<WorkerState>, SegmentedError> {
    let (_, _, metadata_bytes) = estimate_segmented_metadata();
    let metadata = reserve_metadata(&pool, metadata_bytes)?;
    let cpu = super::cpu_budget::create_worker_cpu_budget(pool.worker_cpu_basis_points)?;
    Ok(Arc::new(WorkerState {
        cpu,
        pool,
        admission: Mutex::new(Admission {
            cursor,
            logical,
            slots: std::array::from_fn(|_| None),
            checkpoint: None,
        }),
        failed: AtomicBool::new(false),
        scratch: AtomicU64::new(0),
        _metadata: metadata,
        #[cfg(test)]
        pause: Mutex::new(None),
        _worker_lifetime: worker_lifetime,
    }))
}
