// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::WorkerLifetimeLease;
use std::sync::atomic::Ordering;

pub(super) fn release_segmented_worker(lease: &WorkerLifetimeLease) {
    lease.pool.worker_active.store(false, Ordering::Release);
}
