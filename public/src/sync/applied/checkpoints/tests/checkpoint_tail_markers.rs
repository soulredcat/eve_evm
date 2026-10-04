// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*,
    fixtures::{stage_artifacts, witnesses},
};
use crate::sync::applied::{
    applied_commit, applied_markers, capture_applied_state, finish_applied_state_service,
    open_segmented_applied_state_service, poll_applied_durability,
    segmented::tests::fixtures::{configuration, logical_chain},
    try_apply_recovery_bytes,
};
use std::{
    fs::File,
    time::{Duration, Instant},
};

#[test]
fn ordinary_tail_preserves_checkpoint_markers_in_ram_acknowledgment_and_old_arcs() {
    let directory = tempfile::tempdir().unwrap();
    let archive = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (mut owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("store"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let genesis = capture_applied_state(&reader).unwrap();
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[1],
        &witnesses(&chain, 1),
    )
    .unwrap();
    let prepared = prepare_applied_checkpoint(artifacts, &chain.genesis).unwrap();
    start_applied_checkpoint_activation(&mut owner, prepared).unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while poll_applied_checkpoint_activation(&mut owner)
        .unwrap()
        .is_none()
    {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
    let checkpoint = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_commit(&checkpoint), &chain.commits[1]);
    let reference = applied_markers(&checkpoint);
    assert_eq!(reference.checkpoint.0, 1);
    assert_eq!(reference.authenticated_snapshot_height, 1);
    try_apply_recovery_bytes(&mut owner, &chain.records[1]).unwrap();
    let applied = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_commit(&applied), &chain.commits[2]);
    assert_eq!(applied_markers(&applied).durable_recovery.0, 1);
    for markers in [applied_markers(&applied), reference] {
        assert_eq!(markers.checkpoint, reference.checkpoint);
        assert_eq!(
            markers.authenticated_snapshot_height,
            reference.authenticated_snapshot_height
        );
        assert_eq!(
            markers.oldest_retained_height,
            reference.oldest_retained_height
        );
    }
    let deadline = Instant::now() + Duration::from_secs(10);
    while poll_applied_durability(&mut owner)
        .unwrap()
        .durable_recovery
        .0
        != 2
    {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
    let durable = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_commit(&durable), &chain.commits[2]);
    let markers = applied_markers(&durable);
    assert_eq!(markers.checkpoint, reference.checkpoint);
    assert_eq!(
        markers.authenticated_snapshot_height,
        reference.authenticated_snapshot_height
    );
    assert_eq!(
        markers.oldest_retained_height,
        reference.oldest_retained_height
    );
    assert_eq!(applied_markers(&checkpoint), reference);
    assert_eq!(applied_markers(&genesis).checkpoint.0, 0);
    assert_eq!(applied_markers(&genesis).authenticated_snapshot_height, 0);
    assert_eq!(applied_commit(&genesis), &chain.commits[0]);
    drop(finish_applied_state_service(owner));
}
