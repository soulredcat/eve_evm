// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;
use eve_state::{
    B256, JournalOperation, StateJournal, U256, apply_state_journal, development_state_budget,
    project_state_journal,
};

#[test]
fn metadata_and_single_slot_updates_preserve_untouched_slots_and_zero_deletes() {
    let parent = support::with_slots();
    let old = &parent.state.accounts[&support::contract()];
    let journal = StateJournal {
        parent: parent.target.clone(),
        target_height: 2,
        operations: vec![
            JournalOperation::PutAccount {
                address: support::contract(),
                nonce: 2,
                balance: old.balance,
                code_hash: old.code_hash,
            },
            JournalOperation::PutStorage {
                address: support::contract(),
                slot: U256::ZERO,
                value: U256::from(99),
            },
        ],
    };
    let budget = development_state_budget();
    let state = apply_state_journal(&parent.state, &parent.target, &journal, &budget).unwrap();
    assert_eq!(
        state.accounts[&support::contract()].storage[&U256::from(1)],
        U256::from(11)
    );
    let projected =
        project_state_journal(&parent.state, &parent.target, &state, 2, &budget).unwrap();
    assert_eq!(
        apply_state_journal(&parent.state, &parent.target, &projected, &budget).unwrap(),
        state
    );
    let deletion = StateJournal {
        operations: vec![JournalOperation::PutStorage {
            address: support::contract(),
            slot: U256::ZERO,
            value: U256::ZERO,
        }],
        ..journal
    };
    let deleted = apply_state_journal(&parent.state, &parent.target, &deletion, &budget).unwrap();
    assert!(
        !deleted.accounts[&support::contract()]
            .storage
            .contains_key(&U256::ZERO)
    );
}

#[test]
fn delete_recreate_and_explicit_clear_reset_storage_without_partial_parent_writes() {
    let parent = support::with_slots();
    let original = parent.state.clone();
    let old = &parent.state.accounts[&support::contract()];
    let journal = StateJournal {
        parent: parent.target.clone(),
        target_height: 2,
        operations: vec![
            JournalOperation::DeleteAccount {
                address: support::contract(),
            },
            JournalOperation::PutAccount {
                address: support::contract(),
                nonce: 0,
                balance: old.balance,
                code_hash: old.code_hash,
            },
        ],
    };
    let budget = development_state_budget();
    let recreated = apply_state_journal(&parent.state, &parent.target, &journal, &budget).unwrap();
    assert!(recreated.accounts[&support::contract()].storage.is_empty());
    let invalid = StateJournal {
        operations: vec![
            JournalOperation::ClearStorage {
                address: support::contract(),
            },
            JournalOperation::PutAccount {
                address: support::contract(),
                nonce: 0,
                balance: old.balance,
                code_hash: B256::repeat_byte(9),
            },
        ],
        ..journal
    };
    assert!(apply_state_journal(&parent.state, &parent.target, &invalid, &budget).is_err());
    assert_eq!(parent.state, original);
}

#[test]
fn stale_versions_and_budget_excess_reject_before_journal_serialization_or_apply() {
    let parent = support::with_slots();
    let mut journal = StateJournal {
        parent: parent.target.clone(),
        target_height: 2,
        operations: vec![JournalOperation::ClearStorage {
            address: support::contract(),
        }],
    };
    let mut budget = development_state_budget();
    budget.maximum_journal_bytes = 1_023;
    assert!(apply_state_journal(&parent.state, &parent.target, &journal, &budget).is_err());
    budget = development_state_budget();
    journal.parent.content_digest = B256::repeat_byte(7);
    assert!(apply_state_journal(&parent.state, &parent.target, &journal, &budget).is_err());
}
