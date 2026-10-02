// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{fixture, install_pause, payload};
use crate::persistence::{handoff::observe_handoff, worker::*};
use eve_storage::records::*;
use std::{sync::Arc, time::Duration};

#[test]
fn record_worker_sync_reopen_and_exact_replay_preserve_bytes_and_cursor() {
    let fixture = fixture();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    let worker = start_record_worker(
        fixture.repository,
        Arc::clone(&fixture.pool),
        40 * 1_048_576,
    )
    .ok()
    .unwrap();
    let first = receive_record_ack(
        try_submit_record(&worker, parent, payload(&fixture.pool, b"record-one"))
            .ok()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(first.disposition, OpaqueRecordDisposition::NewlySynced);
    let replay = receive_record_ack(
        try_submit_record(&worker, parent, payload(&fixture.pool, b"record-one"))
            .ok()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(replay.disposition, OpaqueRecordDisposition::ExactReplay);
    assert_eq!(first.appended, replay.appended);
    assert_eq!(first.store_head, replay.store_head);
    let repository = finish_record_worker(worker).ok().unwrap();
    assert_eq!(opaque_record_cursor(&repository).unwrap(), first.store_head);
    drop(repository);
    let reopened =
        open_opaque_record_repository(&fixture.path, fixture.identity, fixture.budget).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), first.store_head);
    assert_eq!(
        read_opaque_record(&reopened, 1).unwrap().unwrap().payload,
        b"record-one"
    );
    assert_eq!(observe_handoff(&fixture.pool).unwrap().retained_bytes, 0);
}

#[test]
fn bad_cursor_latches_worker_failure_and_reopen_preserves_durable_prefix() {
    let fixture = fixture();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    let worker = start_record_worker(
        fixture.repository,
        Arc::clone(&fixture.pool),
        40 * 1_048_576,
    )
    .ok()
    .unwrap();
    let bad = OpaqueRecordCursor {
        sequence: 0,
        content_hash: [99; 32],
    };
    let ticket = try_submit_record(&worker, bad, payload(&fixture.pool, b"bad-parent"))
        .ok()
        .unwrap();
    assert_eq!(
        receive_record_ack(ticket),
        Err(RecordWorkerError::StorageFailed)
    );
    assert!(observe_record_worker(&worker).storage_failed);
    let rejected = try_submit_record(&worker, parent, payload(&fixture.pool, b"must-not-write"))
        .err()
        .unwrap();
    assert_eq!(rejected.error, RecordWorkerError::StorageFailed);
    drop(rejected);
    assert!(matches!(
        finish_record_worker(worker),
        Err(RecordWorkerError::StorageFailed)
    ));
    let reopened =
        open_opaque_record_repository(&fixture.path, fixture.identity, fixture.budget).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), parent);
    assert!(read_opaque_record(&reopened, 1).unwrap().is_none());
    assert_eq!(observe_handoff(&fixture.pool).unwrap().retained_bytes, 0);
}

#[test]
fn cancelled_ticket_retains_payload_until_paused_real_append_finishes() {
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
    let ticket = try_submit_record(&worker, parent, payload(&fixture.pool, b"cancelled-ticket"))
        .ok()
        .unwrap();
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    drop(ticket);
    let retained = observe_handoff(&fixture.pool).unwrap();
    let scratch = observe_record_worker(&worker);
    resume.send(()).unwrap();
    let repository = finish_record_worker(worker).ok().unwrap();
    assert_eq!(retained.retained_bytes, 16);
    assert_eq!(retained.retained_batches, 1);
    assert!(
        scratch.active_scratch_bytes > 0 && scratch.active_scratch_bytes <= scratch.scratch_limit
    );
    assert_eq!(
        read_opaque_record(&repository, 1).unwrap().unwrap().payload,
        b"cancelled-ticket"
    );
    assert_eq!(observe_handoff(&fixture.pool).unwrap().retained_bytes, 0);
}
