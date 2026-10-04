// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{MIB, bounds};
use eve_node_policy::{
    development_public_budget, development_segmented_recovery_policy,
    validate_segmented_recovery_policy,
};

#[test]
fn v2_derives_three_data_parts_and_one_marker_without_changing_v1_caps() {
    let base = development_public_budget();
    let bounds = bounds();
    let policy = development_segmented_recovery_policy(base, bounds).unwrap();
    let capacity = validate_segmented_recovery_policy(base, policy, bounds).unwrap();
    assert_eq!(base.version, 1);
    assert_eq!(
        (base.maximum_record_bytes, base.maximum_batch_bytes),
        (8 * MIB, 8 * MIB)
    );
    assert_eq!(
        (
            base.maximum_batch_records,
            base.queue_batches,
            base.queue_bytes
        ),
        (1, 4, 32 * MIB)
    );
    assert_eq!(policy.version, 2);
    assert_eq!(
        (
            policy.retained_parts,
            policy.maximum_part_bytes,
            policy.maximum_segments_per_part
        ),
        (4, 8 * MIB, 2)
    );
    assert_eq!(capacity.data_bytes_per_segment, 4_194_095);
    assert_eq!(
        (
            capacity.maximum_data_segments,
            capacity.maximum_data_parts,
            capacity.maximum_retained_parts
        ),
        (6, 3, 4)
    );
    assert_eq!(capacity.maximum_retained_payload_bytes, 21_027_288);
    assert_eq!(capacity.maximum_part_payload_bytes, 8 * MIB);
    assert_eq!(capacity.maximum_physical_record_bytes, 4_194_392);
    assert_eq!(capacity.maximum_marker_record_bytes, 553);
    assert_eq!(
        (
            capacity.maximum_physical_records,
            capacity.physical_records_per_transaction
        ),
        (7, 1)
    );
    assert_eq!(capacity.required_metadata_bytes, 4_096);
    assert_eq!(capacity.required_scratch_bytes, 33_566_808);
    assert_eq!(capacity.reserved_auxiliary_bytes, 40 * MIB + 8_192);
    assert_eq!(
        (
            policy.maximum_queue_age_ms,
            policy.maximum_logical_lag_blocks
        ),
        (2_000, 8)
    );
    assert!(policy.maximum_scratch_bytes > base.maximum_batch_bytes);
}

#[test]
fn unused_base_allowance_supports_declared_scratch_without_new_memory_pools() {
    let base = development_public_budget();
    let mut bounds = bounds();
    let policy = development_segmented_recovery_policy(base, bounds).unwrap();
    bounds.available_auxiliary_bytes = policy.maximum_metadata_bytes + policy.maximum_scratch_bytes;
    assert!(validate_segmented_recovery_policy(base, policy, bounds).is_ok());
    bounds.available_auxiliary_bytes -= 1;
    assert!(validate_segmented_recovery_policy(base, policy, bounds).is_err());
}

#[test]
fn v1_physical_batch_counter_is_not_an_alias_for_v2_retained_parts() {
    let mut base = development_public_budget();
    base.queue_batches = 1;
    let policy = development_segmented_recovery_policy(base, bounds()).unwrap();
    assert_eq!(base.queue_batches, 1);
    assert_eq!(policy.retained_parts, 4);
    assert_eq!(base.maximum_batch_records, 1);
    assert_eq!(
        validate_segmented_recovery_policy(base, policy, bounds())
            .unwrap()
            .physical_records_per_transaction,
        1
    );
}
