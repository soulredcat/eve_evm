// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;
use sha2::{Digest, Sha256};

#[test]
fn manifest_has_exact_bounded_uncompressed_layout_and_identity() {
    let fixture = fixture();
    let preflight = manifest(&fixture);
    let stats = checkpoint_manifest_stats(&preflight);
    assert_eq!(stats.body_bytes, fixture.body.len());
    assert_eq!(stats.chunk_bytes, 512);
    assert_eq!(stats.chunks, fixture.body.len().div_ceil(512));
    assert_eq!(
        stats.manifest_bytes,
        76 + fixture.version.len() + 48 * stats.chunks
    );
    assert_eq!(stats.target_version_bytes, fixture.version.len());
    assert_eq!(
        stats.body_sha256,
        <[u8; 32]>::from(Sha256::digest(&fixture.body))
    );
    assert_eq!(
        checkpoint_manifest_id(&preflight),
        <[u8; 32]>::from(Sha256::digest(&fixture.manifest))
    );
}

#[test]
fn wrong_target_truncated_extra_compressed_or_reordered_manifest_is_rejected() {
    let fixture = fixture();
    let mut target = fixture.version.to_vec();
    target[0] ^= 1;
    assert!(matches!(
        preflight_checkpoint_manifest(&fixture.manifest, &target, &fixture.limits),
        Err(CheckpointError::TargetMismatch)
    ));
    for position in [0, 24, 25, 26, 28, 36, 40, 76 + fixture.version.len()] {
        let mut bytes = fixture.manifest.clone();
        bytes[position] ^= 0x80;
        assert!(
            preflight_checkpoint_manifest(&bytes, &fixture.version, &fixture.limits).is_err(),
            "position {position}"
        );
    }
    let mut extra = fixture.manifest.clone();
    extra.push(0);
    assert!(preflight_checkpoint_manifest(&extra, &fixture.version, &fixture.limits).is_err());
    for length in [0, 75, fixture.manifest.len() - 1] {
        assert!(
            preflight_checkpoint_manifest(
                &fixture.manifest[..length],
                &fixture.version,
                &fixture.limits
            )
            .is_err()
        );
    }
    let mut offset = fixture.manifest.clone();
    offset[76 + fixture.version.len() + 11] ^= 1;
    assert!(preflight_checkpoint_manifest(&offset, &fixture.version, &fixture.limits).is_err());
}

#[test]
fn aggregate_hash_failure_cannot_publish_completion_even_with_valid_chunk_hashes() {
    let mut fixture = fixture();
    fixture.manifest[44] ^= 1;
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    assert!(matches!(
        complete_checkpoint_transfer(transfer, fixture.io),
        Err(CheckpointError::CorruptChunk)
    ));
    assert!(
        !namespace(&directory, &fixture)
            .join("complete.bin")
            .exists()
    );
    assert!(chunk_path(&directory, &fixture, 0).is_file());
}

#[test]
fn manifest_creation_rechecks_target_and_frozen_stricter_logical_budget() {
    let fixture = fixture();
    let preflight =
        eve_state::preflight_state_commit(&fixture.body, &fixture.limits.logical).unwrap();
    let mut target = fixture.commit.target.clone();
    target.timestamp += 1;
    assert!(matches!(
        create_checkpoint_manifest(&preflight, &target, &fixture.limits, fixture.metadata),
        Err(CheckpointError::TargetMismatch)
    ));
    let mut limits = fixture.limits;
    limits.logical.maximum_accounts = 1;
    assert!(matches!(
        create_checkpoint_manifest(
            &preflight,
            &fixture.commit.target,
            &limits,
            fixture.metadata
        ),
        Err(CheckpointError::State(_))
    ));
}

#[test]
fn matching_transport_checksums_cannot_override_canonical_body_target() {
    let mut fixture = fixture();
    let mut target = fixture.commit.target.clone();
    target.timestamp += 1;
    let encoded = eve_state::encode_state_version(&target).unwrap();
    assert_eq!(encoded.len(), fixture.version.len());
    fixture.manifest[76..76 + encoded.len()].copy_from_slice(&encoded);
    fixture.version = encoded;
    let (_directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    // Local file integrity is deliberately distinct from canonical state authorization.
    let completed = complete_checkpoint_transfer(transfer, fixture.io).unwrap();
    assert!(matches!(
        read_checkpoint_body(
            &completed,
            required_checkpoint_body_reservation(&completed).unwrap()
        ),
        Err(CheckpointError::TargetMismatch)
    ));
}
