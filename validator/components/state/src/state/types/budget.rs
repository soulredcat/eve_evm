// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Operator bounds for complete-state/reference execution work, not consensus rules.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateBudget {
    pub maximum_accounts: usize,
    pub maximum_storage_slots: usize,
    pub maximum_codes: usize,
    pub maximum_code_bytes: usize,
    pub maximum_total_code_bytes: usize,
    pub maximum_system_records: usize,
    pub maximum_system_bytes: usize,
    pub maximum_block_hashes: usize,
    pub maximum_journal_operations: usize,
    pub maximum_journal_bytes: usize,
    pub maximum_state_bytes: usize,
    pub maximum_commit_bytes: usize,
}
