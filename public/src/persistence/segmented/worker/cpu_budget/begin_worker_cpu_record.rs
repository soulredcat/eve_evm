// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{read_worker_thread_cpu_ns::read_worker_thread_cpu_ns, types::WorkerCpuRecordWindow};
use crate::persistence::segmented::SegmentedError;
use std::time::Instant;
pub(in crate::persistence::segmented) fn begin_worker_cpu_record()
-> Result<WorkerCpuRecordWindow, SegmentedError> {
    let wall = Instant::now();
    let cpu_ns = read_worker_thread_cpu_ns()?;
    Ok(WorkerCpuRecordWindow { wall, cpu_ns })
}
