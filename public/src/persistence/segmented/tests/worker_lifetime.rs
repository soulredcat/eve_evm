// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::*;
use super::fixtures::{batch, fixture};
use eve_storage::records::segmented::SegmentedRecoveryAnchor;
use eve_storage::records::{opaque_record_cursor, open_opaque_record_repository};

#[test]
fn one_pool_refuses_another_worker_until_finished_handles_and_retained_tickets_release_state() {
    let fixture = fixture();
    let other_path = fixture.path.parent().unwrap().join("other");
    let other =
        open_opaque_record_repository(&other_path, fixture.namespace, fixture.budget).unwrap();
    let worker = start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("first worker"));
    assert!(matches!(
        start_segmented_worker(other, fixture.pool.clone(), fixture.parent),
        Err(SegmentedError::WorkerAlreadyActive)
    ));
    let sealed = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"lifetime",
    );
    let ticket = try_submit_segmented_batch(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    let shutdown = finish_segmented_worker(worker);
    assert!(shutdown.repository.is_ok());
    let other =
        open_opaque_record_repository(&other_path, fixture.namespace, fixture.budget).unwrap();
    assert!(matches!(
        start_segmented_worker(other, fixture.pool.clone(), fixture.parent),
        Err(SegmentedError::WorkerAlreadyActive)
    ));
    assert!(try_receive_segmented_ack(&ticket).unwrap().is_some());
    let other =
        open_opaque_record_repository(&other_path, fixture.namespace, fixture.budget).unwrap();
    assert!(matches!(
        start_segmented_worker(other, fixture.pool.clone(), fixture.parent),
        Err(SegmentedError::WorkerAlreadyActive)
    ));
    drop(ticket);
    // Completed raw tail ownership does not own a worker scratch envelope.
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        2
    );
    let other =
        open_opaque_record_repository(&other_path, fixture.namespace, fixture.budget).unwrap();
    let next = start_segmented_worker(other, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("last state owner released lifetime fence"));
    drop(finish_segmented_worker(next));
    drop(shutdown);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        0
    );
}

#[test]
fn startup_failure_before_state_allocation_releases_the_pool_worker_lifetime() {
    let fixture = fixture();
    let mut wrong = fixture.parent;
    wrong.cursor.content_hash[0] ^= 1;
    assert!(matches!(
        start_segmented_worker(fixture.repository, fixture.pool.clone(), wrong),
        Err(SegmentedError::WrongCursor)
    ));
    let repository =
        open_opaque_record_repository(&fixture.path, fixture.namespace, fixture.budget).unwrap();
    let worker = start_segmented_worker(repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("failed startup released its lease"));
    drop(finish_segmented_worker(worker));
}

#[test]
fn startup_failure_after_state_allocation_and_marker_read_releases_the_pool_worker_lifetime() {
    let fixture = fixture();
    let sealed = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"marker-parent",
    );
    let worker = start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("valid worker"));
    let ticket = try_submit_segmented_batch(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    let shutdown = finish_segmented_worker(worker);
    let ack = try_receive_segmented_ack(&ticket).unwrap().unwrap();
    drop(ticket);
    let parent = SegmentedRecoveryAnchor {
        height: ack.identity.target_height,
        cursor: ack.marker_cursor,
        state_binding: ack.target_state_binding,
    };
    let mut wrong = parent;
    wrong.state_binding[0] ^= 1;
    let repository = shutdown.repository.unwrap();
    assert!(matches!(
        start_segmented_worker(repository, fixture.pool.clone(), wrong),
        Err(SegmentedError::WrongCursor)
    ));
    let repository =
        open_opaque_record_repository(&fixture.path, fixture.namespace, fixture.budget).unwrap();
    assert_eq!(opaque_record_cursor(&repository).unwrap(), parent.cursor);
    let next = start_segmented_worker(repository, fixture.pool.clone(), parent)
        .unwrap_or_else(|_| panic!("marker-read failure released the last state lease"));
    drop(finish_segmented_worker(next));
    drop(shutdown.tails);
}
