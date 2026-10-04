// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::{CheckpointError, proofs::*};

#[test]
fn unknown_future_manifest_magic_and_codec_flags_are_preserved_with_admission_refused() {
    for selector in 0..3 {
        let fixture = fixture();
        let (directory, root) = temporary_root();
        create_namespace(&directory, &fixture);
        let mut future = fixture.manifest.clone();
        match selector {
            0 => future[22] = b'2',
            1 => future[24] = 1,
            _ => future[25] = 1,
        }
        let path = namespace(&directory, &fixture).join("manifest.pending");
        std::fs::write(&path, &future).unwrap();
        assert!(
            preflight_checkpoint_proof_manifest(
                &future,
                fixture.target.height,
                &fixture.version,
                &fixture.snapshot,
                &fixture.body,
                &fixture.limits
            )
            .is_err()
        );
        assert!(matches!(
            repair_invalid_checkpoint_proof_pending(
                &root,
                &manifest(&fixture),
                CheckpointProofPendingKind::Manifest,
                fixture.metadata
            ),
            Err(CheckpointError::ValidEntry)
        ));
        assert_eq!(std::fs::read(path).unwrap(), future);
    }
}

#[test]
fn unknown_future_completion_magic_or_short_unknown_metadata_is_preserved_without_promotion() {
    let mut future = vec![0x63_u8; 80];
    future[..16].copy_from_slice(b"EVE_PROOF_DONE_2");
    for bytes in [future, b"partial".to_vec()] {
        let fixture = fixture();
        let (directory, root) = temporary_root();
        drop(
            begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap(),
        );
        let path = namespace(&directory, &fixture).join("complete.pending");
        std::fs::write(&path, &bytes).unwrap();
        assert!(matches!(
            repair_invalid_checkpoint_proof_pending(
                &root,
                &manifest(&fixture),
                CheckpointProofPendingKind::Completion,
                fixture.metadata
            ),
            Err(CheckpointError::ValidEntry)
        ));
        assert_eq!(std::fs::read(path).unwrap(), bytes);
        assert!(
            !namespace(&directory, &fixture)
                .join("complete.bin")
                .exists()
        );
    }
}

#[test]
fn oversized_manifest_and_completion_entries_are_preserved_before_owned_read() {
    for (kind, length) in [
        (CheckpointProofPendingKind::Manifest, 8_192_usize),
        (CheckpointProofPendingKind::Manifest, 1_048_577),
        (CheckpointProofPendingKind::Completion, 81),
    ] {
        let fixture = fixture();
        let (directory, root) = temporary_root();
        let name = match kind {
            CheckpointProofPendingKind::Manifest => {
                create_namespace(&directory, &fixture);
                "manifest.pending"
            }
            CheckpointProofPendingKind::Completion => {
                drop(
                    begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata)
                        .unwrap(),
                );
                "complete.pending"
            }
        };
        let bytes = vec![0x72_u8; length];
        let path = namespace(&directory, &fixture).join(name);
        std::fs::write(&path, &bytes).unwrap();
        assert!(matches!(
            repair_invalid_checkpoint_proof_pending(
                &root,
                &manifest(&fixture),
                kind,
                fixture.metadata
            ),
            Err(CheckpointError::ValidEntry)
        ));
        assert_eq!(std::fs::metadata(&path).unwrap().len(), length as u64);
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }
}

#[test]
fn known_truncated_v1_entries_remain_explicitly_repairable_without_touching_valid_data() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let manifest_pending = namespace(&directory, &fixture).join("manifest.pending");
    std::fs::write(&manifest_pending, &fixture.manifest[..24]).unwrap();
    repair_invalid_checkpoint_proof_pending(
        &root,
        &manifest(&fixture),
        CheckpointProofPendingKind::Manifest,
        fixture.metadata,
    )
    .unwrap();
    assert!(!manifest_pending.exists());
    drop(begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap());
    let before = std::fs::read(namespace(&directory, &fixture).join("manifest.bin")).unwrap();
    let completion_pending = namespace(&directory, &fixture).join("complete.pending");
    std::fs::write(&completion_pending, b"EVE_PROOF").unwrap();
    repair_invalid_checkpoint_proof_pending(
        &root,
        &manifest(&fixture),
        CheckpointProofPendingKind::Completion,
        fixture.metadata,
    )
    .unwrap();
    assert!(!completion_pending.exists());
    assert_eq!(
        std::fs::read(namespace(&directory, &fixture).join("manifest.bin")).unwrap(),
        before
    );
}
