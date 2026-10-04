// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*, fixtures::limits, pending_metadata_fixture::*, reopen_fixtures::private_archive,
};
use crate::sync::applied::{capture_applied_state, finish_applied_state_service};
use std::{fs::File, sync::Arc};

#[test]
fn actual_charged_content_known_pending_repairs_resume_and_preserve_original_view() {
    for manifest_pending in [true, false] {
        let fixture = fixture();
        let old = capture_applied_state(&fixture.reader).unwrap();
        let staging = copied_staging(&fixture, false, manifest_pending);
        let path = namespace(&staging, &fixture.artifacts.content.content_id);
        let pending = if manifest_pending {
            "manifest.pending"
        } else {
            "complete.pending"
        };
        let partial = if manifest_pending {
            b"EVE_CHECKPOINT_STORE_V1\0".as_slice()
        } else {
            b"EVE_CKPT_DONE_V1".as_slice()
        };
        std::fs::write(path.join(pending), partial).unwrap();
        let root = File::open(staging.path()).unwrap();
        let outcome = repair_applied_checkpoint_content_pending(
            &fixture.owner,
            &root,
            content_manifest(&fixture),
            &fixture.target_encoding,
            limits(),
        )
        .unwrap();
        assert_eq!(
            if manifest_pending {
                outcome.manifest
            } else {
                outcome.completion
            },
            CheckpointMetadataRepairStatus::RemovedInvalid
        );
        assert!(!path.join(pending).exists());
        let transfer = begin_applied_checkpoint_transfer(
            &fixture.owner,
            &root,
            content_manifest(&fixture),
            &fixture.target_encoding,
            limits(),
        )
        .unwrap();
        drop(complete_applied_checkpoint_transfer(transfer).unwrap());
        assert!(Arc::ptr_eq(
            &old,
            &capture_applied_state(&fixture.reader).unwrap()
        ));
        assert!(
            namespace(&fixture.archive, &fixture.artifacts.content.content_id)
                .join("complete.bin")
                .exists()
        );
        drop(fixture.artifacts);
        drop(finish_applied_state_service(fixture.owner));
    }
}

#[test]
fn missing_and_matching_valid_metadata_do_not_trigger_repairs_or_create_namespaces() {
    let fixture = fixture();
    let staging = private_archive();
    let root = File::open(staging.path()).unwrap();
    let missing = repair_applied_checkpoint_content_pending(
        &fixture.owner,
        &root,
        content_manifest(&fixture),
        &fixture.target_encoding,
        limits(),
    )
    .unwrap();
    assert_eq!(missing.manifest, CheckpointMetadataRepairStatus::Missing);
    assert_eq!(missing.completion, CheckpointMetadataRepairStatus::Missing);
    let missing = repair_applied_checkpoint_proof_pending(
        &fixture.artifacts.content,
        &root,
        proof_manifest(&fixture),
    )
    .unwrap();
    assert_eq!(missing.manifest, CheckpointMetadataRepairStatus::Missing);
    assert_eq!(std::fs::read_dir(staging.path()).unwrap().count(), 0);
    let matching = copied_staging(&fixture, false, true);
    let path = namespace(&matching, &fixture.artifacts.content.content_id).join("manifest.pending");
    std::fs::write(&path, content_manifest(&fixture)).unwrap();
    let outcome = repair_applied_checkpoint_content_pending(
        &fixture.owner,
        &File::open(matching.path()).unwrap(),
        content_manifest(&fixture),
        &fixture.target_encoding,
        limits(),
    )
    .unwrap();
    assert_eq!(
        outcome.manifest,
        CheckpointMetadataRepairStatus::MatchingValid
    );
    assert_eq!(std::fs::read(path).unwrap(), content_manifest(&fixture));
    drop(fixture.artifacts);
    drop(finish_applied_state_service(fixture.owner));
}
