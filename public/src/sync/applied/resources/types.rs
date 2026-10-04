// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::{Arc, Mutex};

pub(in crate::sync::applied) struct EstimatedWorkingPool {
    pub(super) limit: u64,
    pub(super) accounting: Mutex<EstimatedAccounting>,
}

pub(super) struct EstimatedAccounting {
    pub(super) bytes: u64,
    pub(super) peak: u64,
}

pub(in crate::sync::applied) struct EstimatedWorkingLease {
    pub(super) pool: Arc<EstimatedWorkingPool>,
    pub(super) bytes: u64,
}

/// Sealed capacity lease, independent of execution/finality and retained until caller drop.
pub struct AppliedWorkingReservation {
    pub(super) _lease: EstimatedWorkingLease,
}

pub(in crate::sync::applied) struct EstimatedReplayCharge {
    pub(in crate::sync::applied) retained: usize,
    pub(in crate::sync::applied) total: usize,
}

pub(in crate::sync::applied) struct EstimatedImportCharge {
    pub(in crate::sync::applied) candidate: usize,
    pub(in crate::sync::applied) retained: usize,
    pub(in crate::sync::applied) total: usize,
}

/// Reserved logical estimates, not allocated bytes, RSS, CPU or OS-cache enforcement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstimatedWorkingObservation {
    pub reserved_estimated_bytes: u64,
    pub peak_estimated_bytes: u64,
    pub limit: u64,
}
