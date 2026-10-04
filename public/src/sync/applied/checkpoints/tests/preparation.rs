// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{CheckpointAppliedError, prepare_applied_checkpoint};
use super::fixtures::{stage_artifacts, witnesses};
use crate::sync::applied::{
    applied_commit, capture_applied_state, finish_applied_state_service, observe_estimated_working,
    open_segmented_applied_state_service,
    segmented::tests::fixtures::{configuration, logical_chain},
    state::applied_state_commit,
};
use std::{fs::File, sync::Arc};

#[test]
fn nonempty_checkpoint_preparation_retains_real_generation_charge_and_old_view() {
    let directory = tempfile::tempdir().unwrap();
    let archive = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("store"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let old = capture_applied_state(&reader).unwrap();
    let baseline = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes;
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[2],
        &witnesses(&chain, 2),
    )
    .unwrap();
    let prepared = prepare_applied_checkpoint(artifacts, &chain.genesis).unwrap();
    assert_eq!(
        applied_state_commit(&prepared.generation.state),
        &chain.commits[2]
    );
    assert_eq!(
        applied_commit(&capture_applied_state(&reader).unwrap()),
        &chain.commits[0]
    );
    assert!(Arc::ptr_eq(&old, &prepared.artifacts.content.parent));
    assert!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes
            > baseline
    );
    drop(prepared);
    assert_eq!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes,
        baseline
    );
    drop(finish_applied_state_service(owner));
}

#[test]
fn intact_checksum_manifest_with_forged_native_proof_never_prepares_or_publishes() {
    let directory = tempfile::tempdir().unwrap();
    let archive = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("store"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let old = capture_applied_state(&reader).unwrap();
    let baseline = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes;
    let mut proof = witnesses(&chain, 2);
    let eve_finality_verifier::CheckpointWitness::Lookahead(closing) = proof.last_mut().unwrap()
    else {
        panic!("lookahead")
    };
    closing.frame.header.app_hash[0] ^= 1;
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[2],
        &proof,
    )
    .unwrap();
    assert!(matches!(
        prepare_applied_checkpoint(artifacts, &chain.genesis),
        Err(CheckpointAppliedError::Checkpoint(_))
    ));
    assert!(Arc::ptr_eq(&old, &capture_applied_state(&reader).unwrap()));
    assert_eq!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes,
        baseline
    );
    drop(finish_applied_state_service(owner));
}
