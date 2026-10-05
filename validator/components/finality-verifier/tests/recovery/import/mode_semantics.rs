// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_consensus_comet::consensus::certificates::hash_transaction_data;
use eve_finality_verifier::{
    AuthenticatedImportInput, CompactRecoveryEnvelopeV1, NativeDataFrame, RecoveryError,
    imported_state_commit, imported_transition_state, initialize_authenticated_import,
    initialize_development_recovery, prepare_authenticated_import, prepare_development_recovery,
};
use eve_state::{
    build_state_commit, development_state_budget, initialize_development_state,
    project_state_journal,
};

use crate::{
    native,
    recovery_support::{self as support, CLONE_BYTES},
};

#[test]
fn certified_import_is_not_independent_nonce_execution_and_original_replay_still_rejects() {
    let valid = support::recovery_chain();
    let mut genesis = valid.genesis.clone();
    genesis
        .accounts
        .iter_mut()
        .find(|account| account.address == support::sender())
        .unwrap()
        .nonce = 1;
    let budget = development_state_budget();
    let initial = initialize_development_state(&genesis, &budget).unwrap();
    let imported_parent = initialize_authenticated_import(&genesis, &budget).unwrap();
    let replay_parent = initialize_development_recovery(&genesis, &budget).unwrap();
    let mut block = valid.commits[1].block.clone();
    block.header.parent_hash = initial.target.execution_hash.0;
    block.header.state_root = initial.target.evm_root.0;
    let mut extra = initial
        .target
        .identity
        .protocol_version
        .to_be_bytes()
        .to_vec();
    extra.extend_from_slice(&initial.target.identity.genesis.0.as_slice()[..28]);
    block.header.extra_data = extra.into();
    let false_commit = build_state_commit(
        Some(initial.target.clone()),
        initial.state.clone(),
        block,
        &budget,
    )
    .unwrap();
    let mut first = native::frame(&genesis, 1, None, initial.target.content_digest.0);
    first.header.data_hash = hash_transaction_data(&false_commit.block.transactions)
        .unwrap()
        .to_vec();
    native::resign(&mut first);
    let next = native::frame(
        &genesis,
        2,
        Some(first.id.clone()),
        false_commit.target.application.unwrap().0.0,
    );
    let record = CompactRecoveryEnvelopeV1 {
        parent: initial.target.clone(),
        expected: false_commit.target.clone(),
        execution: false_commit.block.clone(),
        finalized: support::native_frame(&first),
        lookahead: NativeDataFrame {
            frame: support::native_frame(&next),
            transactions: Vec::new(),
        },
    };
    let delta = AuthenticatedImportInput {
        journal: project_state_journal(
            &initial.state,
            &initial.target,
            &false_commit.state,
            1,
            &budget,
        )
        .unwrap(),
        execution: record.execution.clone(),
        finalized: record.finalized.clone(),
        lookahead: record.lookahead.clone(),
    };
    // Unsafe fixture keys deliberately certify an invalid execution outcome.
    // This is outside honest-validator assumptions, not a new safety guarantee.
    let imported =
        prepare_authenticated_import(&imported_parent, Arc::new(delta), &budget, CLONE_BYTES)
            .unwrap();
    assert_eq!(
        imported_state_commit(imported_transition_state(&imported)),
        &false_commit
    );
    assert_eq!(imported_state_commit(&imported_parent), &initial);
    assert_eq!(
        prepare_development_recovery(&replay_parent, Arc::new(record), &budget, CLONE_BYTES)
            .unwrap_err(),
        RecoveryError::Execution(eve_evm::CompleteExecutionError::Execution(
            eve_evm::BlockExecutionError::Execution {
                index: 0,
                message: "Transaction(NonceTooLow { tx: 0, state: 1 })".into(),
            },
        )),
    );
}
