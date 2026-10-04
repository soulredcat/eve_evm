// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::fixtures::fixture;
use eve_state::{StateError, preflight_state_commit, state_commit_preflight_stats};

#[test]
fn exact_actual_cardinality_and_byte_limits_pass_while_one_lower_refuses_before_decode() {
    let (_, bytes, budget) = fixture();
    let stats = state_commit_preflight_stats(&preflight_state_commit(&bytes, &budget).unwrap());
    let mut exact = budget;
    exact.maximum_accounts = stats.accounts;
    exact.maximum_storage_slots = stats.storage_slots;
    exact.maximum_codes = stats.codes;
    exact.maximum_code_bytes = stats.maximum_code_blob_bytes;
    exact.maximum_total_code_bytes = stats.code_bytes;
    exact.maximum_system_records = stats.system_records;
    exact.maximum_system_bytes = stats.system_encoded_bytes;
    exact.maximum_block_hashes = stats.history_entries;
    exact.maximum_state_bytes = stats.state_encoded_bytes;
    exact.maximum_commit_bytes = stats.encoded_bytes;
    assert!(preflight_state_commit(&bytes, &exact).is_ok());
    for dimension in 0..10 {
        let mut smaller = exact;
        match dimension {
            0 => smaller.maximum_accounts -= 1,
            1 => smaller.maximum_storage_slots -= 1,
            2 => smaller.maximum_code_bytes -= 1,
            3 => smaller.maximum_system_records -= 1,
            4 => smaller.maximum_system_bytes -= 1,
            5 => smaller.maximum_block_hashes -= 1,
            6 => smaller.maximum_commit_bytes -= 1,
            7 => smaller.maximum_codes -= 1,
            8 => smaller.maximum_total_code_bytes -= 1,
            _ => smaller.maximum_state_bytes -= 1,
        }
        assert!(matches!(
            preflight_state_commit(&bytes, &smaller),
            Err(StateError::BudgetExceeded)
        ));
    }
}

#[test]
fn inconsistent_or_zero_budget_refuses_borrowed_admission() {
    let (_, bytes, budget) = fixture();
    let mut invalid = budget;
    invalid.maximum_accounts = 0;
    assert!(matches!(
        preflight_state_commit(&bytes, &invalid),
        Err(StateError::BudgetExceeded)
    ));
    let mut invalid = budget;
    invalid.maximum_commit_bytes = 1;
    assert!(preflight_state_commit(&bytes, &invalid).is_err());
}
