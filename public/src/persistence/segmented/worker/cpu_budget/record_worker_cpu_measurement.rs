// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::WorkerCpuBudget;
use crate::persistence::segmented::SegmentedError;
use std::sync::atomic::Ordering;

/// One sole writer publishes a checked coherent counter generation.
pub(super) fn record_worker_cpu_measurement(
    budget: &WorkerCpuBudget,
    cpu: u64,
    wall: u64,
    sleep: u64,
) -> Result<(), SegmentedError> {
    let revision = budget.revision.load(Ordering::SeqCst);
    let completed = budget
        .records
        .load(Ordering::SeqCst)
        .checked_add(1)
        .ok_or(SegmentedError::Overflow)?;
    let cpu_total = budget
        .cpu_ns
        .load(Ordering::SeqCst)
        .checked_add(cpu)
        .ok_or(SegmentedError::Overflow)?;
    let wall_total = budget
        .wall_ns
        .load(Ordering::SeqCst)
        .checked_add(wall)
        .ok_or(SegmentedError::Overflow)?;
    let sleep_total = budget
        .sleep_ns
        .load(Ordering::SeqCst)
        .checked_add(sleep)
        .ok_or(SegmentedError::Overflow)?;
    let next_revision = revision.checked_add(2).ok_or(SegmentedError::Overflow)?;
    let maximum = budget.maximum_burst_ns.load(Ordering::SeqCst).max(cpu);
    if !revision.is_multiple_of(2) {
        return Err(SegmentedError::AccountingUnavailable);
    }
    budget.revision.store(revision + 1, Ordering::SeqCst);
    budget.records.store(completed, Ordering::SeqCst);
    budget.cpu_ns.store(cpu_total, Ordering::SeqCst);
    budget.wall_ns.store(wall_total, Ordering::SeqCst);
    budget.sleep_ns.store(sleep_total, Ordering::SeqCst);
    budget.maximum_burst_ns.store(maximum, Ordering::SeqCst);
    budget.revision.store(next_revision, Ordering::SeqCst);
    Ok(())
}
