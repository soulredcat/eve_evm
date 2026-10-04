// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;

#[test]
fn valid_foreign_chunk_width_outside_receiver_admission_is_preserved() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let mut foreign_limits = fixture.limits;
    foreign_limits.maximum_chunk_bytes = 1_024;
    let body = eve_state::preflight_state_commit(&fixture.body, &foreign_limits.logical).unwrap();
    let foreign = create_checkpoint_manifest(
        &body,
        &fixture.commit.target,
        &foreign_limits,
        required_checkpoint_metadata_reservation(&foreign_limits).unwrap(),
    )
    .unwrap();
    let path = namespace(&directory, &fixture).join("manifest.pending");
    std::fs::write(&path, &foreign).unwrap();
    assert!(preflight_checkpoint_manifest(&foreign, &fixture.version, &fixture.limits).is_err());
    assert!(matches!(
        repair_invalid_checkpoint_manifest_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::ValidEntry)
    ));
    assert_eq!(std::fs::read(path).unwrap(), foreign);
}

#[test]
fn valid_foreign_body_and_count_outside_receiver_limits_are_preserved() {
    let mut fixture = fixture();
    fixture.limits.maximum_chunks = fixture.body.len().div_ceil(512);
    let chain = eve_development_fixtures::recovery::recovery_chain();
    let commit = &chain.commits[2];
    let budget = eve_state::development_state_budget();
    let bytes = eve_state::encode_state_commit(commit, &budget).unwrap();
    assert!(bytes.len() > fixture.body.len());
    let foreign_limits = CheckpointLimits {
        logical: budget,
        maximum_body_bytes: bytes.len(),
        maximum_chunk_bytes: 128,
        maximum_chunks: 4_096,
        maximum_manifest_bytes: 262_144,
    };
    let foreign = create_checkpoint_manifest(
        &eve_state::preflight_state_commit(&bytes, &budget).unwrap(),
        &commit.target,
        &foreign_limits,
        required_checkpoint_metadata_reservation(&foreign_limits).unwrap(),
    )
    .unwrap();
    assert!(bytes.len().div_ceil(128) > fixture.limits.maximum_chunks);
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let path = namespace(&directory, &fixture).join("manifest.pending");
    std::fs::write(&path, &foreign).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_manifest_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::ValidEntry)
    ));
    assert_eq!(std::fs::read(path).unwrap(), foreign);
}

#[test]
fn actual_oversized_pending_file_is_preserved_before_bounded_owned_read() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let path = namespace(&directory, &fixture).join("manifest.pending");
    let file = std::fs::File::create(&path).unwrap();
    file.set_len(fixture.limits.maximum_manifest_bytes as u64 + 1)
        .unwrap();
    file.sync_all().unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_manifest_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::ValidEntry)
    ));
    assert_eq!(
        std::fs::metadata(&path).unwrap().len(),
        fixture.limits.maximum_manifest_bytes as u64 + 1
    );
}

#[test]
fn unrecognized_future_metadata_is_preserved_without_relaxing_current_admission() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let path = namespace(&directory, &fixture).join("manifest.pending");
    let mut future = fixture.manifest.clone();
    future[22] = b'2';
    std::fs::write(&path, &future).unwrap();
    assert!(preflight_checkpoint_manifest(&future, &fixture.version, &fixture.limits).is_err());
    assert!(matches!(
        repair_invalid_checkpoint_manifest_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::ValidEntry)
    ));
    assert_eq!(std::fs::read(path).unwrap(), future);
}
