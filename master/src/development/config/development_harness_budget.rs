// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::development_state_budget;
use eve_storage::{recovery::types::StorageBudget, state::StateStorageBudget};

/// Bounded complete-state development oracle; this is not a production capacity profile.
pub fn development_harness_budget() -> StateStorageBudget {
    let mib = 1_048_576;
    let mut logical = development_state_budget();
    logical.maximum_accounts = 1_024;
    logical.maximum_storage_slots = 8_192;
    logical.maximum_codes = 128;
    logical.maximum_total_code_bytes = 2 * mib;
    logical.maximum_system_records = 1_024;
    logical.maximum_system_bytes = mib;
    logical.maximum_journal_operations = 8_192;
    logical.maximum_journal_bytes = 2 * mib;
    logical.maximum_state_bytes = 8 * mib;
    logical.maximum_commit_bytes = 16 * mib;
    StateStorageBudget {
        database: StorageBudget {
            max_record_bytes: 16 * mib,
            max_batch_bytes: 32 * mib,
            max_batch_records: 32_768,
            write_buffer_bytes: 4 * mib,
            write_buffer_count: 2,
            block_cache_bytes: 4 * mib,
            max_background_jobs: 2,
            max_open_files: 32,
        },
        logical,
        maximum_commit_bytes: 16 * mib,
        maximum_snapshot_bytes: 32 * mib,
        maximum_snapshots: 2,
        maximum_snapshot_files: 128,
    }
}
