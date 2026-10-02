// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RecordWorker, RecordWorkerObservation};
use std::sync::atomic::Ordering;

pub fn observe_record_worker(worker: &RecordWorker) -> RecordWorkerObservation {
    RecordWorkerObservation {
        active_scratch_bytes: worker.state.active_scratch.load(Ordering::Acquire),
        scratch_limit: worker.state.scratch_limit,
        storage_failed: worker.state.failed.load(Ordering::Acquire),
    }
}
