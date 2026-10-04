// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::fixtures::fixture;
use eve_state::{
    decode_preflight_state_commit, development_state_budget, encode_state_commit,
    preflight_state_commit, required_state_commit_decode_reservation,
    state_commit_preflight_budget, state_commit_preflight_bytes, state_commit_preflight_stats,
};

#[test]
fn sealed_full_commit_roundtrip_reports_actual_materialization_counts_without_new_authority() {
    let (commit, bytes, budget) = fixture();
    let preflight = preflight_state_commit(&bytes, &budget).unwrap();
    let stats = state_commit_preflight_stats(&preflight);
    assert_eq!(
        state_commit_preflight_bytes(&preflight).as_ptr(),
        bytes.as_ptr()
    );
    assert_eq!(stats.encoded_bytes, bytes.len());
    assert_eq!(stats.accounts, commit.state.accounts.len());
    assert_eq!(
        stats.storage_slots,
        commit
            .state
            .accounts
            .values()
            .map(|account| account.storage.len())
            .sum::<usize>()
    );
    assert_eq!(stats.codes, commit.state.codes.len());
    assert_eq!(
        stats.code_bytes,
        commit
            .state
            .codes
            .values()
            .map(|code| code.len())
            .sum::<usize>()
    );
    assert_eq!(stats.system_records, commit.state.system.len());
    assert_eq!(stats.history_entries, commit.state.block_hashes.len());
    assert_eq!(
        stats.parent_network_bytes,
        commit.parent.as_ref().unwrap().identity.network_name.len()
    );
    assert_eq!(
        stats.target_network_bytes,
        commit.target.identity.network_name.len()
    );
    assert_eq!(
        stats.state_network_bytes,
        commit.state.identity.network_name.len()
    );
    assert_eq!(stats.transaction_count, 0);
    assert_eq!(stats.receipt_count, 0);
    assert!(stats.system_leaf_count > stats.system_records);
    assert!(required_state_commit_decode_reservation(&preflight).unwrap() >= bytes.len() * 6);
    assert_eq!(decode_preflight_state_commit(&preflight).unwrap(), commit);
}

#[test]
fn frozen_budget_does_not_follow_later_caller_budget_mutation() {
    let (_, bytes, mut budget) = fixture();
    let preflight = preflight_state_commit(&bytes, &budget).unwrap();
    let frozen = budget.maximum_accounts;
    budget.maximum_accounts = 1;
    assert_eq!(
        state_commit_preflight_budget(&preflight).maximum_accounts,
        frozen
    );
    assert!(preflight_state_commit(&bytes, &budget).is_err());
    assert!(decode_preflight_state_commit(&preflight).is_ok());
}

#[test]
fn genesis_has_no_parent_allocation_and_keeps_same_canonical_codec() {
    let commit = crate::support::genesis();
    let budget = development_state_budget();
    let bytes = encode_state_commit(&commit, &budget).unwrap();
    let preflight = preflight_state_commit(&bytes, &budget).unwrap();
    assert_eq!(
        state_commit_preflight_stats(&preflight).parent_network_bytes,
        0
    );
    assert_eq!(decode_preflight_state_commit(&preflight).unwrap(), commit);
}
