// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::cell::Cell;

/// Independent single-thread test-reader logical capacity. It is not the child
/// runtime pool, allocated bytes or a process/RSS measurement.
pub(super) struct CheckpointReadBudget {
    pub(super) used: Cell<usize>,
}

pub(super) struct CheckpointReadLease<'a> {
    pub(super) budget: &'a CheckpointReadBudget,
    pub(super) bytes: usize,
}

pub(super) const CHECKPOINT_READ_LIMIT: usize = 128 * 1_048_576;
pub(super) const CHECKPOINT_FIXTURE_BODY_LIMIT: usize = 4 * 1_048_576;
