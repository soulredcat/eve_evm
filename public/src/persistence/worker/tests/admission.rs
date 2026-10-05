// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{fixture, install_pause, payload};
use crate::persistence::{handoff::*, worker::*};
use eve_storage::records::*;
use std::{sync::Arc, time::Duration};

#[test]
fn foreign_pool_submission_returns_original_charged_payload() {
    let fixture = fixture();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    let foreign = create_handoff_pool(eve_node_policy::development_public_budget())
        .ok()
        .unwrap();
    let worker = start_record_worker(
        fixture.repository,
        Arc::clone(&fixture.pool),
        40 * 1_048_576,
    )
    .ok()
    .unwrap();
    let rejected = try_submit_record(&worker, parent, payload(&foreign, b"foreign"))
        .err()
        .unwrap();
    assert_eq!(rejected.error, RecordWorkerError::ForeignPool);
    assert_eq!(recovery_payload_bytes(&rejected.payload), b"foreign");
    assert_eq!(observe_handoff(&foreign).unwrap().retained_bytes, 7);
    drop(rejected);
    assert_eq!(observe_handoff(&foreign).unwrap().retained_bytes, 0);
    assert_eq!(
        opaque_record_cursor(&finish_record_worker(worker).ok().unwrap()).unwrap(),
        parent
    );
}

#[test]
fn concurrent_clone_submission_cannot_bypass_charged_batch_count() {
    let fixture = fixture();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    let worker = start_record_worker(
        fixture.repository,
        Arc::clone(&fixture.pool),
        40 * 1_048_576,
    )
    .ok()
    .unwrap();
    let (entered, resume) = install_pause(&worker);
    let first = payload(&fixture.pool, b"same-record");
    let ticket = try_submit_record(&worker, parent, first.clone())
        .ok()
        .unwrap();
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    let rejected = try_submit_record(&worker, parent, first).err().unwrap();
    let error = rejected.error;
    drop(rejected);
    resume.send(()).unwrap();
    assert_eq!(error, RecordWorkerError::AlreadySubmitted);
    assert_eq!(
        receive_record_ack(ticket).unwrap().disposition,
        OpaqueRecordDisposition::NewlySynced
    );
    finish_record_worker(worker).ok().unwrap();
    assert_eq!(observe_handoff(&fixture.pool).unwrap().retained_batches, 0);
}

#[test]
fn shutdown_drains_bounded_admitted_queue_including_exact_replay() {
    let fixture = fixture();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    let worker = start_record_worker(
        fixture.repository,
        Arc::clone(&fixture.pool),
        40 * 1_048_576,
    )
    .ok()
    .unwrap();
    let (entered, resume) = install_pause(&worker);
    let mut tickets = vec![
        try_submit_record(&worker, parent, payload(&fixture.pool, b"identical"))
            .ok()
            .unwrap(),
    ];
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    for _ in 0..3 {
        tickets.push(
            try_submit_record(&worker, parent, payload(&fixture.pool, b"identical"))
                .ok()
                .unwrap(),
        );
    }
    let observation = observe_handoff(&fixture.pool).unwrap();
    let exhausted = matches!(
        reserve_recovery_payload(&fixture.pool, 1),
        Err(HandoffError::QueueLimit)
    );
    resume.send(()).unwrap();
    let repository = finish_record_worker(worker).ok().unwrap();
    assert_eq!(observation.retained_batches, 4);
    assert!(exhausted);
    for ticket in tickets {
        assert_eq!(receive_record_ack(ticket).unwrap().store_head.sequence, 1);
    }
    assert_eq!(opaque_record_cursor(&repository).unwrap().sequence, 1);
    assert_eq!(observe_handoff(&fixture.pool).unwrap().retained_batches, 0);
}
