// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::StateBudget;

pub fn development_state_budget() -> StateBudget {
    let mib = 1_048_576;
    StateBudget {
        maximum_accounts: 65_536,
        maximum_storage_slots: 262_144,
        maximum_codes: 65_536,
        maximum_code_bytes: 24_576,
        maximum_total_code_bytes: 16 * mib,
        maximum_system_records: 65_536,
        maximum_system_bytes: 8 * mib,
        maximum_block_hashes: 10_000,
        maximum_journal_operations: 65_536,
        maximum_journal_bytes: 8 * mib,
        maximum_state_bytes: 64 * mib,
        maximum_commit_bytes: 80 * mib,
    }
}
