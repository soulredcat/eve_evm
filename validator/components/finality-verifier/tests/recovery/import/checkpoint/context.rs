// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{execution, limits, lookahead, session};
use crate::recovery_support::{CLONE_BYTES, recovery_chain};
use eve_finality_verifier::{
    CheckpointError, CheckpointWitness, begin_authenticated_checkpoint,
    initialize_authenticated_import, into_imported_state, prepare_authenticated_import,
    required_checkpoint_reservation, verify_checkpoint_witness,
};
use eve_state::{
    Address, B256, SecurityProfile, build_state_commit, build_state_version,
    development_state_budget,
};
use std::sync::Arc;

#[test]
fn timestamp_proposer_mix_hash_and_base_fee_are_checked_against_certified_native_context() {
    let chain = recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    for field in 0..4 {
        let (mut progress, reserved) = session(&parent, &chain);
        let mut block = chain.commits[1].block.clone();
        match field {
            0 => block.header.timestamp += 1,
            1 => block.header.beneficiary = Address::repeat_byte(0x73),
            2 => block.header.mix_hash = B256::repeat_byte(0x74),
            _ => block.header.base_fee_per_gas = Some(1),
        }
        let version = build_state_commit(
            chain.commits[1].parent.clone(),
            chain.commits[1].state.clone(),
            block.clone(),
            &budget,
        )
        .unwrap()
        .target;
        let mut witness = execution(&chain, 1);
        let CheckpointWitness::Execution(ref mut execution) = witness else {
            unreachable!()
        };
        execution.version = version;
        execution.block = block;
        assert!(matches!(
            verify_checkpoint_witness(&mut progress, &witness, reserved),
            Err(CheckpointError::Import(_))
        ));
        verify_checkpoint_witness(
            &mut progress,
            &super::support::execution(&chain, 1),
            reserved,
        )
        .unwrap();
    }
}

#[test]
fn changed_new_history_hash_cannot_be_rebound_to_a_peer_selected_parent_header() {
    let chain = recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let mut state = chain.commits[2].state.clone();
    let mut altered_parent_header = chain.commits[1].block.header.clone();
    altered_parent_header.mix_hash = B256::repeat_byte(0x75);
    let parent_version =
        build_state_version(&chain.commits[1].state, &altered_parent_header, &budget).unwrap();
    let changed = parent_version.execution_hash;
    state.block_hashes.insert(1, changed);
    let mut block = chain.commits[2].block.clone();
    block.header.parent_hash = changed.0;
    let target = build_state_commit(Some(parent_version), state, block, &budget).unwrap();
    let required = required_checkpoint_reservation(&parent, &target, &budget, limits()).unwrap();
    let mut progress =
        begin_authenticated_checkpoint(&parent, Arc::new(target), &budget, limits(), required)
            .unwrap();
    assert!(matches!(
        verify_checkpoint_witness(&mut progress, &execution(&chain, 1), required),
        Err(CheckpointError::WrongHistory)
    ));
}

#[test]
fn changed_profile_and_nonlocal_lookahead_cannot_replace_actual_parent_trust() {
    let chain = recovery_chain();
    let budget = development_state_budget();
    let genesis = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let parent = into_imported_state(
        prepare_authenticated_import(
            &genesis,
            Arc::new(super::super::support::input(&chain, 1)),
            &budget,
            CLONE_BYTES,
        )
        .unwrap(),
    );
    let (mut progress, reserved) = session(&parent, &chain);
    let mut changed_seed = execution(&chain, 2);
    let CheckpointWitness::Execution(ref mut execution) = changed_seed else {
        unreachable!()
    };
    execution.native.frame.commit.signatures[0].signature[0] ^= 1;
    assert!(matches!(
        verify_checkpoint_witness(&mut progress, &changed_seed, reserved),
        Err(CheckpointError::WrongNativeData)
    ));
    verify_checkpoint_witness(
        &mut progress,
        &super::support::execution(&chain, 2),
        reserved,
    )
    .unwrap();
    verify_checkpoint_witness(&mut progress, &lookahead(&chain), reserved).unwrap();
    let mut wrong_profile = chain.commits[2].clone();
    wrong_profile.target.identity.security_profile = SecurityProfile::HybridExperimental;
    wrong_profile.state.identity.security_profile = SecurityProfile::HybridExperimental;
    assert!(
        begin_authenticated_checkpoint(
            &parent,
            Arc::new(wrong_profile),
            &budget,
            limits(),
            reserved
        )
        .is_err()
    );
}

#[test]
fn checkpoint_parent_auxiliary_version_cannot_replace_the_exact_actual_imported_parent() {
    let chain = recovery_chain();
    let budget = development_state_budget();
    let genesis = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let parent = into_imported_state(
        prepare_authenticated_import(
            &genesis,
            Arc::new(super::super::support::input(&chain, 1)),
            &budget,
            CLONE_BYTES,
        )
        .unwrap(),
    );
    let mut target = chain.commits[2].clone();
    target.parent.as_mut().unwrap().content_digest.0[0] ^= 1;
    let required = required_checkpoint_reservation(&parent, &target, &budget, limits()).unwrap();
    let mut progress =
        begin_authenticated_checkpoint(&parent, Arc::new(target), &budget, limits(), required)
            .unwrap();
    assert!(matches!(
        verify_checkpoint_witness(&mut progress, &execution(&chain, 2), required),
        Err(CheckpointError::WrongTarget)
    ));
}
