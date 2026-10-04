// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{SegmentedWorkerCpuObservation, observe_worker_cpu_budget::observe_worker_cpu_budget};
use crate::persistence::segmented::{SegmentedError, SegmentedWorker};
pub fn observe_segmented_worker_cpu(
    worker: &SegmentedWorker,
) -> Result<SegmentedWorkerCpuObservation, SegmentedError> {
    observe_worker_cpu_budget(&worker.state.cpu)
}
