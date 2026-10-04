// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Cooperative safe ThreadCPUTime pacing of bounded sole-writer record operations.
mod begin_worker_cpu_record;
mod create_worker_cpu_budget;
mod observe_segmented_worker_cpu;
mod observe_worker_cpu_budget;
mod pace_worker_cpu_record;
mod read_worker_thread_cpu_ns;
mod record_worker_cpu_measurement;
mod required_worker_cpu_pacing_ns;
#[cfg(test)]
mod tests;
mod types;
pub(in crate::persistence::segmented) use begin_worker_cpu_record::begin_worker_cpu_record;
pub(in crate::persistence::segmented) use create_worker_cpu_budget::create_worker_cpu_budget;
pub use observe_segmented_worker_cpu::observe_segmented_worker_cpu;
pub(in crate::persistence::segmented) use pace_worker_cpu_record::pace_worker_cpu_record;
pub use types::SegmentedWorkerCpuObservation;
pub(in crate::persistence::segmented) use types::WorkerCpuBudget;
