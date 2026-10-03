// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use crate::recovery_support::{self as support, CLONE_BYTES};
use eve_finality_verifier::{
    initialize_development_recovery, into_recovery_state, prepare_development_recovery,
    recovery_state_anchor, recovery_state_commit, recovery_transition_envelope,
    recovery_transition_state,
};
use eve_state::{Address, U256, development_state_budget};

#[test]
fn canonical_signed_replay_anchors_exact_roots_receipts_and_contract_effects() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    assert!(recovery_state_anchor(&parent).is_none());
    let record = Arc::new(support::envelope(&chain, 1));
    let transition = prepare_development_recovery(
        &parent,
        Arc::clone(&record),
        &development_state_budget(),
        CLONE_BYTES,
    )
    .unwrap();
    assert!(Arc::ptr_eq(
        &record,
        recovery_transition_envelope(&transition)
    ));
    let applied = recovery_transition_state(&transition);
    assert_eq!(recovery_state_commit(applied), &chain.commits[1]);
    assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
    let anchor = recovery_state_anchor(applied).unwrap();
    assert_eq!(anchor.execution_height(), 1);
    assert_eq!(anchor.consensus_height(), 2);
    assert_eq!(anchor.consensus_block_id(), &chain.frames[1].id);
    let accounts = &recovery_state_commit(applied).state.accounts;
    assert_eq!(accounts[&support::sender()].nonce, 1);
    assert_eq!(
        accounts[&Address::with_last_byte(0x42)].storage[&U256::ZERO],
        U256::from(99),
    );
    assert_eq!(recovery_state_commit(applied).block.header.gas_used, 43_106);
}

#[test]
fn two_consecutive_records_reuse_authenticated_lookahead_without_reverifying_its_height() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    let first = prepare_development_recovery(
        &parent,
        Arc::new(support::envelope(&chain, 1)),
        &development_state_budget(),
        CLONE_BYTES,
    )
    .unwrap();
    let first = into_recovery_state(first);
    let second = prepare_development_recovery(
        &first,
        Arc::new(support::envelope(&chain, 2)),
        &development_state_budget(),
        CLONE_BYTES,
    )
    .unwrap();
    let second = into_recovery_state(second);
    assert_eq!(recovery_state_commit(&first), &chain.commits[1]);
    assert_eq!(recovery_state_commit(&second), &chain.commits[2]);
    assert_eq!(
        recovery_state_anchor(&second).unwrap().consensus_height(),
        3
    );
    assert!(
        prepare_development_recovery(
            &second,
            Arc::new(support::envelope(&chain, 2)),
            &development_state_budget(),
            CLONE_BYTES,
        )
        .is_err()
    );
    assert_eq!(recovery_state_commit(&second), &chain.commits[2]);
}

#[test]
fn parent_and_caller_envelope_ownership_are_retained_during_private_preparation() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    let mut record = Arc::new(support::envelope(&chain, 1));
    let transition = prepare_development_recovery(
        &parent,
        Arc::clone(&record),
        &development_state_budget(),
        CLONE_BYTES,
    )
    .unwrap();
    assert!(Arc::get_mut(&mut record).is_none());
    assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
    drop(transition);
    assert!(Arc::get_mut(&mut record).is_some());
}
