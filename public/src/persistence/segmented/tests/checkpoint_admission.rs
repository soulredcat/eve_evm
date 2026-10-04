// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{checkpoints::*, *};
use super::{
    checkpoint_fixtures::{metadata, record, version},
    fixtures::{batch, fixture},
    pause,
};
use std::time::Duration;

#[test]
fn checkpoint_rejects_foreign_pool_wrong_cursor_and_active_batch() {
    let fixture = fixture();
    let other = super::fixtures::fixture();
    let foreign = record(&other.pool, fixture.parent, fixture.parent.cursor, 3);
    let worker =
        start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent).unwrap();
    assert_eq!(
        try_submit_checkpoint_base(&worker, fixture.parent.cursor, foreign)
            .err()
            .unwrap()
            .error,
        SegmentedError::ForeignPool
    );
    let local = record(&fixture.pool, fixture.parent, fixture.parent.cursor, 3);
    let mut wrong = fixture.parent.cursor;
    wrong.content_hash = [91; 32];
    assert_eq!(
        try_submit_checkpoint_base(&worker, wrong, local.clone())
            .err()
            .unwrap()
            .error,
        SegmentedError::WrongCursor
    );
    let (entered, resume) = pause(&worker, 0, false);
    let sealed = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"existing",
    );
    let batch_ticket = try_submit_segmented_batch(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    entered.recv_timeout(Duration::from_secs(20)).unwrap();
    assert_eq!(
        try_submit_checkpoint_base(&worker, fixture.parent.cursor, local)
            .err()
            .unwrap()
            .error,
        SegmentedError::QueueFull
    );
    resume.send(()).unwrap();
    super::checkpoint_fixtures::wait_batch(&batch_ticket);
    assert!(finish_segmented_worker(worker).repository.is_ok());
}
#[test]
fn checkpoint_blocks_batches_duplicates_and_releases_only_after_exact_ack() {
    let fixture = fixture();
    let worker =
        start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent).unwrap();
    let sealed = record(&fixture.pool, fixture.parent, fixture.parent.cursor, 3);
    let cursor = checkpoint_record_cursor(&sealed);
    let (entered, resume) = pause(&worker, 0, false);
    let ticket = try_submit_checkpoint_base(&worker, fixture.parent.cursor, sealed.clone())
        .ok()
        .unwrap();
    entered.recv_timeout(Duration::from_secs(20)).unwrap();
    assert_eq!(
        try_submit_checkpoint_base(&worker, fixture.parent.cursor, sealed)
            .err()
            .unwrap()
            .error,
        SegmentedError::AlreadySubmitted
    );
    let parent = eve_storage::records::segmented::SegmentedRecoveryAnchor {
        height: 3,
        cursor,
        state_binding: [5; 32],
    };
    let next = batch(&fixture.pool, parent, cursor, b"blocked");
    assert_eq!(
        try_submit_segmented_batch(&worker, cursor, next)
            .err()
            .unwrap()
            .error,
        SegmentedError::QueueFull
    );
    assert_eq!(try_receive_checkpoint_ack(&ticket), Ok(None));
    resume.send(()).unwrap();
    super::checkpoint_fixtures::wait_checkpoint(&ticket);
    assert!(worker.state.admission.lock().unwrap().checkpoint.is_none());
    assert!(finish_segmented_worker(worker).checkpoint_tail.is_none());
}
#[test]
fn checkpoint_limit_or_actual_metadata_capacity_failure_does_not_leak_charge() {
    let fixture = fixture();
    let before = observe_segmented_parts(&fixture.pool)
        .unwrap()
        .estimated_metadata_bytes;
    let invalid = eve_storage::records::segmented::checkpoints::CheckpointBaseLimits {
        maximum_payload_bytes: 1,
    };
    assert!(
        seal_checkpoint_base_record(
            &fixture.pool,
            metadata(fixture.parent, fixture.parent.cursor, 3),
            &version(3),
            invalid
        )
        .is_err()
    );
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .estimated_metadata_bytes,
        before
    );
    let mut accounting = fixture.pool.accounting.lock().unwrap();
    let original = accounting.metadata;
    accounting.metadata = fixture.pool.policy.maximum_metadata_bytes;
    drop(accounting);
    assert!(matches!(
        record_capacity_result(&fixture),
        Err(SegmentedError::Capacity)
    ));
    fixture.pool.accounting.lock().unwrap().metadata = original;
}
fn record_capacity_result(
    fixture: &super::fixtures::Fixture,
) -> Result<SealedCheckpointRecord, SegmentedError> {
    seal_checkpoint_base_record(
        &fixture.pool,
        metadata(fixture.parent, fixture.parent.cursor, 3),
        &version(3),
        super::checkpoint_fixtures::limits(),
    )
}
#[test]
fn checkpoint_age_and_wrong_logical_parent_are_rejected_without_io() {
    let fixture = fixture();
    let worker =
        start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent).unwrap();
    let mut stale = record(&fixture.pool, fixture.parent, fixture.parent.cursor, 3);
    std::sync::Arc::get_mut(&mut stale.0).unwrap().created -= Duration::from_secs(3);
    assert_eq!(
        try_submit_checkpoint_base(&worker, fixture.parent.cursor, stale)
            .err()
            .unwrap()
            .error,
        SegmentedError::QueueAged
    );
    let mut wrong_parent = fixture.parent;
    wrong_parent.state_binding = [93; 32];
    let mismatched = record(&fixture.pool, wrong_parent, fixture.parent.cursor, 3);
    assert_eq!(
        try_submit_checkpoint_base(&worker, fixture.parent.cursor, mismatched)
            .err()
            .unwrap()
            .error,
        SegmentedError::WrongCursor
    );
    assert!(finish_segmented_worker(worker).checkpoint_tail.is_none());
}
