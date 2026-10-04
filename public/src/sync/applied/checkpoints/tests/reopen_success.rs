// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*,
    reopen_fixtures::{append_tail, completed_checkpoint, recovery_config, reopen},
};
use crate::sync::applied::{
    applied_commit, applied_cursors, applied_markers, applied_readiness, capture_applied_state,
    finish_applied_state_service, open_segmented_applied_state_service,
    segmented::tests::fixtures::{configuration, logical_chain},
};

#[test]
fn complete_nonempty_checkpoint_and_canonical_tail_reopen_exact_roots_receipts_markers_and_cursors()
{
    let fixture = completed_checkpoint(true);
    let (owner, reader) = reopen(&fixture).unwrap();
    let restored = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_commit(&restored), &fixture.chain.commits[2]);
    assert_eq!(applied_commit(&restored), applied_commit(&fixture.expected));
    assert_eq!(
        applied_cursors(&restored),
        applied_cursors(&fixture.expected)
    );
    assert_eq!(
        applied_markers(&restored),
        applied_markers(&fixture.expected)
    );
    assert_eq!(applied_markers(&restored).checkpoint.0, 1);
    assert_eq!(applied_markers(&restored).authenticated_snapshot_height, 1);
    assert!(matches!(
        applied_readiness(&restored),
        eve_node_policy::PublicReadiness::NotReady(_)
    ));
    drop(finish_applied_state_service(owner));
}

#[test]
fn complete_base_without_suffix_uses_explicit_base_startup_and_accepts_a_verified_tail() {
    let fixture = completed_checkpoint(false);
    assert!(
        open_segmented_applied_state_service(
            configuration(&fixture.database.path().join("store"), &fixture.chain),
            &fixture.chain.genesis
        )
        .is_err()
    );
    let (mut owner, reader) = reopen(&fixture).unwrap();
    let checkpoint = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_commit(&checkpoint), &fixture.chain.commits[1]);
    assert_eq!(applied_cursors(&checkpoint).0.sequence, 1);
    append_tail(&mut owner, &fixture.chain);
    let updated = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_commit(&updated), &fixture.chain.commits[2]);
    assert_eq!(applied_markers(&updated).checkpoint.0, 1);
    assert_eq!(applied_commit(&checkpoint), &fixture.chain.commits[1]);
    drop(finish_applied_state_service(owner));
}

#[test]
fn no_typed_base_does_not_open_or_create_missing_artifact_roots() {
    let database = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let absent = database.path().join("not-created");
    let (owner, reader) = open_segmented_applied_state_service_with_checkpoints(
        configuration(&database.path().join("store"), &chain),
        recovery_config(&absent),
        &chain.genesis,
    )
    .unwrap();
    let restored = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_commit(&restored), &chain.commits[0]);
    assert_eq!(applied_markers(&restored).checkpoint.0, 0);
    assert!(!absent.exists());
    drop(finish_applied_state_service(owner));
}
