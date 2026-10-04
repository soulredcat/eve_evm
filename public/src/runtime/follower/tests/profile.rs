// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::build_segmented_follower_configuration::build_segmented_follower_configuration;
use eve_node_policy::development_public_budget;
use eve_storage::records::OpaqueRecordIdentity;

#[test]
fn follower_profile_preserves_declared_working_queue_scratch_and_single_record_storage_limits() {
    let config = build_segmented_follower_configuration(
        "unused-test-namespace".into(),
        OpaqueRecordIdentity {
            genesis_hash: [1; 32],
            owner: [2; 32],
            domain: [3; 32],
        },
    )
    .unwrap();
    let base = development_public_budget();
    assert_eq!(config.application.public_budget, base);
    assert_eq!(
        config.application.repository_budget.maximum_batch_records,
        1
    );
    assert_eq!(config.application.repository_budget.maximum_open_files, 32);
    assert_eq!(config.policy.retained_parts, 4);
    assert_eq!(config.policy.maximum_queue_age_ms, 2_000);
    assert_eq!(config.policy.maximum_scratch_bytes, 40 * 1_048_576);
    assert_eq!(
        config.codec.maximum_logical_bytes,
        eve_finality_verifier::MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES
    );
    assert_eq!(base.maximum_working_state_bytes, 256 * 1_048_576);
    assert_eq!(
        base.query_cache_bytes + base.simulation_overlay_bytes,
        32 * 1_048_576
    );
}
