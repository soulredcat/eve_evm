// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::import_fixtures::{import_chain, import_config};
use crate::{
    persistence::{handoff::observe_handoff, worker::install_record_append_pause},
    sync::applied::*,
};
use eve_finality_verifier::encode_authenticated_import_wire;
use std::{sync::Arc, time::Duration};

#[test]
fn failed_import_preflight_proof_and_parent_preserve_all_published_state_and_charges() {
    let directory = tempfile::tempdir().unwrap();
    let chain = import_chain();
    let (mut owner, reader) = open_applied_state_service_with_mode(
        import_config(&directory.path().join("import"), &chain),
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    let before = capture_applied_state(&reader).unwrap();
    let charged = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes;
    let mut bad_proof = chain.inputs[0].clone();
    bad_proof.finalized.commit.signatures[0].signature[0] ^= 1;
    let mut bad_parent = chain.inputs[0].clone();
    bad_parent.journal.parent.content_digest.0[0] ^= 1;
    let records = [
        b"invalid-import-wire".to_vec(),
        encode_authenticated_import_wire(&bad_proof, &owner.config.state_budget).unwrap(),
        encode_authenticated_import_wire(&bad_parent, &owner.config.state_budget).unwrap(),
    ];
    for bytes in records {
        assert!(try_apply_recovery_bytes(&mut owner, &bytes).is_err());
        let after = capture_applied_state(&reader).unwrap();
        assert!(Arc::ptr_eq(&before, &after));
        assert_eq!(owner.admitted_cursor, applied_cursors(&before).0);
        assert_eq!(
            observe_estimated_working(&reader)
                .unwrap()
                .reserved_estimated_bytes,
            charged
        );
        assert_eq!(
            observe_handoff(crate::sync::applied::admission::compact_pool(&owner).unwrap())
                .unwrap()
                .retained_bytes,
            0
        );
    }
    drop(finish_applied_state_service(owner));
}

#[test]
fn imported_queue_refusal_preserves_the_verified_parent_and_admitted_cursor() {
    let directory = tempfile::tempdir().unwrap();
    let chain = import_chain();
    let mut config = import_config(&directory.path().join("import"), &chain);
    config.public_budget.queue_batches = 1;
    let (mut owner, reader) = open_applied_state_service_with_mode(
        config,
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    let (entered, resume) = install_record_append_pause(
        crate::sync::applied::admission::compact_worker(&owner).unwrap(),
    );
    let first = try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    let before = capture_applied_state(&reader).unwrap();
    let result = try_apply_recovery_bytes(&mut owner, &chain.records[1]);
    let after = capture_applied_state(&reader).unwrap();
    resume.send(()).unwrap();
    assert!(matches!(result, Err(AppliedError::QueueLimit)));
    assert!(Arc::ptr_eq(&before, &after));
    assert_eq!(owner.admitted_cursor, first.admitted_cursor);
    assert_eq!(applied_markers(&after).durable_recovery.0, 0);
    drop(finish_applied_state_service(owner));
}

#[test]
fn failed_import_storage_keeps_exact_tail_and_its_charge_through_shutdown() {
    let directory = tempfile::tempdir().unwrap();
    let chain = import_chain();
    let path = directory.path().join("import");
    let (mut owner, reader) = open_applied_state_service_with_mode(
        import_config(&path, &chain),
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    let pool = Arc::clone(crate::sync::applied::admission::compact_pool(&owner).unwrap());
    // Unit-only control-cursor corruption; actual repository refusal, not hardware failure.
    owner.admitted_cursor.content_hash[0] ^= 1;
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.repository.is_err());
    assert!(shutdown.acknowledgement_error.is_some());
    assert_eq!(
        retained_tail_bytes(&shutdown.unacknowledged_tail, 0),
        Some(chain.records[0].as_slice())
    );
    assert_eq!(observe_handoff(&pool).unwrap().retained_batches, 1);
    let failed = capture_applied_state(&reader).unwrap();
    assert!(applied_storage_failed(&failed));
    assert_eq!(applied_commit(&failed), &chain.commits[1]);
    assert_eq!(applied_markers(&failed).durable_recovery.0, 0);
    let (reopened, restored) = open_applied_state_service_with_mode(
        import_config(&path, &chain),
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    assert_eq!(
        applied_commit(&capture_applied_state(&restored).unwrap()),
        &chain.commits[0]
    );
    drop(finish_applied_state_service(reopened));
    drop(shutdown);
    assert_eq!(observe_handoff(&pool).unwrap().retained_bytes, 0);
}
