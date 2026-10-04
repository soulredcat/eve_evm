// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;

#[test]
fn future_completion_magic_and_short_unknown_metadata_are_preserved() {
    let mut future = vec![0x73_u8; 80];
    future[..16].copy_from_slice(b"EVE_CKPT_DONE_V2");
    for bytes in [future, b"partial".to_vec(), b"FUTURE_FORMAT_V3".to_vec()] {
        let fixture = fixture();
        let (directory, root) = temporary_root();
        drop(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap());
        let path = namespace(&directory, &fixture).join("complete.pending");
        std::fs::write(&path, &bytes).unwrap();
        assert!(matches!(
            repair_invalid_checkpoint_completion_pending(
                &root,
                &manifest(&fixture),
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
fn oversized_completion_is_preserved_before_bounded_owned_read() {
    for length in [81, 131_072] {
        let fixture = fixture();
        let (directory, root) = temporary_root();
        drop(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap());
        let bytes = vec![0x74_u8; length];
        let path = namespace(&directory, &fixture).join("complete.pending");
        std::fs::write(&path, &bytes).unwrap();
        assert!(matches!(
            repair_invalid_checkpoint_completion_pending(
                &root,
                &manifest(&fixture),
                fixture.metadata
            ),
            Err(CheckpointError::ValidEntry)
        ));
        assert_eq!(std::fs::metadata(&path).unwrap().len(), length as u64);
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }
}

#[test]
fn exact_known_v1_prefix_is_repairable_only_when_completion_is_truncated() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    drop(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap());
    let before = std::fs::read(namespace(&directory, &fixture).join("manifest.bin")).unwrap();
    let path = namespace(&directory, &fixture).join("complete.pending");
    std::fs::write(&path, b"EVE_CKPT_DONE_V1").unwrap();
    repair_invalid_checkpoint_completion_pending(&root, &manifest(&fixture), fixture.metadata)
        .unwrap();
    assert!(!path.exists());
    assert_eq!(
        std::fs::read(namespace(&directory, &fixture).join("manifest.bin")).unwrap(),
        before
    );
}

#[test]
fn completed_manifest_getter_borrows_exact_retained_bytes_without_path_read() {
    let fixture = fixture();
    let (_directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    let completed = complete_checkpoint_transfer(transfer, fixture.io).unwrap();
    let bytes = checkpoint_store_manifest_bytes(&completed);
    assert_eq!(bytes, fixture.manifest);
    let sealed = preflight_checkpoint_manifest(bytes, &fixture.version, &fixture.limits).unwrap();
    assert_eq!(
        checkpoint_manifest_id(&sealed),
        checkpoint_manifest_id(&manifest(&fixture))
    );
    assert_eq!(
        checkpoint_manifest_stats(&sealed),
        checkpoint_manifest_stats(&manifest(&fixture))
    );
}
