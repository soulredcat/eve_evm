// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;

#[test]
fn pending_probe_is_read_only_for_absence_published_manifest_matching_unknown_and_completion_presence()
 {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let observe = |kind| {
        observe_checkpoint_pending_metadata(&root, &manifest(&fixture), kind, fixture.metadata)
            .unwrap()
    };
    assert_eq!(
        observe(CheckpointPendingMetadataKind::Manifest),
        CheckpointPendingMetadataStatus::Missing
    );
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    drop(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap());
    assert_eq!(
        observe(CheckpointPendingMetadataKind::Manifest),
        CheckpointPendingMetadataStatus::Missing
    );
    let pending = namespace(&directory, &fixture).join("manifest.pending");
    std::fs::write(&pending, &fixture.manifest).unwrap();
    assert_eq!(
        observe(CheckpointPendingMetadataKind::Manifest),
        CheckpointPendingMetadataStatus::MatchingValid
    );
    assert_eq!(std::fs::read(&pending).unwrap(), fixture.manifest);
    std::fs::write(&pending, b"partial").unwrap();
    assert_eq!(
        observe(CheckpointPendingMetadataKind::Manifest),
        CheckpointPendingMetadataStatus::Refused
    );
    assert_eq!(std::fs::read(&pending).unwrap(), b"partial");
    std::fs::write(&pending, &fixture.manifest[..24]).unwrap();
    assert_eq!(
        observe(CheckpointPendingMetadataKind::Manifest),
        CheckpointPendingMetadataStatus::InvalidKnown
    );
    assert_eq!(std::fs::read(&pending).unwrap(), fixture.manifest[..24]);
    std::fs::write(
        namespace(&directory, &fixture).join("complete.bin"),
        b"presence only",
    )
    .unwrap();
    assert_eq!(
        observe(CheckpointPendingMetadataKind::Completion),
        CheckpointPendingMetadataStatus::PublishedCompletionPresent
    );
    assert_eq!(std::fs::read(&pending).unwrap(), fixture.manifest[..24]);
}

#[test]
fn pending_probe_metadata_boundary_and_oversize_refuse_without_allocating_or_mutating_namespace() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    assert!(matches!(
        observe_checkpoint_pending_metadata(
            &root,
            &manifest(&fixture),
            CheckpointPendingMetadataKind::Manifest,
            fixture.metadata - 1
        ),
        Err(CheckpointError::ResourceReservation)
    ));
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    create_namespace(&directory, &fixture);
    let path = namespace(&directory, &fixture).join("manifest.pending");
    let file = std::fs::File::create(&path).unwrap();
    file.set_len(fixture.limits.maximum_manifest_bytes as u64 + 1)
        .unwrap();
    assert_eq!(
        observe_checkpoint_pending_metadata(
            &root,
            &manifest(&fixture),
            CheckpointPendingMetadataKind::Manifest,
            fixture.metadata
        )
        .unwrap(),
        CheckpointPendingMetadataStatus::Refused
    );
    assert_eq!(
        std::fs::metadata(path).unwrap().len(),
        fixture.limits.maximum_manifest_bytes as u64 + 1
    );
}
