// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::*;
use super::fixtures::{stage_artifacts, witnesses};
use crate::{
    persistence::segmented::install_segmented_record_pause,
    sync::applied::{
        AppliedError, applied_commit, applied_cursors, applied_markers, capture_applied_state,
        finish_applied_state_service, observe_estimated_working,
        open_segmented_applied_state_service, poll_applied_durability,
        segmented::tests::fixtures::{configuration, logical_chain},
        try_apply_recovery_bytes,
        types::AppliedBackend,
    },
};
use std::{
    fs::File,
    sync::Arc,
    time::{Duration, Instant},
};

#[test]
fn base_sync_does_not_hold_ram_lock_and_shutdown_reconciles_actual_checkpoint_ack() {
    let directory = tempfile::tempdir().unwrap();
    let archive = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (mut owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("store"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let old = capture_applied_state(&reader).unwrap();
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[2],
        &witnesses(&chain, 2),
    )
    .unwrap();
    let prepared = prepare_applied_checkpoint(artifacts, &chain.genesis).unwrap();
    let AppliedBackend::Segmented { worker, .. } = &owner.backend else {
        panic!("segmented")
    };
    let (entered, resume) = install_segmented_record_pause(worker.as_ref().unwrap(), 0, false);
    start_applied_checkpoint_activation(&mut owner, prepared).unwrap();
    entered.recv_timeout(Duration::from_secs(10)).unwrap();
    assert!(
        poll_applied_checkpoint_activation(&mut owner)
            .unwrap()
            .is_none()
    );
    assert!(matches!(
        try_apply_recovery_bytes(&mut owner, &chain.records[0]),
        Err(AppliedError::CheckpointPending)
    ));
    assert!(matches!(
        poll_applied_durability(&mut owner),
        Err(AppliedError::CheckpointPending)
    ));
    let copied_reader = reader.clone();
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let read = std::thread::spawn(move || {
        sender
            .send(capture_applied_state(&copied_reader))
            .ok()
            .unwrap()
    });
    let captured = receiver.recv_timeout(Duration::from_secs(1));
    resume.send(()).unwrap();
    read.join().unwrap();
    assert!(Arc::ptr_eq(&old, &captured.unwrap().unwrap()));
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.checkpoint_error.is_none());
    assert!(shutdown.acknowledgement_error.is_none());
    assert!(!retained_checkpoint_activation(
        &shutdown.unacknowledged_tail
    ));
    let current = shutdown.publication.as_ref().unwrap();
    assert_eq!(applied_commit(current), &chain.commits[2]);
    assert_eq!(applied_markers(current).durable_recovery.0, 2);
    assert_eq!(applied_markers(current).checkpoint.0, 2);
    assert_eq!(applied_markers(current).authenticated_snapshot_height, 2);
    assert_eq!(applied_cursors(current).0.sequence, 1);
    assert_eq!(applied_commit(&old), &chain.commits[0]);
    drop(shutdown.repository.unwrap());
}

#[test]
fn completed_candidate_bound_to_old_arc_rejects_after_another_durable_publication() {
    let directory = tempfile::tempdir().unwrap();
    let archive = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (mut owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("store"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[2],
        &witnesses(&chain, 2),
    )
    .unwrap();
    let prepared = prepare_applied_checkpoint(artifacts, &chain.genesis).unwrap();
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while poll_applied_durability(&mut owner)
        .unwrap()
        .durable_recovery
        .0
        != 1
    {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
    let before = capture_applied_state(&reader).unwrap();
    assert!(matches!(
        start_applied_checkpoint_activation(&mut owner, prepared),
        Err(CheckpointAppliedError::StaleParent)
    ));
    assert!(Arc::ptr_eq(
        &before,
        &capture_applied_state(&reader).unwrap()
    ));
    assert!(owner.checkpoint.is_none());
    drop(finish_applied_state_service(owner));
}

#[test]
fn panic_before_base_sync_preserves_old_ram_and_all_candidate_artifact_charges() {
    let directory = tempfile::tempdir().unwrap();
    let archive = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (mut owner, reader) = open_segmented_applied_state_service(
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
    let AppliedBackend::Segmented { worker, .. } = &owner.backend else {
        panic!("segmented")
    };
    let (entered, resume) = install_segmented_record_pause(worker.as_ref().unwrap(), 0, true);
    start_applied_checkpoint_activation(&mut owner, prepared).unwrap();
    entered.recv_timeout(Duration::from_secs(10)).unwrap();
    resume.send(()).unwrap();
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.repository.is_err());
    assert!(shutdown.checkpoint_error.is_some());
    assert!(retained_checkpoint_activation(
        &shutdown.unacknowledged_tail
    ));
    assert!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes
            > baseline
    );
    let current = shutdown.publication.as_ref().unwrap();
    assert_eq!(applied_commit(current), applied_commit(&old));
    assert_eq!(applied_markers(current).durable_recovery.0, 0);
    assert!(current.storage_failed);
    drop(shutdown);
    assert!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes
            <= baseline
    );
}
