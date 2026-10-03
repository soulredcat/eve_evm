// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_finality_verifier::{
    imported_state_anchor, imported_state_commit, imported_transition_input,
    imported_transition_state, initialize_authenticated_import, into_imported_state,
    prepare_authenticated_import,
};
use eve_state::{Address, U256, development_state_budget};

use super::support::input;
use crate::recovery_support::{self as support, CLONE_BYTES};

#[test]
fn real_nonempty_import_matches_canonical_contract_receipts_fees_and_h_h_plus_one_anchor() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    assert!(imported_state_anchor(&parent).is_none());
    let mut delta = Arc::new(input(&chain, 1));
    let transition =
        prepare_authenticated_import(&parent, Arc::clone(&delta), &budget, CLONE_BYTES).unwrap();
    assert!(Arc::ptr_eq(&delta, imported_transition_input(&transition)));
    assert!(Arc::get_mut(&mut delta).is_none());
    let candidate = imported_transition_state(&transition);
    let commit = imported_state_commit(candidate);
    // This honest fixture has identical auxiliary representation. Certificates
    // alone do not establish exact content_digest equivalence for every import.
    assert_eq!(commit, &chain.commits[1]);
    assert_eq!(commit.state.system, chain.commits[1].state.system);
    assert_eq!(commit.state.accounts[&support::sender()].nonce, 1);
    assert_eq!(
        commit.state.accounts[&Address::with_last_byte(0x42)].storage[&U256::ZERO],
        U256::from(99),
    );
    assert_eq!(commit.block.header.gas_used, 43_106);
    let anchor = imported_state_anchor(candidate).unwrap();
    assert_eq!(anchor.execution_height(), 1);
    assert_eq!(anchor.consensus_height(), 2);
    assert_eq!(anchor.consensus_block_id(), &chain.frames[1].id);
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
    drop(transition);
    assert!(Arc::get_mut(&mut delta).is_some());
}

#[test]
fn consecutive_imports_reuse_certified_lookahead_and_duplicate_delivery_changes_nothing() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let genesis = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let first = into_imported_state(
        prepare_authenticated_import(&genesis, Arc::new(input(&chain, 1)), &budget, CLONE_BYTES)
            .unwrap(),
    );
    let second = into_imported_state(
        prepare_authenticated_import(&first, Arc::new(input(&chain, 2)), &budget, CLONE_BYTES)
            .unwrap(),
    );
    assert_eq!(imported_state_commit(&first), &chain.commits[1]);
    assert_eq!(imported_state_commit(&second), &chain.commits[2]);
    assert_eq!(
        imported_state_anchor(&second).unwrap().consensus_height(),
        3
    );
    assert!(
        prepare_authenticated_import(&second, Arc::new(input(&chain, 2)), &budget, CLONE_BYTES)
            .is_err()
    );
    assert_eq!(imported_state_commit(&second), &chain.commits[2]);
}
