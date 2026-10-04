// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{checkpoints::*, *};
use super::{
    checkpoint_fixtures::{limits, record, version, wait_checkpoint},
    fixtures::fixture,
};
use eve_storage::records::{
    open_opaque_record_repository,
    segmented::{
        SegmentedRecoveryAnchor,
        checkpoints::{
            checkpoint_base_membership_view, prepare_checkpoint_base_target,
            read_checkpoint_base_membership, required_checkpoint_base_encoding_reservation,
            required_checkpoint_base_membership_reservation,
        },
    },
};

#[test]
fn legacy_worker_rejects_checkpoint_and_explicit_constructor_rereads_same_repository() {
    let fixture = fixture();
    let worker =
        start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent).unwrap();
    let sealed = record(&fixture.pool, fixture.parent, fixture.parent.cursor, 3);
    let metadata = checkpoint_record_metadata(&sealed);
    let ticket = try_submit_checkpoint_base(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    let ack = wait_checkpoint(&ticket);
    drop(ticket);
    let repository = finish_segmented_worker(worker).repository.unwrap();
    let target = prepare_checkpoint_base_target(
        &version(3),
        &limits(),
        required_checkpoint_base_encoding_reservation(&limits()).unwrap(),
    )
    .unwrap();
    let membership = read_checkpoint_base_membership(
        &repository,
        ack.cursor,
        metadata,
        &target,
        &limits(),
        required_checkpoint_base_membership_reservation(&fixture.budget, &limits()).unwrap(),
    )
    .unwrap();
    let parent = SegmentedRecoveryAnchor {
        height: 3,
        cursor: ack.cursor,
        state_binding: [5; 32],
    };
    assert!(matches!(
        start_segmented_worker(repository, fixture.pool.clone(), parent),
        Err(SegmentedError::Codec)
    ));
    let repository =
        open_opaque_record_repository(&fixture.path, fixture.namespace, fixture.budget).unwrap();
    let started = start_segmented_worker_from_checkpoint(
        repository,
        fixture.pool.clone(),
        &membership,
        &target,
        limits(),
    )
    .unwrap();
    assert_eq!(
        checkpoint_base_membership_view(&membership).metadata,
        metadata
    );
    assert!(finish_segmented_worker(started).repository.is_ok());
    let foreign = super::fixtures::fixture();
    // Same configured namespace and budgets are insufficient: the row must exist in this store.
    assert!(matches!(
        start_segmented_worker_from_checkpoint(
            foreign.repository,
            foreign.pool.clone(),
            &membership,
            &target,
            limits()
        ),
        Err(SegmentedError::WrongCursor)
    ));
}
#[test]
fn explicit_checkpoint_start_rejects_different_prepared_target() {
    let fixture = fixture();
    let worker =
        start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent).unwrap();
    let sealed = record(&fixture.pool, fixture.parent, fixture.parent.cursor, 3);
    let metadata = checkpoint_record_metadata(&sealed);
    let ticket = try_submit_checkpoint_base(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    let ack = wait_checkpoint(&ticket);
    drop(ticket);
    let repository = finish_segmented_worker(worker).repository.unwrap();
    let required = required_checkpoint_base_encoding_reservation(&limits()).unwrap();
    let target = prepare_checkpoint_base_target(&version(3), &limits(), required).unwrap();
    let membership = read_checkpoint_base_membership(
        &repository,
        ack.cursor,
        metadata,
        &target,
        &limits(),
        required_checkpoint_base_membership_reservation(&fixture.budget, &limits()).unwrap(),
    )
    .unwrap();
    let wrong = prepare_checkpoint_base_target(&version(4), &limits(), required).unwrap();
    assert!(matches!(
        start_segmented_worker_from_checkpoint(
            repository,
            fixture.pool.clone(),
            &membership,
            &wrong,
            limits()
        ),
        Err(SegmentedError::WrongCursor)
    ));
}
