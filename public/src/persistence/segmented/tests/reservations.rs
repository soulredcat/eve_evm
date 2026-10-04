// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::*;
use super::fixtures::{batch, fixture};
use eve_storage::records::segmented::{SegmentedLogicalIdentity, SegmentedRecoveryMode};
#[test]
fn all_four_parts_reserve_atomically_before_allocation_and_cancel_as_one_owned_group() {
    let fixture = fixture();
    let identity = SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: [7; 32],
        parent: fixture.parent,
        target_height: 1,
        total_length: 21_025_569,
    };
    let plan = plan_segmented_batch(&fixture.pool, identity, [8; 32]).unwrap();
    let before = observe_segmented_parts(&fixture.pool).unwrap();
    let held = reserve_segmented_batch(&fixture.pool, &plan).unwrap();
    let full = observe_segmented_parts(&fixture.pool).unwrap();
    assert_eq!(full.retained_parts, 4);
    assert_eq!(full.retained_encoded_bytes, 21_027_288);
    assert!(full.estimated_metadata_bytes > before.estimated_metadata_bytes);
    assert!(matches!(
        reserve_segmented_batch(&fixture.pool, &plan),
        Err(SegmentedError::Capacity)
    ));
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_encoded_bytes,
        full.retained_encoded_bytes
    );
    drop(held);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        0
    );
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .estimated_metadata_bytes,
        before.estimated_metadata_bytes
    );
}
#[test]
fn immutable_clones_share_exact_part_buffers_and_release_only_with_the_last_owner() {
    let fixture = fixture();
    let sealed = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"bounded-logical-payload",
    );
    let captured = sealed.clone();
    let pointer = segmented_part_bytes(&sealed, 0, 0).unwrap().as_ptr();
    assert_eq!(
        pointer,
        segmented_part_bytes(&captured, 0, 0).unwrap().as_ptr()
    );
    let retained = observe_segmented_parts(&fixture.pool).unwrap();
    assert_eq!(retained.retained_parts, 2);
    drop(sealed);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_encoded_bytes,
        retained.retained_encoded_bytes
    );
    drop(captured);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_encoded_bytes,
        0
    );
}
#[test]
fn actual_repository_transaction_count_and_namespace_must_match_the_new_profile() {
    let fixture = fixture();
    let mut bad = fixture.budget;
    bad.maximum_batch_records = 64;
    assert!(
        create_segmented_part_pool(
            eve_node_policy::development_public_budget(),
            fixture.pool.policy,
            fixture.pool.codec,
            bad,
            fixture.namespace
        )
        .is_err()
    );
    let mut wrong = fixture.parent;
    wrong.cursor.content_hash[0] ^= 1;
    assert!(matches!(
        start_segmented_worker(fixture.repository, fixture.pool, wrong),
        Err(SegmentedError::WrongCursor)
    ));
}

#[test]
fn foreign_pool_and_exact_age_refuse_without_changing_admitted_positions_or_dropping_ownership() {
    let first = fixture();
    let second = fixture();
    let sealed = batch(&first.pool, first.parent, first.parent.cursor, b"foreign");
    let worker = start_segmented_worker(second.repository, second.pool.clone(), second.parent)
        .unwrap_or_else(|_| panic!("valid worker"));
    let rejected = try_submit_segmented_batch(&worker, second.parent.cursor, sealed)
        .err()
        .unwrap();
    assert_eq!(rejected.error, SegmentedError::ForeignPool);
    assert_eq!(
        observe_segmented_parts(&first.pool).unwrap().retained_parts,
        2
    );
    drop(rejected);
    drop(finish_segmented_worker(worker));
    let mut fixture = fixture();
    std::sync::Arc::get_mut(&mut fixture.pool)
        .unwrap()
        .policy
        .maximum_queue_age_ms = 1;
    let sealed = batch(
        &fixture.pool,
        fixture.parent,
        fixture.parent.cursor,
        b"aged",
    );
    let worker = start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("valid worker"));
    std::thread::sleep(std::time::Duration::from_millis(5));
    let rejected = try_submit_segmented_batch(&worker, fixture.parent.cursor, sealed)
        .err()
        .unwrap();
    assert_eq!(rejected.error, SegmentedError::QueueAged);
    assert_eq!(
        worker.state.admission.lock().unwrap().cursor,
        fixture.parent.cursor
    );
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        2
    );
    drop(rejected);
    drop(finish_segmented_worker(worker));
}

#[test]
fn maximum_segment_staging_uses_actual_repository_budget_and_exceeds_eight_mib() {
    let fixture = fixture();
    assert_eq!(
        super::super::worker::required_segment_scratch(4_194_304, fixture.budget),
        Ok(33_566_808)
    );
    assert_eq!(fixture.pool.policy.maximum_scratch_bytes, 40 * 1_048_576);
    let (base, batch, worker) = super::super::pool::estimate_segmented_metadata();
    assert!((base + 2 * batch + worker) as u64 <= fixture.pool.policy.maximum_metadata_bytes);
}
