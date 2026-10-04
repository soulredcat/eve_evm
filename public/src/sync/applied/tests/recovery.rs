// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{config, empty_chain};
use crate::{persistence::handoff::observe_handoff, sync::applied::*};
use eve_storage::records::{
    compare_and_append_opaque_records, opaque_record_cursor, open_opaque_record_repository,
};
use std::sync::Arc;

#[test]
fn rejected_storage_acknowledgement_keeps_exact_charged_tail_through_shutdown() {
    let directory = tempfile::tempdir().unwrap();
    let chain = empty_chain();
    let path = directory.path().join("applied");
    let (mut owner, reader) = open_applied_state_service(config(&path, &chain), &chain.genesis)
        .ok()
        .unwrap();
    let pool = Arc::clone(crate::sync::applied::admission::compact_pool(&owner).unwrap());
    // Unit-only control-cursor corruption triggers an actual repository rejection.
    // This is neither a hardware fsync failure nor a production control surface.
    owner.admitted_cursor.content_hash[0] ^= 1;
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.repository.is_err());
    assert!(shutdown.acknowledgement_error.is_some());
    assert_eq!(retained_tail_len(&shutdown.unacknowledged_tail), 1);
    assert_eq!(
        retained_tail_bytes(&shutdown.unacknowledged_tail, 0),
        Some(chain.records[0].as_slice())
    );
    assert_eq!(observe_handoff(&pool).unwrap().retained_batches, 1);
    let view = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_markers(&view).applied.0, 1);
    assert_eq!(applied_markers(&view).durable_recovery.0, 0);
    assert!(applied_storage_failed(&view));
    let (reopened, reopened_reader) =
        open_applied_state_service(config(&path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    assert_eq!(
        applied_commit(&capture_applied_state(&reopened_reader).unwrap()),
        &chain.commits[0]
    );
    drop(finish_applied_state_service(reopened));
    drop(shutdown);
    assert_eq!(observe_handoff(&pool).unwrap().retained_batches, 0);
}

#[test]
fn reopening_rejects_an_opaque_synced_but_unverifiable_prefix_without_resetting_it() {
    let directory = tempfile::tempdir().unwrap();
    let chain = empty_chain();
    let path = directory.path().join("applied");
    let initial = config(&path, &chain);
    let mut repository =
        open_opaque_record_repository(&path, initial.identity, initial.repository_budget).unwrap();
    let parent = opaque_record_cursor(&repository).unwrap();
    let ack =
        compare_and_append_opaque_records(&mut repository, parent, &[b"invalid-recovery".to_vec()])
            .unwrap();
    drop(repository);
    assert!(open_applied_state_service(config(&path, &chain), &chain.genesis).is_err());
    let repository =
        open_opaque_record_repository(&path, initial.identity, initial.repository_budget).unwrap();
    assert_eq!(opaque_record_cursor(&repository).unwrap(), ack.appended);
}

#[test]
fn default_unfit_state_budget_and_wrong_genesis_refuse_before_creating_namespace() {
    let directory = tempfile::tempdir().unwrap();
    let chain = empty_chain();
    let path = directory.path().join("applied");
    let mut initial = config(&path, &chain);
    initial.state_budget = eve_state::development_state_budget();
    assert!(matches!(
        open_applied_state_service(initial, &chain.genesis),
        Err(AppliedError::EstimatedCapacity)
    ));
    assert!(!path.exists());
    let mut initial = config(&path, &chain);
    initial.identity.genesis_hash[0] ^= 1;
    assert!(matches!(
        open_applied_state_service(initial, &chain.genesis),
        Err(AppliedError::InvalidConfiguration)
    ));
    assert!(!path.exists());
    let mut pathological = config(&path, &chain);
    pathological.public_budget.queue_batches = u64::MAX;
    assert!(open_applied_state_service(pathological, &chain.genesis).is_err());
    assert!(!path.exists());
}
