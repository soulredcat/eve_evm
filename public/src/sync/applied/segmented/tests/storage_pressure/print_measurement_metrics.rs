// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::PhaseMetrics;
use crate::{
    persistence::segmented::SegmentedWorkerCpuObservation,
    sync::applied::{AppliedStorageObservation, EstimatedWorkingObservation},
};
pub(super) fn print_measurement_metrics(
    baseline: &PhaseMetrics,
    pressure: &PhaseMetrics,
    cpu: SegmentedWorkerCpuObservation,
    storage: AppliedStorageObservation,
    working: EstimatedWorkingObservation,
    queue_age_ms: u128,
    maintenance: super::archive_types::ArchiveJobCounts,
) {
    println!(
        "B4_T_N09_METRICS baseline_balance_p99_ns={} baseline_call_p99_ns={} pressure_balance_p99_ns={} pressure_call_p99_ns={} sampled_peak_rss_bytes={} process_lifetime_hwm_bytes={} storage_peak_reads={} stage_peak_bytes={} working_peak_estimated_bytes={} writer_cpu_ns={} writer_wall_ns={} writer_sleep_ns={} writer_max_record_burst_cpu_ns={} writer_record_operations={} queue_observed_age_ms={}",
        baseline.balance_p99_ns,
        baseline.call_p99_ns,
        pressure.balance_p99_ns,
        pressure.call_p99_ns,
        baseline
            .sampled_peak_rss_bytes
            .max(pressure.sampled_peak_rss_bytes),
        baseline
            .process_lifetime_hwm_bytes
            .max(pressure.process_lifetime_hwm_bytes),
        storage.peak_reads,
        storage.peak_staging_bytes,
        working.peak_estimated_bytes,
        cpu.measured_cpu_ns,
        cpu.measured_wall_ns,
        cpu.paced_sleep_ns,
        cpu.maximum_record_burst_cpu_ns,
        cpu.completed_records,
        queue_age_ms
    );
    println!(
        "B4_T_N09_MAINTENANCE_METRICS archive_compaction_operations={} archive_secondary_index_jobs={}",
        maintenance.compactions, maintenance.index_lookups
    );
}
