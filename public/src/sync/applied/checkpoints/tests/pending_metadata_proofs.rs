// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{super::*, pending_metadata_fixture::*};
use crate::sync::applied::{capture_applied_state, finish_applied_state_service};
use std::{fs::File, sync::Arc};

#[test]
fn actual_charged_proof_known_pending_repairs_resume_and_preserve_original_artifacts() {
    for manifest_pending in [true, false] {
        let fixture = fixture();
        let old = capture_applied_state(&fixture.reader).unwrap();
        let staging = copied_staging(&fixture, true, manifest_pending);
        let path = namespace(&staging, &fixture.artifacts.proof_id);
        let pending = if manifest_pending {
            "manifest.pending"
        } else {
            "complete.pending"
        };
        let partial = if manifest_pending {
            b"EVE_CHECKPOINT_PROOF_V1\0".as_slice()
        } else {
            b"EVE_PROOF_DONE_1".as_slice()
        };
        std::fs::write(path.join(pending), partial).unwrap();
        let root = File::open(staging.path()).unwrap();
        let outcome = repair_applied_checkpoint_proof_pending(
            &fixture.artifacts.content,
            &root,
            proof_manifest(&fixture),
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
        let (_content_staging, content) = completed_content_copy(&fixture);
        let transfer =
            begin_applied_checkpoint_proofs(content, &root, proof_manifest(&fixture)).unwrap();
        drop(complete_applied_checkpoint_proofs(transfer).unwrap());
        assert!(Arc::ptr_eq(
            &old,
            &capture_applied_state(&fixture.reader).unwrap()
        ));
        assert!(
            namespace(&fixture.archive, &fixture.artifacts.proof_id)
                .join("complete.bin")
                .exists()
        );
        drop(fixture.artifacts);
        drop(finish_applied_state_service(fixture.owner));
    }
}
