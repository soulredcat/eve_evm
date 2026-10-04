// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::Result;
use eve_storage::state::StateStorageBudget;

/// Conservative complete-commit decode/root/projection/batch envelope kept by the owner.
/// RocksDB configured cache and write buffers are separately declared, not measured RSS.
pub(in crate::sync) fn storage_working_bytes(budget: &StateStorageBudget) -> Result<usize> {
    let logical = budget.logical;
    let terms = [
        (budget.maximum_commit_bytes, 12),
        (budget.database.max_batch_bytes, 2),
        (logical.maximum_state_bytes, 8),
        (logical.maximum_accounts, 1_024),
        (logical.maximum_storage_slots, 512),
        (logical.maximum_codes, 1_024),
        (logical.maximum_total_code_bytes, 4),
        (logical.maximum_system_records, 1_024),
        (logical.maximum_system_bytes, 4),
        (logical.maximum_block_hashes, 256),
    ];
    terms
        .into_iter()
        .try_fold(2_097_152_usize, |sum, (count, factor)| {
            sum.checked_add(count.checked_mul(factor)?)
        })
        .ok_or_else(|| anyhow::anyhow!("MASTER_STORAGE_RESOURCE_ARITHMETIC"))
}
