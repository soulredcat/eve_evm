// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::import_fixtures::{import_chain, import_config};
use crate::sync::applied::{
    resources::{
        create_estimated_working_pool, estimate_import_charge, estimate_pending_metadata,
        reserve_estimated_working,
        storage_admission::required_storage_admission_control_reservation,
    },
    *,
};
use eve_finality_verifier::{import_wire_stats, preflight_authenticated_import_wire};
use eve_state::{
    BOUNDED_STATE_CODEC_SCRATCH_BYTES, development_state_budget,
    estimate_genesis_initialization_reservation, estimate_journal_wire_candidate_reservation,
};
use std::sync::Arc;

#[test]
fn actual_import_count_charge_has_exact_reservation_boundaries_and_checked_overflow() {
    let chain = import_chain();
    let budget = development_state_budget();
    let sizing_pool =
        create_estimated_working_pool(BOUNDED_STATE_CODEC_SCRATCH_BYTES as u64).unwrap();
    let _scratch =
        reserve_estimated_working(&sizing_pool, BOUNDED_STATE_CODEC_SCRATCH_BYTES).unwrap();
    let preflight = preflight_authenticated_import_wire(&chain.records[0], &budget).unwrap();
    let stats = import_wire_stats(&preflight);
    let candidate = estimate_journal_wire_candidate_reservation(
        &chain.commits[0].state,
        &stats.journal,
        &budget,
    )
    .unwrap();
    let charge = estimate_import_charge(stats, candidate).unwrap();
    assert!(
        charge.total
            < eve_node_policy::development_public_budget().maximum_working_state_bytes as usize
    );
    assert!(
        charge.total
            >= charge.retained + charge.candidate + stats.journal.operation_allocation_bytes
    );
    let exact = create_estimated_working_pool(charge.total as u64).unwrap();
    let held = reserve_estimated_working(&exact, charge.total).unwrap();
    assert!(matches!(
        reserve_estimated_working(&exact, 1),
        Err(AppliedError::EstimatedCapacity)
    ));
    drop(held);
    assert!(reserve_estimated_working(&exact, charge.total).is_ok());
    for dimension in 0..3 {
        let mut bad = stats;
        match dimension {
            0 => bad.journal.operation_allocation_bytes = usize::MAX,
            1 => bad.execution.transaction_count = usize::MAX,
            _ => bad.lookahead.frame.signature_count = usize::MAX,
        }
        assert!(matches!(
            estimate_import_charge(bad, candidate),
            Err(AppliedError::ArithmeticOverflow)
        ));
    }
}

#[test]
fn captured_import_generation_backpressures_then_releases_only_after_the_last_view_drops() {
    let directory = tempfile::tempdir().unwrap();
    let chain = import_chain();
    let mut config = import_config(&directory.path().join("import"), &chain);
    let sizing = create_estimated_working_pool(BOUNDED_STATE_CODEC_SCRATCH_BYTES as u64).unwrap();
    let scratch = reserve_estimated_working(&sizing, BOUNDED_STATE_CODEC_SCRATCH_BYTES).unwrap();
    let genesis =
        estimate_genesis_initialization_reservation(&chain.genesis, &config.state_budget).unwrap();
    let preflight =
        preflight_authenticated_import_wire(&chain.records[0], &config.state_budget).unwrap();
    let stats = import_wire_stats(&preflight);
    let candidate = estimate_journal_wire_candidate_reservation(
        &chain.commits[0].state,
        &stats.journal,
        &config.state_budget,
    )
    .unwrap();
    let charge = estimate_import_charge(stats, candidate).unwrap();
    let metadata = estimate_pending_metadata(config.public_budget.queue_batches as usize).unwrap();
    let control = required_storage_admission_control_reservation().unwrap();
    // This fixture freezes the exact minimum, including the real owner controller allocation.
    config.public_budget.maximum_working_state_bytes =
        (genesis + metadata + control + BOUNDED_STATE_CODEC_SCRATCH_BYTES + charge.total) as u64;
    drop(scratch);
    let (mut owner, reader) = open_applied_state_service_with_mode(
        config,
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    let old = capture_applied_state(&reader).unwrap();
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    let first = capture_applied_state(&reader).unwrap();
    let cursor = owner.admitted_cursor;
    assert!(matches!(
        try_apply_recovery_bytes(&mut owner, &chain.records[1]),
        Err(AppliedError::EstimatedCapacity)
    ));
    assert!(Arc::ptr_eq(
        &first,
        &capture_applied_state(&reader).unwrap()
    ));
    assert_eq!(owner.admitted_cursor, cursor);
    assert_eq!(applied_commit(&old), &chain.commits[0]);
    drop(old);
    assert_eq!(
        try_apply_recovery_bytes(&mut owner, &chain.records[1])
            .unwrap()
            .applied
            .0,
        2
    );
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.acknowledgement_error.is_none());
    drop(first);
    drop(shutdown);
    let last = capture_applied_state(&reader).unwrap();
    let retained = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes
        .checked_sub(control as u64)
        .unwrap() as usize;
    let limit = observe_estimated_working(&reader).unwrap().limit as usize;
    let working = Arc::clone(&reader.working);
    drop(reader);
    // The same pool proves exact controller release while the actual last generation survives.
    let remainder = reserve_estimated_working(&working, limit - retained).unwrap();
    assert!(matches!(
        reserve_estimated_working(&working, 1),
        Err(AppliedError::EstimatedCapacity)
    ));
    drop(remainder);
    assert!(matches!(
        reserve_estimated_working(&working, limit),
        Err(AppliedError::EstimatedCapacity)
    ));
    drop(last);
    let full = reserve_estimated_working(&working, limit).unwrap();
    assert!(matches!(
        reserve_estimated_working(&working, 1),
        Err(AppliedError::EstimatedCapacity)
    ));
    drop(full);
    assert!(reserve_estimated_working(&working, limit).is_ok());
}
