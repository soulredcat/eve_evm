// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{read_worker_thread_cpu_ns::read_worker_thread_cpu_ns, types::WorkerCpuBudget};
use crate::persistence::segmented::SegmentedError;
use std::sync::atomic::AtomicU64;
pub(in crate::persistence::segmented) fn create_worker_cpu_budget(
    basis_points: u64,
) -> Result<WorkerCpuBudget, SegmentedError> {
    if basis_points == 0 || basis_points > 10_000 {
        return Err(SegmentedError::InvalidConfiguration);
    }
    read_worker_thread_cpu_ns()?;
    Ok(WorkerCpuBudget {
        basis_points,
        revision: AtomicU64::new(0),
        records: AtomicU64::new(0),
        cpu_ns: AtomicU64::new(0),
        wall_ns: AtomicU64::new(0),
        sleep_ns: AtomicU64::new(0),
        maximum_burst_ns: AtomicU64::new(0),
    })
}
