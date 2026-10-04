// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// Real finite logical capacity, rounded upward to KiB before any covered allocation.
pub(in crate::sync) fn reserve_master_bytes(
    pool: &Arc<Semaphore>,
    bytes: usize,
) -> Result<OwnedSemaphorePermit> {
    let units = bytes
        .checked_add(1_023)
        .and_then(|value| u32::try_from(value / 1_024).ok())
        .ok_or_else(|| anyhow::anyhow!("MASTER_RESOURCE_ARITHMETIC"))?;
    ensure!(units > 0, "MASTER_RESOURCE_EMPTY");
    Arc::clone(pool)
        .try_acquire_many_owned(units)
        .map_err(|_| anyhow::anyhow!("MASTER_RESOURCE_CAPACITY"))
}
