// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{config, empty_chain};
use crate::{
    persistence::{handoff::observe_handoff, worker::install_record_append_pause},
    sync::applied::*,
};
use eve_finality_verifier::{decode_compact_recovery_envelope, encode_compact_recovery_envelope};
use eve_state::{Bytes, build_state_commit};
use std::{sync::Arc, time::Duration};

#[test]
fn invalid_proof_or_execution_leaves_publication_and_admitted_cursor_unchanged() {
    let directory = tempfile::tempdir().unwrap();
    let chain = empty_chain();
    let config = config(&directory.path().join("applied"), &chain);
    let budget = config.state_budget;
    let (mut owner, reader) = open_applied_state_service(config, &chain.genesis)
        .ok()
        .unwrap();
    let original = capture_applied_state(&reader).unwrap();
    let original_cursor = owner.admitted_cursor;
    let mut bad_proof = decode_compact_recovery_envelope(&chain.records[0], &budget).unwrap();
    bad_proof.finalized.commit.signatures[0].signature[0] ^= 1;
    let bad_proof = encode_compact_recovery_envelope(&bad_proof, &budget).unwrap();
    let mut bad_execution = decode_compact_recovery_envelope(&chain.records[0], &budget).unwrap();
    let mut block = bad_execution.execution.clone();
    block.header.beneficiary = eve_state::Address::repeat_byte(0x70);
    let structural = build_state_commit(
        Some(bad_execution.parent.clone()),
        chain.commits[1].state.clone(),
        block,
        &budget,
    )
    .unwrap();
    bad_execution.expected = structural.target;
    bad_execution.execution = structural.block;
    let bad_execution = encode_compact_recovery_envelope(&bad_execution, &budget).unwrap();
    let malformed = b"invalid-recovery".to_vec();
    for bytes in [&bad_proof, &bad_execution, &malformed] {
        assert!(try_apply_recovery_bytes(&mut owner, bytes).is_err());
        assert!(Arc::ptr_eq(
            &original,
            &capture_applied_state(&reader).unwrap()
        ));
        assert_eq!(owner.admitted_cursor, original_cursor);
        assert_eq!(observe_handoff(&owner.pool).unwrap().retained_bytes, 0);
    }
    assert_eq!(
        try_apply_recovery_bytes(&mut owner, &chain.records[0])
            .unwrap()
            .applied
            .0,
        1
    );
    drop(finish_applied_state_service(owner));
}

#[test]
fn execution_and_lookahead_transactions_reject_before_handoff_or_replay() {
    let directory = tempfile::tempdir().unwrap();
    let chain = empty_chain();
    let config = config(&directory.path().join("applied"), &chain);
    let budget = config.state_budget;
    let (mut owner, reader) = open_applied_state_service(config, &chain.genesis)
        .ok()
        .unwrap();
    let before = capture_applied_state(&reader).unwrap();
    for lookahead in [false, true] {
        let mut envelope = decode_compact_recovery_envelope(&chain.records[0], &budget).unwrap();
        if lookahead {
            envelope
                .lookahead
                .transactions
                .push(Bytes::from_static(&[0xc0]));
        } else {
            envelope
                .execution
                .transactions
                .push(Bytes::from_static(&[0xc0]));
            envelope
                .execution
                .receipts
                .push(Bytes::from_static(&[0xc0]));
        }
        let bytes = encode_compact_recovery_envelope(&envelope, &budget).unwrap();
        assert!(try_apply_recovery_bytes(&mut owner, &bytes).is_err());
        assert!(Arc::ptr_eq(
            &before,
            &capture_applied_state(&reader).unwrap()
        ));
        assert_eq!(observe_handoff(&owner.pool).unwrap().retained_batches, 0);
    }
    drop(finish_applied_state_service(owner));
}

#[test]
fn bounded_queue_rejection_preserves_the_last_complete_view_and_cursor() {
    let directory = tempfile::tempdir().unwrap();
    let chain = empty_chain();
    let mut config = config(&directory.path().join("applied"), &chain);
    config.public_budget.queue_batches = 1;
    let (mut owner, reader) = open_applied_state_service(config, &chain.genesis)
        .ok()
        .unwrap();
    let (entered, resume) = install_record_append_pause(owner.worker.as_ref().unwrap());
    let admitted = try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    let before = capture_applied_state(&reader).unwrap();
    let rejected = try_apply_recovery_bytes(&mut owner, &chain.records[1]);
    let after = capture_applied_state(&reader).unwrap();
    resume.send(()).unwrap();
    assert!(matches!(rejected, Err(AppliedError::QueueLimit)));
    assert!(Arc::ptr_eq(&before, &after));
    assert_eq!(owner.admitted_cursor, admitted.admitted_cursor);
    assert_eq!(applied_markers(&after).durable_recovery.0, 0);
    drop(finish_applied_state_service(owner));
}
