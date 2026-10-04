// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{sync::atomic::AtomicU64, time::Instant};

pub(in crate::persistence::segmented) struct WorkerCpuBudget {
    pub(super) basis_points: u64,
    pub(super) revision: AtomicU64,
    pub(super) records: AtomicU64,
    pub(super) cpu_ns: AtomicU64,
    pub(super) wall_ns: AtomicU64,
    pub(super) sleep_ns: AtomicU64,
    pub(super) maximum_burst_ns: AtomicU64,
}
pub(in crate::persistence::segmented) struct WorkerCpuRecordWindow {
    pub(super) cpu_ns: u64,
    pub(super) wall: Instant,
}
/// Real sole-writer record-operation accounting; DB background threads are outside its scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedWorkerCpuObservation {
    pub basis_points: u64,
    pub completed_records: u64,
    pub measured_cpu_ns: u64,
    pub measured_wall_ns: u64,
    pub paced_sleep_ns: u64,
    pub maximum_record_burst_cpu_ns: u64,
}
