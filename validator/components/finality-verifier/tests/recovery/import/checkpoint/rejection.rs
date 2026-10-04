// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{execution, limits, lookahead, session};
use crate::recovery_support::recovery_chain;
use eve_finality_verifier::{
    CheckpointError, CheckpointWitness, begin_authenticated_checkpoint,
    finish_authenticated_checkpoint, initialize_authenticated_import,
    required_checkpoint_reservation, verify_checkpoint_witness,
};
use eve_state::{
    B256, ExecutionBlockHash, SystemValue, U256, build_state_commit, development_state_budget,
};
use std::sync::Arc;

#[test]
fn full_target_root_mutation_and_changed_actual_parent_history_refuse_checkpoint_creation() {
    let chain = recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let required =
        required_checkpoint_reservation(&parent, &chain.commits[2], &budget, limits()).unwrap();
    let mut wrong_root = chain.commits[2].clone();
    wrong_root.target.evm_root.0 = B256::repeat_byte(0x71);
    assert!(
        begin_authenticated_checkpoint(&parent, Arc::new(wrong_root), &budget, limits(), required)
            .is_err()
    );
    let mut state = chain.commits[2].state.clone();
    state
        .block_hashes
        .insert(0, ExecutionBlockHash(B256::repeat_byte(0x72)));
    let target = build_state_commit(
        chain.commits[2].parent.clone(),
        state,
        chain.commits[2].block.clone(),
        &budget,
    )
    .unwrap();
    let required = required_checkpoint_reservation(&parent, &target, &budget, limits()).unwrap();
    assert!(matches!(
        begin_authenticated_checkpoint(&parent, Arc::new(target), &budget, limits(), required),
        Err(CheckpointError::WrongHistory)
    ));
}

#[test]
fn skipping_or_duplicating_witnesses_and_missing_h_plus_one_cannot_finish_checkpoint() {
    let chain = recovery_chain();
    let parent =
        initialize_authenticated_import(&chain.genesis, &development_state_budget()).unwrap();
    let (mut progress, reserved) = session(&parent, &chain);
    assert!(matches!(
        verify_checkpoint_witness(&mut progress, &execution(&chain, 2), reserved),
        Err(CheckpointError::WrongHeight)
    ));
    verify_checkpoint_witness(&mut progress, &execution(&chain, 1), reserved).unwrap();
    assert!(verify_checkpoint_witness(&mut progress, &execution(&chain, 1), reserved).is_err());
    verify_checkpoint_witness(&mut progress, &execution(&chain, 2), reserved).unwrap();
    assert!(matches!(
        finish_authenticated_checkpoint(progress),
        Err(CheckpointError::Incomplete)
    ));
}

#[test]
fn wrong_certificate_and_native_data_refuse_without_advancing_the_stream() {
    let chain = recovery_chain();
    let parent =
        initialize_authenticated_import(&chain.genesis, &development_state_budget()).unwrap();
    let (mut progress, reserved) = session(&parent, &chain);
    let mut invalid = execution(&chain, 1);
    let CheckpointWitness::Execution(ref mut witness) = invalid else {
        unreachable!()
    };
    witness.native.frame.commit.signatures[0].signature[0] ^= 1;
    assert!(verify_checkpoint_witness(&mut progress, &invalid, reserved).is_err());
    let mut invalid = execution(&chain, 1);
    let CheckpointWitness::Execution(ref mut witness) = invalid else {
        unreachable!()
    };
    witness.native.transactions.clear();
    assert!(verify_checkpoint_witness(&mut progress, &invalid, reserved).is_err());
    verify_checkpoint_witness(&mut progress, &execution(&chain, 1), reserved).unwrap();
    verify_checkpoint_witness(&mut progress, &execution(&chain, 2), reserved).unwrap();
    verify_checkpoint_witness(&mut progress, &lookahead(&chain), reserved).unwrap();
    assert!(finish_authenticated_checkpoint(progress).is_ok());
}

#[test]
fn canonical_but_uncertified_system_root_cannot_close_at_the_actual_next_header() {
    let chain = recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let mut state = chain.commits[2].state.clone();
    for record in state.system.values_mut() {
        if let SystemValue::Fee { burned, .. } = &mut record.value {
            *burned += U256::from(1);
        }
    }
    let target = build_state_commit(
        chain.commits[2].parent.clone(),
        state,
        chain.commits[2].block.clone(),
        &budget,
    )
    .unwrap();
    let required = required_checkpoint_reservation(&parent, &target, &budget, limits()).unwrap();
    let mut changed = execution(&chain, 2);
    let CheckpointWitness::Execution(ref mut witness) = changed else {
        unreachable!()
    };
    witness.version = target.target.clone();
    let mut progress =
        begin_authenticated_checkpoint(&parent, Arc::new(target), &budget, limits(), required)
            .unwrap();
    verify_checkpoint_witness(&mut progress, &execution(&chain, 1), required).unwrap();
    verify_checkpoint_witness(&mut progress, &changed, required).unwrap();
    assert!(verify_checkpoint_witness(&mut progress, &lookahead(&chain), required).is_err());
    assert!(matches!(
        finish_authenticated_checkpoint(progress),
        Err(CheckpointError::Incomplete)
    ));
}
