// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{checkpoints::*, *};
use super::{
    checkpoint_fixtures::{record, wait_batch, wait_checkpoint},
    fixtures::{batch, fixture},
};
use eve_storage::records::{
    opaque_record_cursor, open_opaque_record_repository, read_opaque_record,
    segmented::{
        SegmentedRecoveryAnchor,
        checkpoints::{
            checkpoint_base_view, preflight_checkpoint_base, prepare_checkpoint_base_target,
            required_checkpoint_base_encoding_reservation,
        },
    },
};

#[test]
fn batch_checkpoint_batch_uses_same_wal_and_reopens_exact_physical_prefix() {
    let fixture = fixture();
    let first = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"before-checkpoint",
    );
    let worker =
        start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent).unwrap();
    let first_ticket = try_submit_segmented_batch(&worker, fixture.parent.cursor, first)
        .ok()
        .unwrap();
    let first_ack = wait_batch(&first_ticket);
    let parent = SegmentedRecoveryAnchor {
        height: 1,
        cursor: first_ack.marker_cursor,
        state_binding: first_ack.target_state_binding,
    };
    let sealed = record(&fixture.pool, parent, first_ack.marker_cursor, 7);
    let expected = checkpoint_record_cursor(&sealed);
    let metadata = checkpoint_record_metadata(&sealed);
    let ticket = try_submit_checkpoint_base(&worker, first_ack.marker_cursor, sealed)
        .ok()
        .unwrap();
    let ack = wait_checkpoint(&ticket);
    assert_eq!(ack.cursor, expected);
    assert_eq!(ack.metadata, metadata);
    assert!(ack.database_sequence > first_ack.database_sequence);
    let checkpoint_parent = SegmentedRecoveryAnchor {
        height: 7,
        cursor: ack.cursor,
        state_binding: ack.metadata.target_state_binding,
    };
    let last = batch(
        &fixture.pool,
        checkpoint_parent,
        ack.cursor,
        b"after-checkpoint",
    );
    let last_ticket = try_submit_segmented_batch(&worker, ack.cursor, last)
        .ok()
        .unwrap();
    let last_ack = wait_batch(&last_ticket);
    assert_eq!(last_ack.identity.parent, checkpoint_parent);
    assert_eq!(last_ack.identity.target_height, 8);
    let shutdown = finish_segmented_worker(worker);
    assert!(shutdown.checkpoint_tail.is_none());
    assert!(shutdown.tails.iter().all(Option::is_none));
    let repository = shutdown.repository.unwrap();
    let row = read_opaque_record(&repository, ack.cursor.sequence)
        .unwrap()
        .unwrap();
    assert_eq!(row.parent, first_ack.marker_cursor);
    assert_eq!(row.content_hash, expected.content_hash);
    let limits = super::checkpoint_fixtures::limits();
    let target = prepare_checkpoint_base_target(
        &super::checkpoint_fixtures::version(7),
        &limits,
        required_checkpoint_base_encoding_reservation(&limits).unwrap(),
    )
    .unwrap();
    let checked =
        preflight_checkpoint_base(&row.payload, &target, row.parent, parent, &limits).unwrap();
    assert_eq!(checkpoint_base_view(&checked).metadata, metadata);
    drop(repository);
    let reopened =
        open_opaque_record_repository(&fixture.path, fixture.namespace, fixture.budget).unwrap();
    assert_eq!(
        opaque_record_cursor(&reopened).unwrap(),
        last_ack.marker_cursor
    );
}
