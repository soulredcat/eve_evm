// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    fixtures::small_state_budget,
    import_fixtures::{import_chain, import_config},
};
use crate::sync::applied::*;
use eve_finality_verifier::{CompactRecoveryEnvelopeV1, encode_compact_recovery_envelope};
use eve_storage::records::{
    compare_and_append_opaque_records, opaque_record_cursor, open_opaque_record_repository,
};

#[test]
fn changing_mode_rejects_the_existing_namespace_at_genesis_in_both_directions() {
    let directory = tempfile::tempdir().unwrap();
    let chain = import_chain();
    for imported in [true, false] {
        let path = directory
            .path()
            .join(if imported { "import" } else { "replay" });
        let mut config = import_config(&path, &chain);
        if !imported {
            config.state_budget = small_state_budget();
        }
        let mode = if imported {
            AppliedMode::AuthenticatedImport
        } else {
            AppliedMode::EmptyReplay
        };
        let (owner, reader) = open_applied_state_service_with_mode(config, &chain.genesis, mode)
            .ok()
            .unwrap();
        assert_eq!(applied_mode(&capture_applied_state(&reader).unwrap()), mode);
        drop(finish_applied_state_service(owner));
        let mut wrong = import_config(&path, &chain);
        let other = if imported {
            wrong.state_budget = small_state_budget();
            AppliedMode::EmptyReplay
        } else {
            AppliedMode::AuthenticatedImport
        };
        assert!(matches!(
            open_applied_state_service_with_mode(wrong, &chain.genesis, other),
            Err(AppliedError::StorageUnavailable)
        ));
    }
}

#[test]
fn imported_namespace_rejects_compact_replay_wire_on_admission_and_reopen_without_reset() {
    let directory = tempfile::tempdir().unwrap();
    let chain = import_chain();
    let path = directory.path().join("import");
    let original = &chain.inputs[0];
    let replay = CompactRecoveryEnvelopeV1 {
        parent: original.journal.parent.clone(),
        expected: chain.commits[1].target.clone(),
        execution: original.execution.clone(),
        finalized: original.finalized.clone(),
        lookahead: original.lookahead.clone(),
    };
    let wrong_wire =
        encode_compact_recovery_envelope(&replay, &eve_state::development_state_budget()).unwrap();
    let (mut owner, reader) = open_applied_state_service_with_mode(
        import_config(&path, &chain),
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    let before = capture_applied_state(&reader).unwrap();
    assert!(matches!(
        try_apply_recovery_bytes(&mut owner, &wrong_wire),
        Err(AppliedError::ImportWire(_))
    ));
    assert_eq!(
        applied_cursors(&before),
        applied_cursors(&capture_applied_state(&reader).unwrap())
    );
    let identity = owner.effective_storage_identity;
    let shutdown = finish_applied_state_service(owner);
    let mut repository = shutdown.repository.unwrap();
    let parent = opaque_record_cursor(&repository).unwrap();
    let ack = compare_and_append_opaque_records(&mut repository, parent, &[wrong_wire]).unwrap();
    drop(repository);
    assert!(matches!(
        open_applied_state_service_with_mode(
            import_config(&path, &chain),
            &chain.genesis,
            AppliedMode::AuthenticatedImport
        ),
        Err(AppliedError::ImportWire(_))
    ));
    let config = import_config(&path, &chain);
    let repository =
        open_opaque_record_repository(&path, identity, config.repository_budget).unwrap();
    assert_eq!(opaque_record_cursor(&repository).unwrap(), ack.appended);
}
