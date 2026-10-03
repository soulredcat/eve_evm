// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#![cfg(test)]

use super::{fixture, install_pause, payload};
use crate::persistence::{handoff::observe_handoff, worker::*};
use eve_storage::records::*;
use std::{
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

#[test]
fn nonblocking_ack_keeps_two_paused_admissions_ordered_through_sync_and_reopen() {
    let fixture = fixture();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    let first_cursor = prospective_opaque_record_cursor(
        fixture.identity,
        parent,
        b"record-one",
        fixture.budget.maximum_record_bytes,
    )
    .unwrap();
    let second_cursor = prospective_opaque_record_cursor(
        fixture.identity,
        first_cursor,
        b"record-two",
        fixture.budget.maximum_record_bytes,
    )
    .unwrap();
    let worker = start_record_worker(
        fixture.repository,
        Arc::clone(&fixture.pool),
        40 * 1_048_576,
    )
    .ok()
    .unwrap();
    let (entered, resume) = install_pause(&worker);
    let first_ticket = try_submit_record(&worker, parent, payload(&fixture.pool, b"record-one"))
        .ok()
        .unwrap();
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    let second_ticket =
        try_submit_record(&worker, first_cursor, payload(&fixture.pool, b"record-two"))
            .ok()
            .unwrap();
    // A separate poller lets an accidental blocking receive fail within a bounded interval.
    // Resume before asserting the deadline so a failure cannot leave the writer paused.
    let (polled_sender, polled_receiver) = mpsc::sync_channel(1);
    let poller = thread::spawn(move || {
        let pending = (
            try_receive_record_ack(&first_ticket),
            try_receive_record_ack(&second_ticket),
        );
        polled_sender
            .send((first_ticket, second_ticket, pending))
            .unwrap();
    });
    let immediate = polled_receiver.recv_timeout(Duration::from_secs(1));
    let retained = observe_handoff(&fixture.pool);
    resume.send(()).unwrap();
    poller.join().unwrap();
    let repository = finish_record_worker(worker).ok().unwrap();
    let (first_ticket, second_ticket, pending) = immediate.expect("poll must finish before sync");
    assert_eq!(pending, (Ok(None), Ok(None)));
    let retained = retained.unwrap();
    assert_eq!(retained.retained_batches, 2);
    assert_eq!(retained.retained_bytes, 20);
    let first = try_receive_record_ack(&first_ticket).unwrap().unwrap();
    let second = try_receive_record_ack(&second_ticket).unwrap().unwrap();
    assert_eq!(first.disposition, OpaqueRecordDisposition::NewlySynced);
    assert_eq!(second.disposition, OpaqueRecordDisposition::NewlySynced);
    assert_eq!(first.appended, first_cursor);
    assert_eq!(first.store_head, first_cursor);
    assert_eq!(second.appended, second_cursor);
    assert_eq!(second.store_head, second_cursor);
    assert!(second.database_sequence > first.database_sequence);
    assert_eq!(opaque_record_cursor(&repository).unwrap(), second_cursor);
    drop(repository);
    let reopened =
        open_opaque_record_repository(&fixture.path, fixture.identity, fixture.budget).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), second_cursor);
    assert_eq!(
        read_opaque_record(&reopened, 1).unwrap().unwrap().payload,
        b"record-one"
    );
    assert_eq!(
        read_opaque_record(&reopened, 2).unwrap().unwrap().payload,
        b"record-two"
    );
    assert_eq!(observe_handoff(&fixture.pool).unwrap().retained_bytes, 0);
}

#[test]
fn nonblocking_ack_propagates_real_storage_failure_without_advancing_prefix() {
    let fixture = fixture();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    let worker = start_record_worker(
        fixture.repository,
        Arc::clone(&fixture.pool),
        40 * 1_048_576,
    )
    .ok()
    .unwrap();
    let bad_parent = OpaqueRecordCursor {
        sequence: parent.sequence,
        content_hash: [99; 32],
    };
    let ticket = try_submit_record(&worker, bad_parent, payload(&fixture.pool, b"bad-parent"))
        .ok()
        .unwrap();
    assert!(matches!(
        finish_record_worker(worker),
        Err(RecordWorkerError::StorageFailed)
    ));
    assert_eq!(
        try_receive_record_ack(&ticket),
        Err(RecordWorkerError::StorageFailed)
    );
    let reopened =
        open_opaque_record_repository(&fixture.path, fixture.identity, fixture.budget).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), parent);
    assert!(read_opaque_record(&reopened, 1).unwrap().is_none());
}

#[test]
fn nonblocking_ack_reports_lost_acknowledgement_after_sender_disconnects() {
    let (reply, receiver) = mpsc::sync_channel(1);
    let ticket = RecordTicket { receiver };
    drop(reply);
    assert_eq!(
        try_receive_record_ack(&ticket),
        Err(RecordWorkerError::AcknowledgementLost)
    );
}
