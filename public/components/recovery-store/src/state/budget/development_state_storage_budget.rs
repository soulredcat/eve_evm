// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateStorageBudget;
use crate::recovery::types::StorageBudget;
use eve_state::development_state_budget;

/// Bounded complete-state development baseline; not a mainnet capacity assertion.
pub fn development_state_storage_budget() -> StateStorageBudget {
    let mib = 1_048_576;
    StateStorageBudget {
        database: StorageBudget {
            max_record_bytes: 80 * mib,
            max_batch_bytes: 160 * mib,
            max_batch_records: 1,
            write_buffer_bytes: 8 * mib,
            write_buffer_count: 2,
            block_cache_bytes: 8 * mib,
            max_background_jobs: 2,
            max_open_files: 128,
        },
        logical: development_state_budget(),
        maximum_commit_bytes: 160 * mib,
        maximum_snapshot_bytes: 512 * mib,
        maximum_snapshots: 8,
        maximum_snapshot_files: 16_384,
    }
}
