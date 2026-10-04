// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::*;
use super::{
    fixtures::{batch, fixture},
    pause,
};
use eve_storage::records::segmented::SegmentedRecoveryAnchor;
use eve_storage::records::{
    compare_and_append_opaque_records, opaque_record_cursor, open_opaque_record_repository,
};
use std::time::Duration;
#[test]
fn two_small_logical_batches_admit_while_first_sync_is_paused_without_aliasing_physical_heights() {
    let fixture = fixture();
    let first = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"first",
    );
    let next = SegmentedRecoveryAnchor {
        height: 1,
        cursor: first.0.marker_cursor,
        state_binding: [8; 32],
    };
    let second = batch(&fixture.pool, next, next.cursor, b"second");
    let worker = start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("valid worker"));
    let (entered, resume) = pause(&worker, 0, false);
    let one = try_submit_segmented_batch(&worker, fixture.parent.cursor, first)
        .ok()
        .unwrap();
    entered.recv_timeout(Duration::from_secs(5)).unwrap();
    let two = try_submit_segmented_batch(&worker, next.cursor, second)
        .ok()
        .unwrap();
    let pending = (
        try_receive_segmented_ack(&one),
        try_receive_segmented_ack(&two),
    );
    resume.send(()).unwrap();
    assert_eq!(pending, (Ok(None), Ok(None)));
    let shutdown = finish_segmented_worker(worker);
    let first = try_receive_segmented_ack(&one).unwrap().unwrap();
    let second = try_receive_segmented_ack(&two).unwrap().unwrap();
    assert_eq!(
        (first.identity.target_height, first.marker_cursor.sequence),
        (1, 2)
    );
    assert_eq!(
        (second.identity.target_height, second.marker_cursor.sequence),
        (2, 4)
    );
    assert!(second.database_sequence > first.database_sequence);
    drop(shutdown);
}

#[test]
fn known_orphan_segments_do_not_change_logical_parent_and_nonzero_parent_requires_real_marker() {
    let mut fixture = fixture();
    let orphan = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"orphan",
    );
    let bytes = &orphan.0.parts[0].as_ref().unwrap().buffers[0];
    let orphan_ack = compare_and_append_opaque_records(
        &mut fixture.repository,
        fixture.parent.cursor,
        std::slice::from_ref(bytes),
    )
    .unwrap();
    drop(orphan);
    let retry = batch(&fixture.pool, fixture.parent, orphan_ack.appended, b"retry");
    let worker = start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("known orphan head"));
    let ticket = try_submit_segmented_batch(&worker, orphan_ack.appended, retry)
        .ok()
        .unwrap();
    let shutdown = finish_segmented_worker(worker);
    let ack = try_receive_segmented_ack(&ticket).unwrap().unwrap();
    assert_eq!(
        (
            ack.identity.target_height,
            ack.references[0].sequence,
            ack.marker_cursor.sequence
        ),
        (1, 2, 3)
    );
    let repository = shutdown.repository.unwrap();
    let parent = SegmentedRecoveryAnchor {
        height: 1,
        cursor: ack.marker_cursor,
        state_binding: ack.target_state_binding,
    };
    drop(ticket);
    let worker = start_segmented_worker(repository, fixture.pool.clone(), parent)
        .unwrap_or_else(|_| panic!("actual complete marker"));
    let next = finish_segmented_worker(worker);
    drop(next.repository.unwrap());
    let repository =
        open_opaque_record_repository(&fixture.path, fixture.namespace, fixture.budget).unwrap();
    assert_eq!(
        opaque_record_cursor(&repository).unwrap(),
        ack.marker_cursor
    );
    let mut wrong = parent;
    wrong.state_binding[0] ^= 1;
    assert!(matches!(
        start_segmented_worker(repository, fixture.pool.clone(), wrong),
        Err(SegmentedError::WrongCursor)
    ));
}
