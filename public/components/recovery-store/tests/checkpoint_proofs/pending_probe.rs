// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::{CheckpointError, CheckpointPendingMetadataStatus, proofs::*};

#[test]
fn proof_pending_probe_distinguishes_absence_matching_known_invalid_unknown_and_marker_presence_without_writes()
 {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let observe = |kind| {
        observe_checkpoint_proof_pending_metadata(
            &root,
            &manifest(&fixture),
            kind,
            fixture.metadata,
        )
        .unwrap()
    };
    assert_eq!(
        observe(CheckpointProofPendingKind::Manifest),
        CheckpointPendingMetadataStatus::Missing
    );
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    drop(begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap());
    assert_eq!(
        observe(CheckpointProofPendingKind::Manifest),
        CheckpointPendingMetadataStatus::Missing
    );
    let pending = namespace(&directory, &fixture).join("manifest.pending");
    std::fs::write(&pending, &fixture.manifest).unwrap();
    assert_eq!(
        observe(CheckpointProofPendingKind::Manifest),
        CheckpointPendingMetadataStatus::MatchingValid
    );
    assert_eq!(std::fs::read(&pending).unwrap(), fixture.manifest);
    std::fs::write(&pending, &fixture.manifest[..24]).unwrap();
    assert_eq!(
        observe(CheckpointProofPendingKind::Manifest),
        CheckpointPendingMetadataStatus::InvalidKnown
    );
    assert_eq!(std::fs::read(&pending).unwrap(), fixture.manifest[..24]);
    std::fs::write(&pending, b"unknown future metadata").unwrap();
    assert_eq!(
        observe(CheckpointProofPendingKind::Manifest),
        CheckpointPendingMetadataStatus::Refused
    );
    assert_eq!(std::fs::read(&pending).unwrap(), b"unknown future metadata");
    std::fs::write(
        namespace(&directory, &fixture).join("complete.bin"),
        b"unverified presence",
    )
    .unwrap();
    assert_eq!(
        observe(CheckpointProofPendingKind::Completion),
        CheckpointPendingMetadataStatus::PublishedCompletionPresent
    );
    assert_eq!(std::fs::read(&pending).unwrap(), b"unknown future metadata");
}

#[test]
fn proof_pending_probe_short_metadata_reservation_and_oversize_refuse_without_source_mutation() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    assert!(matches!(
        observe_checkpoint_proof_pending_metadata(
            &root,
            &manifest(&fixture),
            CheckpointProofPendingKind::Manifest,
            fixture.metadata - 1
        ),
        Err(CheckpointError::ResourceReservation)
    ));
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    create_namespace(&directory, &fixture);
    let path = namespace(&directory, &fixture).join("complete.pending");
    let bytes = vec![0x97_u8; 81];
    std::fs::write(&path, &bytes).unwrap();
    assert_eq!(
        observe_checkpoint_proof_pending_metadata(
            &root,
            &manifest(&fixture),
            CheckpointProofPendingKind::Completion,
            fixture.metadata
        )
        .unwrap(),
        CheckpointPendingMetadataStatus::Refused
    );
    assert_eq!(std::fs::read(path).unwrap(), bytes);
}
