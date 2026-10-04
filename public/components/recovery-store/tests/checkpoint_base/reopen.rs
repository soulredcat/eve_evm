// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::*;
use eve_storage::records::segmented::{SegmentedRecoveryAnchor, checkpoints::*};

#[test]
fn actual_synced_base_and_nonzero_logical_parent_survive_wal_reopen() {
    let directory = tempfile::tempdir().unwrap();
    let mut repository = open(directory.path());
    let parent = bootstrap(&repository);
    let target = prepared(4);
    let first = metadata(parent, parent.cursor, 4);
    let first_cursor = append(&mut repository, encode(first, &target));
    let logical = SegmentedRecoveryAnchor {
        height: 4,
        cursor: first_cursor,
        state_binding: first.target_state_binding,
    };
    let physical = append(
        &mut repository,
        b"retained orphan after first base".to_vec(),
    );
    let target = prepared(9);
    let second = metadata(logical, physical, 9);
    let second_cursor = append(&mut repository, encode(second, &target));
    drop(repository);
    let reopened = open(directory.path());
    let actual = read_checkpoint_base_membership(
        &reopened,
        second_cursor,
        second,
        &target,
        &limits(),
        membership_charge(&reopened),
    )
    .unwrap();
    let record = checkpoint_base_membership_record(&actual);
    assert_eq!(record.sequence, second_cursor.sequence);
    assert_eq!(record.content_hash, second_cursor.content_hash);
    assert_eq!(record.parent, physical);
    assert_eq!(checkpoint_base_membership_view(&actual).metadata, second);
    assert_eq!(
        checkpoint_base_membership_view(&actual).target_version_bytes,
        prepared_checkpoint_base_target_bytes(&target)
    );
    assert_eq!(
        checkpoint_base_membership_target_bytes(&actual),
        prepared_checkpoint_base_target_bytes(&target)
    );
    assert_eq!(prepared_checkpoint_base_target_height(&target), 9);
    assert_eq!(
        prepared_checkpoint_base_target_security_profile(&target),
        checkpoint_base_membership_view(&actual).security_profile
    );
    assert!(record.sequence > 0);
}
