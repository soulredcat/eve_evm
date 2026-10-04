// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::*;
use super::{
    fixtures::{batch, fixture, fixture_with_retention},
    pause,
};
use eve_storage::records::{
    opaque_record_cursor, open_opaque_record_repository, read_opaque_record,
};
use std::time::Duration;
#[test]
fn dropped_ticket_and_worker_panic_preserve_all_parts_and_return_the_unsynced_marker_tail() {
    let fixture = fixture();
    let bytes = vec![0x32; fixture.pool.codec.maximum_chunk_bytes + 17];
    let sealed = batch(&fixture.pool, fixture.parent, fixture.parent.cursor, &bytes);
    let worker = start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("valid worker"));
    let (entered, resume) = pause(&worker, 1, true);
    let ticket = try_submit_segmented_batch(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    entered.recv_timeout(Duration::from_secs(20)).unwrap();
    drop(ticket);
    let retained = observe_segmented_parts(&fixture.pool).unwrap();
    resume.send(()).unwrap();
    let shutdown = finish_segmented_worker(worker);
    assert!(matches!(
        shutdown.repository,
        Err(SegmentedError::WorkerPanicked)
    ));
    let tail = shutdown.tails[0].as_ref().unwrap();
    assert_eq!(tail.last_acknowledged_physical_cursor.sequence, 1);
    assert!(tail.complete_marker.is_none());
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_encoded_bytes,
        retained.retained_encoded_bytes
    );
    let repository =
        open_opaque_record_repository(&fixture.path, fixture.namespace, fixture.budget).unwrap();
    assert_eq!(opaque_record_cursor(&repository).unwrap().sequence, 1);
    assert!(read_opaque_record(&repository, 2).unwrap().is_none());
    drop(shutdown);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        0
    );
}
#[test]
fn cancelled_complete_ticket_keeps_full_charged_data_until_explicit_shutdown_reconciliation() {
    let fixture = fixture();
    let sealed = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"small-complete-record",
    );
    let worker = start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("valid worker"));
    let ticket = try_submit_segmented_batch(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    drop(ticket);
    let shutdown = finish_segmented_worker(worker);
    assert!(shutdown.repository.is_ok());
    assert!(
        shutdown.tails[0]
            .as_ref()
            .unwrap()
            .complete_marker
            .is_some()
    );
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        2
    );
    drop(shutdown);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        0
    );
}

#[test]
fn real_repository_capacity_refusal_preserves_cancelled_full_tail_and_no_marker() {
    let fixture = fixture_with_retention(Some(1));
    let sealed = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"capacity-refusal",
    );
    let worker = start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("valid worker"));
    let ticket = try_submit_segmented_batch(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    drop(ticket);
    let shutdown = finish_segmented_worker(worker);
    assert!(matches!(
        shutdown.repository,
        Err(SegmentedError::StorageFailed)
    ));
    let tail = shutdown.tails[0].as_ref().unwrap();
    assert_eq!(tail.last_acknowledged_physical_cursor.sequence, 1);
    assert!(tail.complete_marker.is_none());
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        2
    );
    let repository =
        open_opaque_record_repository(&fixture.path, fixture.namespace, fixture.budget).unwrap();
    assert_eq!(opaque_record_cursor(&repository).unwrap().sequence, 1);
    assert!(read_opaque_record(&repository, 2).unwrap().is_none());
    drop(shutdown);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        0
    );
}
