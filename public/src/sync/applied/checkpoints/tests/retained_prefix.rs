// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*,
    fixtures::{advance_first_block, stage_artifacts, witnesses},
};
use crate::sync::applied::{
    applied_commit, capture_applied_state, finish_applied_state_service,
    open_segmented_applied_state_service,
    segmented::tests::fixtures::{configuration, logical_chain},
};
use std::{fs::File, sync::Arc};

#[test]
fn forged_retained_prefix_is_rejected_even_when_the_actual_parent_already_authenticates_it() {
    let directory = tempfile::tempdir().unwrap();
    let archive = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (mut owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("store"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    advance_first_block(&mut owner, &chain);
    let parent = capture_applied_state(&reader).unwrap();
    let mut proofs = witnesses(&chain, 2);
    let eve_finality_verifier::CheckpointWitness::Execution(first) = &mut proofs[0] else {
        panic!("execution")
    };
    first.native.frame.header.app_hash[0] ^= 1;
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[2],
        &proofs,
    )
    .unwrap();
    assert!(matches!(
        prepare_applied_checkpoint(artifacts, &chain.genesis),
        Err(CheckpointAppliedError::Checkpoint(_))
    ));
    assert!(Arc::ptr_eq(
        &parent,
        &capture_applied_state(&reader).unwrap()
    ));
    drop(finish_applied_state_service(owner));
}

#[test]
fn correct_retained_full_stream_and_exact_nonzero_parent_seed_activate_a_newer_height() {
    let directory = tempfile::tempdir().unwrap();
    let archive = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (mut owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("store"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    advance_first_block(&mut owner, &chain);
    let parent = capture_applied_state(&reader).unwrap();
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[2],
        &witnesses(&chain, 2),
    )
    .unwrap();
    let prepared = prepare_applied_checkpoint(artifacts, &chain.genesis).unwrap();
    start_applied_checkpoint_activation(&mut owner, prepared).unwrap();
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.checkpoint_error.is_none());
    assert_eq!(
        applied_commit(shutdown.publication.as_ref().unwrap()),
        &chain.commits[2]
    );
    assert_eq!(applied_commit(&parent), &chain.commits[1]);
    assert!(!retained_checkpoint_activation(
        &shutdown.unacknowledged_tail
    ));
    drop(shutdown.repository.unwrap());
}

#[test]
fn a_different_local_genesis_cannot_supply_checkpoint_authentication_policy() {
    let directory = tempfile::tempdir().unwrap();
    let archive = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("store"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let parent = capture_applied_state(&reader).unwrap();
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[2],
        &witnesses(&chain, 2),
    )
    .unwrap();
    let mut different = chain.genesis.clone();
    different.accounts[0].nonce += 1;
    assert!(matches!(
        prepare_applied_checkpoint(artifacts, &different),
        Err(CheckpointAppliedError::InvalidArtifactBinding)
    ));
    assert!(Arc::ptr_eq(
        &parent,
        &capture_applied_state(&reader).unwrap()
    ));
    drop(finish_applied_state_service(owner));
}
