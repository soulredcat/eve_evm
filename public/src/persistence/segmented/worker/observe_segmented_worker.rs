// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedError, SegmentedWorker, SegmentedWorkerObservation};
use std::sync::atomic::Ordering;
pub fn observe_segmented_worker(
    worker: &SegmentedWorker,
) -> Result<SegmentedWorkerObservation, SegmentedError> {
    let admission = worker
        .state
        .admission
        .lock()
        .map_err(|_| SegmentedError::AccountingUnavailable)?;
    Ok(SegmentedWorkerObservation {
        active_estimated_scratch_bytes: worker.state.scratch.load(Ordering::Acquire),
        scratch_limit: worker.state.pool.policy.maximum_scratch_bytes,
        retained_logical_batches: admission.slots.iter().flatten().count(),
        storage_failed: worker.state.failed.load(Ordering::Acquire) || worker.thread.is_finished(),
    })
}
