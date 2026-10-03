// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_finality_verifier::{
    ImportError, imported_state_commit, initialize_authenticated_import,
    prepare_authenticated_import,
};
use eve_state::{development_state_budget, estimate_journal_candidate_reservation};

use super::support::input;
use crate::recovery_support::{self as support, CLONE_BYTES};

#[test]
fn insufficient_candidate_reservation_is_explicit_and_parent_remains_unchanged() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let delta = input(&chain, 1);
    let required =
        estimate_journal_candidate_reservation(&chain.commits[0].state, &delta.journal, &budget)
            .unwrap();
    assert!(required > 0);
    assert_eq!(
        prepare_authenticated_import(&parent, Arc::new(delta), &budget, required - 1).unwrap_err(),
        ImportError::CandidateReservation {
            required,
            reserved: required - 1
        },
    );
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn typed_import_component_limits_refuse_excessive_operations_or_native_payload() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let restricted = eve_state::StateBudget {
        maximum_journal_operations: 1,
        ..budget
    };
    assert!(
        prepare_authenticated_import(
            &parent,
            Arc::new(input(&chain, 1)),
            &restricted,
            CLONE_BYTES
        )
        .is_err()
    );
    let mut delta = input(&chain, 1);
    delta
        .lookahead
        .transactions
        .push(vec![0_u8; 131_073].into());
    assert!(prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES).is_err());
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}
