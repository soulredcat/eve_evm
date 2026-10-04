// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::*;
use super::fixtures::fixture;
use eve_storage::records::segmented::{
    SegmentedLogicalIdentity, SegmentedRecoveryMode, hash_segmented_logical_body,
    preflight_segmented_record, segmented_marker_view,
};

#[test]
fn unbound_capacity_is_held_and_bad_binding_returns_the_same_owned_reservation() {
    let fixture = fixture();
    let bytes = b"reserve before authenticated candidate preparation";
    let identity = SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: hash_segmented_logical_body(bytes),
        parent: fixture.parent,
        target_height: 1,
        total_length: bytes.len() as u64,
    };
    let layout = plan_segmented_layout(&fixture.pool, identity).unwrap();
    let held = reserve_segmented_layout(&fixture.pool, &layout).unwrap();
    let observation = observe_segmented_parts(&fixture.pool).unwrap();
    assert_eq!(observation.retained_parts, 2);
    let rejected = bind_segmented_reservation(held, [0; 32]).err().unwrap();
    assert_eq!(rejected.error, SegmentedError::InvalidPlan);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_encoded_bytes,
        observation.retained_encoded_bytes
    );
    let bound = bind_segmented_reservation(rejected.reservation, [9; 32])
        .unwrap_or_else(|_| panic!("valid nonzero binding"));
    let batch = write_and_seal_segmented_batch(bound, bytes, fixture.parent.cursor).unwrap();
    assert_eq!(segmented_batch_identity(&batch), identity);
    assert_eq!(segmented_batch_target_binding(&batch), [9; 32]);
    assert_eq!(
        segmented_batch_expected_cursor(&batch),
        fixture.parent.cursor
    );
    assert_eq!(segmented_batch_references(&batch).len(), 1);
    assert_eq!(segmented_batch_marker_cursor(&batch).sequence, 2);
    let marker_bytes = segmented_part_bytes(&batch, 1, 0).unwrap();
    let preflight = preflight_segmented_record(marker_bytes, &fixture.pool.codec).unwrap();
    assert_eq!(
        segmented_marker_view(&preflight)
            .unwrap()
            .metadata
            .target_state_binding,
        [9; 32]
    );
    drop(batch);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        0
    );
}

#[test]
fn maximum_unbound_layout_reserves_all_parts_atomically_without_target_binding() {
    let fixture = fixture();
    let identity = SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: [7; 32],
        parent: fixture.parent,
        target_height: 1,
        total_length: 21_025_569,
    };
    let layout = plan_segmented_layout(&fixture.pool, identity).unwrap();
    let reservation = reserve_segmented_layout(&fixture.pool, &layout).unwrap();
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        4
    );
    assert!(matches!(
        reserve_segmented_layout(&fixture.pool, &layout),
        Err(SegmentedError::Capacity)
    ));
    drop(reservation);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        0
    );
}
#[test]
fn same_length_substitution_is_rejected_before_any_part_is_sealed() {
    let fixture = fixture();
    let bytes = b"expected-body";
    let identity = SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: hash_segmented_logical_body(bytes),
        parent: fixture.parent,
        target_height: 1,
        total_length: bytes.len() as u64,
    };
    let plan = plan_segmented_batch(&fixture.pool, identity, [8; 32]).unwrap();
    let held = reserve_segmented_batch(&fixture.pool, &plan).unwrap();
    assert!(matches!(
        write_and_seal_segmented_batch(held, b"changed--body", fixture.parent.cursor),
        Err(SegmentedError::InvalidPlan)
    ));
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        0
    );
}
