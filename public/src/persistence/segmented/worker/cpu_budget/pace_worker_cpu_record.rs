// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    read_worker_thread_cpu_ns::read_worker_thread_cpu_ns,
    record_worker_cpu_measurement::record_worker_cpu_measurement,
    required_worker_cpu_pacing_ns::required_worker_cpu_pacing_ns,
    types::{WorkerCpuBudget, WorkerCpuRecordWindow},
};
use crate::persistence::segmented::SegmentedError;
use std::time::{Duration, Instant};

/// Called only by the sole writer, after one bounded record operation and outside RAM/admission locks.
pub(in crate::persistence::segmented) fn pace_worker_cpu_record(
    budget: &WorkerCpuBudget,
    window: WorkerCpuRecordWindow,
) -> Result<(), SegmentedError> {
    let cpu = read_worker_thread_cpu_ns()?
        .checked_sub(window.cpu_ns)
        .ok_or(SegmentedError::Overflow)?;
    let before_sleep =
        u64::try_from(window.wall.elapsed().as_nanos()).map_err(|_| SegmentedError::Overflow)?;
    let requested_sleep = required_worker_cpu_pacing_ns(cpu, before_sleep, budget.basis_points)?;
    let sleep_started = Instant::now();
    if requested_sleep > 0 {
        std::thread::sleep(Duration::from_nanos(requested_sleep));
    }
    let sleep = if requested_sleep > 0 {
        u64::try_from(sleep_started.elapsed().as_nanos()).map_err(|_| SegmentedError::Overflow)?
    } else {
        0
    };
    let wall =
        u64::try_from(window.wall.elapsed().as_nanos()).map_err(|_| SegmentedError::Overflow)?;
    record_worker_cpu_measurement(budget, cpu, wall, sleep)
}
