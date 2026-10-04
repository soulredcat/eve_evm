// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{BudgetError, PublicBudget};

/// Derive unused declared v1 pool allowance without changing any existing pool.
pub fn available_segmented_allowance(base: PublicBudget) -> Result<u64, BudgetError> {
    crate::validate_public_budget(base)?;
    let write_buffers = base
        .write_buffer_bytes
        .checked_mul(base.write_buffer_count)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let reserved = [
        write_buffers,
        base.block_cache_bytes,
        base.maximum_batch_bytes,
        base.queue_bytes,
        base.maximum_working_state_bytes,
        base.query_cache_bytes,
        base.mempool_bytes,
        base.simulation_overlay_bytes,
        base.snapshot_staging_bytes,
        base.global_bulk_inflight_bytes,
        base.global_ordinary_inflight_bytes,
    ]
    .into_iter()
    .try_fold(0_u64, |sum, bytes| sum.checked_add(bytes))
    .ok_or(BudgetError::ArithmeticOverflow)?;
    base.process_memory_budget_bytes
        .checked_sub(reserved)
        .ok_or(BudgetError::InconsistentBound)
}
