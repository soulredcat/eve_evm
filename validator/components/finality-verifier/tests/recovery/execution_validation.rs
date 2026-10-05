// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use crate::{
    native,
    recovery_support::{self as support, CLONE_BYTES},
};
use eve_consensus_comet::consensus::certificates::hash_transaction_data;
use eve_finality_verifier::{
    CompactRecoveryEnvelopeV1, NativeDataFrame, RecoveryError,
    authenticate_current_application_version, initialize_development_finality,
    initialize_development_recovery, prepare_development_recovery, recovery_state_commit,
    validate_recovery_envelope_bounds, verify_next_development_header,
};
use eve_state::{
    build_state_commit, development_state_budget, initialize_development_state,
    validate_retained_block, validate_state_commit,
};

#[test]
fn a_quorum_certified_structural_post_state_does_not_replace_nonce_validation() {
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
    let parent = initialize_development_recovery(&genesis, &budget).unwrap();
    let mut false_block = valid.commits[1].block.clone();
    false_block.header.parent_hash = initial.target.execution_hash.0;
    false_block.header.state_root = initial.target.evm_root.0;
    let mut extra = initial
        .target
        .identity
        .protocol_version
        .to_be_bytes()
        .to_vec();
    extra.extend_from_slice(&initial.target.identity.genesis.0.as_slice()[..28]);
    false_block.header.extra_data = extra.into();
    // The complete structural builder intentionally does not execute nonce transitions.
    let false_commit = build_state_commit(
        Some(initial.target.clone()),
        initial.state.clone(),
        false_block,
        &budget,
    )
    .unwrap();
    validate_state_commit(&false_commit, &budget).unwrap();
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
        expected: false_commit.target,
        execution: false_commit.block,
        finalized: support::native_frame(&first),
        lookahead: NativeDataFrame {
            frame: support::native_frame(&next),
            transactions: Vec::new(),
        },
    };
    validate_recovery_envelope_bounds(&record).unwrap();
    validate_retained_block(
        &record.expected,
        Some(&record.parent),
        &record.execution,
        &budget,
    )
    .unwrap();
    let mut finality = initialize_development_finality(&genesis, &budget).unwrap();
    let native_transactions = record
        .execution
        .transactions
        .iter()
        .map(|transaction| transaction.to_vec())
        .collect::<Vec<_>>();
    verify_next_development_header(
        &mut finality,
        &first.id,
        &first.header,
        &first.commit,
        &first.validators,
        &native_transactions,
    )
    .unwrap();
    verify_next_development_header(
        &mut finality,
        &next.id,
        &next.header,
        &next.commit,
        &next.validators,
        &[],
    )
    .unwrap();
    authenticate_current_application_version(&finality, &record.expected).unwrap();
    let error =
        prepare_development_recovery(&parent, Arc::new(record), &budget, CLONE_BYTES).unwrap_err();
    assert_eq!(
        error,
        RecoveryError::Execution(eve_evm::CompleteExecutionError::Execution(
            eve_evm::BlockExecutionError::Execution {
                index: 0,
                message: "Transaction(NonceTooLow { tx: 0, state: 1 })".into(),
            }
        )),
    );
    assert_eq!(recovery_state_commit(&parent), &initial);
}
