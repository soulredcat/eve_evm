// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedError, SegmentedPartPool, types::WorkerLifetimeLease};
use std::sync::{Arc, atomic::Ordering};

/// One lifetime owner for the pool's declared scratch envelope, across paths and threads.
pub(in crate::persistence::segmented) fn acquire_segmented_worker(
    pool: &Arc<SegmentedPartPool>,
) -> Result<WorkerLifetimeLease, SegmentedError> {
    pool.worker_active
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .map_err(|_| SegmentedError::WorkerAlreadyActive)?;
    Ok(WorkerLifetimeLease {
        pool: Arc::clone(pool),
    })
}
