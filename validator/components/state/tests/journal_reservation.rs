// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;
use eve_state::{
    Address, JournalOperation, StateError, StateJournal, U256, apply_state_journal,
    apply_state_journal_reserved, development_state_budget, estimate_journal_candidate_reservation,
};

#[test]
fn malformed_parent_identity_rejects_before_candidate_measurement_even_with_zero_reservation() {
    let parent = support::with_slots();
    let mut malformed = parent.state.clone();
    malformed.identity.network_name = "x".repeat(65_536);
    let journal = StateJournal {
        parent: parent.target.clone(),
        target_height: 2,
        operations: Vec::new(),
    };
    let budget = development_state_budget();
    assert_eq!(
        estimate_journal_candidate_reservation(&malformed, &journal, &budget),
        Err(StateError::InvalidIdentity)
    );
    assert_eq!(
        apply_state_journal_reserved(&malformed, &parent.target, &journal, &budget, 0),
        Err(StateError::InvalidIdentity)
    );
    assert_eq!(parent.state.identity.network_name, "eve-local-v1");
}

#[test]
fn exact_candidate_reservation_matches_existing_application_and_one_byte_less_rejects() {
    let parent = support::with_slots();
    let before = parent.state.clone();
    let budget = development_state_budget();
    let journal = StateJournal {
        parent: parent.target.clone(),
        target_height: 2,
        operations: vec![JournalOperation::PutStorage {
            address: support::contract(),
            slot: U256::ZERO,
            value: U256::from(99),
        }],
    };
    let required =
        estimate_journal_candidate_reservation(&parent.state, &journal, &budget).unwrap();
    assert_eq!(
        apply_state_journal_reserved(
            &parent.state,
            &parent.target,
            &journal,
            &budget,
            required - 1
        ),
        Err(StateError::BudgetExceeded)
    );
    let reserved =
        apply_state_journal_reserved(&parent.state, &parent.target, &journal, &budget, required)
            .unwrap();
    assert_eq!(
        reserved,
        apply_state_journal(&parent.state, &parent.target, &journal, &budget).unwrap()
    );
    assert_eq!(parent.state, before);
}

#[test]
fn transient_insert_delete_flood_is_charged_even_when_the_final_state_is_unchanged() {
    let parent = support::with_slots();
    let budget = development_state_budget();
    let empty = StateJournal {
        parent: parent.target.clone(),
        target_height: 2,
        operations: Vec::new(),
    };
    let small = estimate_journal_candidate_reservation(&parent.state, &empty, &budget).unwrap();
    let mut journal = empty;
    for seed in 10..40_u8 {
        let address = Address::repeat_byte(seed);
        journal.operations.push(JournalOperation::PutAccount {
            address,
            nonce: 0,
            balance: U256::ZERO,
            code_hash: alloy_primitives::KECCAK256_EMPTY,
        });
        journal
            .operations
            .push(JournalOperation::DeleteAccount { address });
    }
    let required =
        estimate_journal_candidate_reservation(&parent.state, &journal, &budget).unwrap();
    assert!(required > small);
    assert_eq!(
        apply_state_journal_reserved(&parent.state, &parent.target, &journal, &budget, small),
        Err(StateError::BudgetExceeded)
    );
    assert_eq!(
        apply_state_journal_reserved(&parent.state, &parent.target, &journal, &budget, required)
            .unwrap(),
        parent.state
    );
}

#[test]
fn reserved_application_preserves_parent_after_invalid_journal_and_rejects_wrong_base() {
    let parent = support::with_slots();
    let before = parent.state.clone();
    let budget = development_state_budget();
    let mut journal = StateJournal {
        parent: parent.target.clone(),
        target_height: 2,
        operations: vec![
            JournalOperation::DeleteAccount {
                address: support::contract(),
            },
            JournalOperation::PutStorage {
                address: support::contract(),
                slot: U256::ZERO,
                value: U256::from(1),
            },
        ],
    };
    let required =
        estimate_journal_candidate_reservation(&parent.state, &journal, &budget).unwrap();
    assert_eq!(
        apply_state_journal_reserved(&parent.state, &parent.target, &journal, &budget, required),
        Err(StateError::MissingAccount(support::contract()))
    );
    assert_eq!(parent.state, before);
    journal.parent.content_digest.0[0] ^= 1;
    assert_eq!(
        apply_state_journal_reserved(&parent.state, &parent.target, &journal, &budget, 0),
        Err(StateError::ParentMismatch)
    );
    assert_eq!(parent.state, before);
}
