// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::{CheckpointError, proofs::*};

#[test]
fn wrong_snapshot_body_target_height_compression_or_reference_range_is_rejected() {
    let fixture = fixture();
    for position in [
        0,
        24,
        25,
        28,
        36,
        40,
        48,
        80,
        144 + fixture.version.len(),
        144 + fixture.version.len() + 8,
        144 + fixture.version.len() + 9,
        144 + fixture.version.len() + 23,
    ] {
        let mut bytes = fixture.manifest.clone();
        bytes[position] ^= 0x80;
        assert!(
            preflight_checkpoint_proof_manifest(
                &bytes,
                fixture.target.height,
                &fixture.version,
                &fixture.snapshot,
                &fixture.body,
                &fixture.limits
            )
            .is_err()
        );
    }
    assert!(matches!(
        preflight_checkpoint_proof_manifest(
            &fixture.manifest,
            fixture.target.height + 1,
            &fixture.version,
            &fixture.snapshot,
            &fixture.body,
            &fixture.limits
        ),
        Err(CheckpointError::TargetMismatch)
    ));
    let mut truncated = fixture.manifest.clone();
    truncated.pop();
    assert!(
        preflight_checkpoint_proof_manifest(
            &truncated,
            fixture.target.height,
            &fixture.version,
            &fixture.snapshot,
            &fixture.body,
            &fixture.limits
        )
        .is_err()
    );
}

#[test]
fn wrong_export_reference_range_and_missing_closing_witness_are_rejected() {
    let fixture = fixture();
    let hash = checkpoint_proof_manifest_stats(&manifest(&fixture)).stream_sha256;
    let mut references = fixture.references.clone();
    references[0].height = 2;
    assert!(
        create_checkpoint_proof_manifest(
            &fixture.snapshot,
            &fixture.body,
            &fixture.target,
            &references,
            &hash,
            &fixture.limits,
            fixture.metadata
        )
        .is_err()
    );
    references = fixture.references.clone();
    references[2].kind = CheckpointProofKind::Execution;
    assert!(
        create_checkpoint_proof_manifest(
            &fixture.snapshot,
            &fixture.body,
            &fixture.target,
            &references,
            &hash,
            &fixture.limits,
            fixture.metadata
        )
        .is_err()
    );
    assert!(
        create_checkpoint_proof_manifest(
            &fixture.snapshot,
            &fixture.body,
            &fixture.target,
            &references[..2],
            &hash,
            &fixture.limits,
            fixture.metadata
        )
        .is_err()
    );
}

#[test]
fn real_reservation_contract_boundaries_refuse_before_io_and_body_allocation() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    assert!(matches!(
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata - 1),
        Err(CheckpointError::ResourceReservation)
    ));
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert!(matches!(
        write_checkpoint_proof_witness(&mut transfer, 0, &fixture.blobs[0], fixture.io - 1),
        Err(CheckpointError::ResourceReservation)
    ));
    assert!(!witness_path(&directory, &fixture, 0).exists());
    write_all(&mut transfer, &fixture);
    let completed = complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap();
    let required = required_checkpoint_proof_witness_reservation(&completed, 0).unwrap();
    assert!(matches!(
        read_checkpoint_proof_witness(&completed, 0, required - 1),
        Err(CheckpointError::ResourceReservation)
    ));
    assert!(required_checkpoint_proof_witness_reservation(&completed, usize::MAX).is_err());
}

#[test]
fn count_body_manifest_disk_and_witness_bounds_are_all_enforced() {
    let fixture = fixture();
    for selector in 0..5 {
        let mut limits = fixture.limits;
        match selector {
            0 => limits.maximum_files = 2,
            1 => limits.maximum_total_bytes = 1_000,
            2 => limits.maximum_manifest_bytes = fixture.manifest.len() - 1,
            3 => {
                let total = fixture.blobs.iter().map(Vec::len).sum::<usize>();
                limits.maximum_total_bytes = total;
                limits.maximum_disk_bytes = total + 2 * fixture.manifest.len() + 160 - 1;
            }
            _ => limits.maximum_witness_bytes = 1_023,
        }
        assert!(
            preflight_checkpoint_proof_manifest(
                &fixture.manifest,
                fixture.target.height,
                &fixture.version,
                &fixture.snapshot,
                &fixture.body,
                &limits
            )
            .is_err()
        );
    }
    let mut limits = fixture.limits;
    limits.maximum_files = 10_002;
    assert!(matches!(
        required_checkpoint_proof_metadata_reservation(&limits),
        Err(CheckpointError::InvalidLimits)
    ));
}
