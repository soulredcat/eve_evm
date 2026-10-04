// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::*;
use eve_storage::records::{
    OpaqueRecordCursor, opaque_record_bootstrap_cursor, open_opaque_record_repository,
    segmented::checkpoints::*,
};

#[test]
fn bootstrap_zero_and_missing_or_foreign_cursors_cannot_assert_height_h_membership() {
    let directory = tempfile::tempdir().unwrap();
    let mut repository = open(directory.path());
    let parent = bootstrap(&repository);
    let target = prepared(4);
    let expected = metadata(parent, parent.cursor, 4);
    let cursor = append(&mut repository, encode(expected, &target));
    let charge = membership_charge(&repository);
    assert!(matches!(
        read_checkpoint_base_membership(
            &repository,
            parent.cursor,
            expected,
            &target,
            &limits(),
            charge
        ),
        Err(CheckpointBaseError::MembershipMismatch)
    ));
    let mut foreign = cursor;
    foreign.content_hash[0] ^= 1;
    assert!(matches!(
        read_checkpoint_base_membership(&repository, foreign, expected, &target, &limits(), charge),
        Err(CheckpointBaseError::MembershipMismatch)
    ));
    let missing = OpaqueRecordCursor {
        sequence: 2,
        content_hash: [5; 32],
    };
    let mut missing_expected = expected;
    missing_expected.previous_opaque_cursor = cursor;
    assert!(matches!(
        read_checkpoint_base_membership(
            &repository,
            missing,
            missing_expected,
            &target,
            &limits(),
            charge
        ),
        Err(CheckpointBaseError::MissingRecord)
    ));
    let foreign_directory = tempfile::tempdir().unwrap();
    let mut foreign_identity = identity();
    foreign_identity.domain = [19; 32];
    let foreign_repository = open_opaque_record_repository(
        &foreign_directory.path().join("store"),
        foreign_identity,
        budget(),
    )
    .unwrap();
    assert_ne!(
        opaque_record_bootstrap_cursor(&foreign_repository),
        parent.cursor
    );
    assert!(matches!(
        read_checkpoint_base_membership(
            &foreign_repository,
            cursor,
            expected,
            &target,
            &limits(),
            charge
        ),
        Err(CheckpointBaseError::MembershipMismatch)
    ));
}

#[test]
fn actual_parent_and_all_expected_base_fields_must_match_not_only_height_binding() {
    let directory = tempfile::tempdir().unwrap();
    let mut repository = open(directory.path());
    let parent = bootstrap(&repository);
    let target = prepared(4);
    let expected = metadata(parent, parent.cursor, 4);
    let cursor = append(&mut repository, encode(expected, &target));
    let charge = membership_charge(&repository);
    let mut wrong = expected;
    wrong.snapshot_body_hash[0] ^= 1;
    assert!(matches!(
        read_checkpoint_base_membership(&repository, cursor, wrong, &target, &limits(), charge),
        Err(CheckpointBaseError::MembershipMismatch)
    ));
    wrong = expected;
    wrong.previous_opaque_cursor.content_hash[0] ^= 1;
    assert!(matches!(
        read_checkpoint_base_membership(&repository, cursor, wrong, &target, &limits(), charge),
        Err(CheckpointBaseError::MembershipMismatch)
    ));
    let mut next = expected;
    next.previous_opaque_cursor = cursor;
    let mut payload = encode(next, &target);
    payload[36] ^= 1;
    rehash(&mut payload);
    let bad_cursor = append(&mut repository, payload);
    assert!(matches!(
        read_checkpoint_base_membership(&repository, bad_cursor, next, &target, &limits(), charge),
        Err(CheckpointBaseError::ParentMismatch)
    ));
}

#[test]
fn malformed_actual_payload_rejects_without_replacing_previous_complete_base() {
    let directory = tempfile::tempdir().unwrap();
    let mut repository = open(directory.path());
    let parent = bootstrap(&repository);
    let target = prepared(4);
    let expected = metadata(parent, parent.cursor, 4);
    let cursor = append(&mut repository, encode(expected, &target));
    let mut next = expected;
    next.previous_opaque_cursor = cursor;
    let mut malformed = encode(next, &target);
    malformed.push(0);
    let bad_cursor = append(&mut repository, malformed);
    assert!(matches!(
        read_checkpoint_base_membership(
            &repository,
            bad_cursor,
            next,
            &target,
            &limits(),
            membership_charge(&repository)
        ),
        Err(CheckpointBaseError::MalformedEncoding)
    ));
    drop(repository);
    let reopened = open(directory.path());
    let actual = read_checkpoint_base_membership(
        &reopened,
        cursor,
        expected,
        &target,
        &limits(),
        membership_charge(&reopened),
    )
    .unwrap();
    assert_eq!(checkpoint_base_membership_view(&actual).metadata, expected);
}
