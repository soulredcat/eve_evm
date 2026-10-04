// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*,
    fixtures::limits,
    reopen_fixtures::{private_archive, recovery_config},
};
use crate::sync::applied::{
    applied_commit, applied_markers, applied_owner_reader, capture_applied_state,
    finish_applied_state_service, open_segmented_applied_state_service, reserve_applied_working,
    segmented::tests::fixtures::{configuration, logical_chain},
};
use eve_state::{encode_state_commit, encode_state_version, preflight_state_commit};
use eve_storage::checkpoints::{
    create_checkpoint_manifest, required_checkpoint_metadata_reservation,
};
use std::fs::File;

#[test]
fn incomplete_staging_without_a_durable_base_never_changes_genesis_or_checkpoint_markers() {
    let directory = tempfile::tempdir().unwrap();
    let archive = private_archive();
    let chain = logical_chain(false);
    let (owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("store"), &chain),
        &chain.genesis,
    )
    .unwrap();
    let _ingress = reserve_applied_working(&applied_owner_reader(&owner), 8 * 1_048_576).unwrap();
    let limits = limits();
    let body = encode_state_commit(&chain.commits[1], &limits.content.logical).unwrap();
    assert!(body.len() > limits.content.maximum_chunk_bytes);
    let sealed = preflight_state_commit(&body, &limits.content.logical).unwrap();
    let manifest = create_checkpoint_manifest(
        &sealed,
        &chain.commits[1].target,
        &limits.content,
        required_checkpoint_metadata_reservation(&limits.content).unwrap(),
    )
    .unwrap();
    let target = encode_state_version(&chain.commits[1].target).unwrap();
    let mut transfer = begin_applied_checkpoint_transfer(
        &owner,
        &File::open(archive.path()).unwrap(),
        &manifest,
        &target,
        limits,
    )
    .unwrap();
    let id = transfer.content_id;
    write_applied_checkpoint_chunk(
        &mut transfer,
        0,
        &body[..limits.content.maximum_chunk_bytes],
    )
    .unwrap();
    drop(transfer);
    assert_eq!(
        applied_commit(&capture_applied_state(&reader).unwrap()),
        &chain.commits[0]
    );
    drop(finish_applied_state_service(owner));
    let (restored_owner, restored_reader) = open_segmented_applied_state_service_with_checkpoints(
        configuration(&directory.path().join("store"), &chain),
        recovery_config(archive.path()),
        &chain.genesis,
    )
    .unwrap();
    let restored = capture_applied_state(&restored_reader).unwrap();
    assert_eq!(applied_commit(&restored), &chain.commits[0]);
    assert_eq!(applied_markers(&restored).checkpoint.0, 0);
    assert_eq!(applied_markers(&restored).authenticated_snapshot_height, 0);
    let namespace = archive.path().join(hex::encode(id));
    assert!(namespace.join("chunk-0000.bin").is_file());
    assert!(!namespace.join("complete.bin").exists());
    drop(finish_applied_state_service(restored_owner));
}
