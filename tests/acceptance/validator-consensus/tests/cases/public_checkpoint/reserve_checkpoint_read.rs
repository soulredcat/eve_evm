// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{CHECKPOINT_READ_LIMIT, CheckpointReadBudget, CheckpointReadLease};
use anyhow::{Context, Result, ensure};

pub(super) fn reserve_checkpoint_read(
    budget: &CheckpointReadBudget,
    bytes: usize,
) -> Result<CheckpointReadLease<'_>> {
    let total = budget
        .used
        .get()
        .checked_add(bytes)
        .context("PUBLIC_CHECKPOINT_READ_RESERVATION_OVERFLOW")?;
    ensure!(
        total <= CHECKPOINT_READ_LIMIT,
        "PUBLIC_CHECKPOINT_READ_CAPACITY"
    );
    budget.used.set(total);
    Ok(CheckpointReadLease { budget, bytes })
}
