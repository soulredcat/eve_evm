// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::fixtures::{fields, fixture, list, replace_state_map};
use eve_state::{StateError, preflight_state_commit};

#[test]
fn duplicate_and_reordered_account_system_history_and_code_keys_refuse_without_materializing_maps()
{
    let (_, bytes, budget) = fixture();
    let commit = fields(&bytes);
    let state = fields(&commit[3]);
    for field in [1, 2, 3, 4] {
        let original = fields(&state[field]);
        assert!(
            original.len() > 1,
            "fixture must exercise every map's ordering"
        );
        let mut duplicate = original.clone();
        duplicate.insert(0, original[0].clone());
        assert!(matches!(
            preflight_state_commit(&replace_state_map(&bytes, field, duplicate), &budget),
            Err(StateError::NonCanonicalEncoding)
        ));
        let mut reordered = original;
        reordered.swap(0, 1);
        assert!(matches!(
            preflight_state_commit(&replace_state_map(&bytes, field, reordered), &budget),
            Err(StateError::NonCanonicalEncoding)
        ));
    }
}

#[test]
fn duplicate_or_reordered_storage_keys_refuse_before_inserting_any_owned_account() {
    let (_, bytes, budget) = fixture();
    let mut commit = fields(&bytes);
    let mut state = fields(&commit[3]);
    let accounts = fields(&state[1]);
    let index = accounts
        .iter()
        .position(|account| fields(&fields(account)[4]).len() == 2)
        .unwrap();
    for duplicate in [true, false] {
        let mut altered_accounts = accounts.clone();
        let mut account = fields(&altered_accounts[index]);
        let mut slots = fields(&account[4]);
        if duplicate {
            slots.insert(0, slots[0].clone());
        } else {
            slots.swap(0, 1);
        }
        account[4] = list(&slots);
        altered_accounts[index] = list(&account);
        state[1] = list(&altered_accounts);
        commit[3] = list(&state);
        assert!(matches!(
            preflight_state_commit(&list(&commit), &budget),
            Err(StateError::NonCanonicalEncoding)
        ));
    }
}
