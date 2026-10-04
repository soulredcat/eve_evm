// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{SegmentedWorkerCpuObservation, types::WorkerCpuBudget};
use crate::persistence::segmented::SegmentedError;
use std::sync::atomic::Ordering;
pub(in crate::persistence::segmented) fn observe_worker_cpu_budget(
    budget: &WorkerCpuBudget,
) -> Result<SegmentedWorkerCpuObservation, SegmentedError> {
    for _ in 0..32 {
        let revision = budget.revision.load(Ordering::SeqCst);
        if !revision.is_multiple_of(2) {
            std::hint::spin_loop();
            continue;
        }
        let result = SegmentedWorkerCpuObservation {
            basis_points: budget.basis_points,
            completed_records: budget.records.load(Ordering::SeqCst),
            measured_cpu_ns: budget.cpu_ns.load(Ordering::SeqCst),
            measured_wall_ns: budget.wall_ns.load(Ordering::SeqCst),
            paced_sleep_ns: budget.sleep_ns.load(Ordering::SeqCst),
            maximum_record_burst_cpu_ns: budget.maximum_burst_ns.load(Ordering::SeqCst),
        };
        if budget.revision.load(Ordering::SeqCst) == revision {
            return Ok(result);
        }
    }
    Err(SegmentedError::AccountingUnavailable)
}
