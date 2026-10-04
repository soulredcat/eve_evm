// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{super::*, fixtures::limits, pending_metadata_fixture::*};
use crate::sync::applied::finish_applied_state_service;
use std::fs::File;

#[test]
fn published_completion_presence_skips_repairs_and_preserves_original_bytes() {
    let fixture = fixture();
    // Create independent bounded namespace copies under task-owned staging.
    for proof in [false, true] {
        let staging = super::reopen_fixtures::private_archive();
        let id = if proof {
            &fixture.artifacts.proof_id
        } else {
            &fixture.artifacts.content.content_id
        };
        super::reopen_fixtures::copy_namespace(fixture.archive.path(), staging.path(), id);
        let path = namespace(&staging, id);
        let before = std::fs::read(path.join("complete.bin")).unwrap();
        let root = File::open(staging.path()).unwrap();
        let result = if proof {
            repair_applied_checkpoint_proof_pending(
                &fixture.artifacts.content,
                &root,
                proof_manifest(&fixture),
            )
        } else {
            repair_applied_checkpoint_content_pending(
                &fixture.owner,
                &root,
                content_manifest(&fixture),
                &fixture.target_encoding,
                limits(),
            )
        };
        let outcome = result.unwrap();
        assert_eq!(
            outcome.manifest,
            CheckpointMetadataRepairStatus::PublishedCompletionPresent
        );
        assert_eq!(
            outcome.completion,
            CheckpointMetadataRepairStatus::PublishedCompletionPresent
        );
        assert_eq!(std::fs::read(path.join("complete.bin")).unwrap(), before);
    }
    drop(fixture.artifacts);
    drop(finish_applied_state_service(fixture.owner));
}
