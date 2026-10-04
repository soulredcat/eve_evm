// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{checkpoints::*, *};
use super::{
    checkpoint_fixtures::record,
    fixtures::{fixture, fixture_with_retention},
    pause,
};
use eve_storage::records::{
    opaque_record_cursor, open_opaque_record_repository, read_opaque_record,
};
use std::time::Duration;

#[test]
fn cancelled_checkpoint_ticket_retains_full_charge_and_complete_tail_until_drop() {
    let fixture = fixture();
    let baseline = observe_segmented_parts(&fixture.pool)
        .unwrap()
        .estimated_metadata_bytes;
    let worker =
        start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent).unwrap();
    let sealed = record(&fixture.pool, fixture.parent, fixture.parent.cursor, 3);
    let expected = checkpoint_record_cursor(&sealed);
    let ticket = try_submit_checkpoint_base(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    let charged = observe_segmented_parts(&fixture.pool)
        .unwrap()
        .estimated_metadata_bytes;
    drop(ticket);
    let shutdown = finish_segmented_worker(worker);
    let tail = shutdown.checkpoint_tail.as_ref().unwrap();
    assert_eq!(tail.complete.unwrap().cursor, expected);
    assert_eq!(tail.last_acknowledged_physical_cursor, expected);
    assert!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .estimated_metadata_bytes
            > baseline
    );
    assert!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .estimated_metadata_bytes
            <= charged
    );
    assert_eq!(
        opaque_record_cursor(shutdown.repository.as_ref().unwrap()).unwrap(),
        expected
    );
    drop(shutdown);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .estimated_metadata_bytes,
        baseline
    );
}
#[test]
fn checkpoint_worker_panic_preserves_unsynced_charged_tail_and_fences_io() {
    let fixture = fixture();
    let baseline = observe_segmented_parts(&fixture.pool)
        .unwrap()
        .estimated_metadata_bytes;
    let worker =
        start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent).unwrap();
    let sealed = record(&fixture.pool, fixture.parent, fixture.parent.cursor, 3);
    let (entered, resume) = pause(&worker, 0, true);
    let ticket = try_submit_checkpoint_base(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    entered.recv_timeout(Duration::from_secs(20)).unwrap();
    drop(ticket);
    resume.send(()).unwrap();
    let shutdown = finish_segmented_worker(worker);
    assert!(matches!(
        shutdown.repository,
        Err(SegmentedError::WorkerPanicked)
    ));
    let tail = shutdown.checkpoint_tail.as_ref().unwrap();
    assert_eq!(
        tail.last_acknowledged_physical_cursor,
        fixture.parent.cursor
    );
    assert!(tail.complete.is_none());
    assert!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .estimated_metadata_bytes
            > baseline
    );
    let reopened =
        open_opaque_record_repository(&fixture.path, fixture.namespace, fixture.budget).unwrap();
    assert_eq!(
        opaque_record_cursor(&reopened).unwrap(),
        fixture.parent.cursor
    );
    assert!(read_opaque_record(&reopened, 1).unwrap().is_none());
    drop(shutdown);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .estimated_metadata_bytes,
        baseline
    );
}
#[test]
fn actual_repository_capacity_failure_preserves_checkpoint_tail_and_latches_failure() {
    let fixture = fixture_with_retention(Some(1));
    let worker =
        start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent).unwrap();
    let first = record(&fixture.pool, fixture.parent, fixture.parent.cursor, 3);
    let first_ticket = try_submit_checkpoint_base(&worker, fixture.parent.cursor, first)
        .ok()
        .unwrap();
    let ack = super::checkpoint_fixtures::wait_checkpoint(&first_ticket);
    let parent = eve_storage::records::segmented::SegmentedRecoveryAnchor {
        height: 3,
        cursor: ack.cursor,
        state_binding: ack.metadata.target_state_binding,
    };
    let second = record(&fixture.pool, parent, ack.cursor, 4);
    let second_ticket = try_submit_checkpoint_base(&worker, ack.cursor, second)
        .ok()
        .unwrap();
    drop(first_ticket);
    drop(second_ticket);
    let shutdown = finish_segmented_worker(worker);
    assert!(matches!(
        shutdown.repository,
        Err(SegmentedError::StorageFailed)
    ));
    let tail = shutdown.checkpoint_tail.as_ref().unwrap();
    assert!(tail.complete.is_none());
    assert_eq!(tail.last_acknowledged_physical_cursor, ack.cursor);
    let reopened =
        open_opaque_record_repository(&fixture.path, fixture.namespace, fixture.budget).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), ack.cursor);
    assert!(read_opaque_record(&reopened, 2).unwrap().is_none());
}
#[test]
fn mismatched_checkpoint_ack_keeps_retained_record_and_latches_failure() {
    let fixture = fixture();
    let worker =
        start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent).unwrap();
    let sealed = record(&fixture.pool, fixture.parent, fixture.parent.cursor, 3);
    let ticket = try_submit_checkpoint_base(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    let shutdown = finish_segmented_worker(worker);
    {
        let mut admission = ticket.state.admission.lock().unwrap();
        let complete = admission
            .checkpoint
            .as_mut()
            .unwrap()
            .complete
            .as_mut()
            .unwrap();
        complete.metadata.target_state_binding = [94; 32];
    }
    assert_eq!(
        try_receive_checkpoint_ack(&ticket),
        Err(SegmentedError::AckMismatch)
    );
    assert!(
        ticket
            .state
            .failed
            .load(std::sync::atomic::Ordering::Acquire)
    );
    assert!(ticket.state.admission.lock().unwrap().checkpoint.is_some());
    drop(shutdown);
    drop(ticket);
}
