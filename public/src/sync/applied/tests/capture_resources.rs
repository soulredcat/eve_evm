// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{config, empty_chain};
use crate::sync::applied::{
    resources::{
        estimate_pending_metadata, estimate_replay_charge, estimated_clone_ceiling,
        estimated_repository_read_charge, reserve_estimated_working,
        storage_admission::required_storage_admission_control_reservation,
    },
    *,
};
use std::sync::Arc;

#[test]
fn old_captured_state_keeps_its_charge_and_backpressures_later_application() {
    let directory = tempfile::tempdir().unwrap();
    let chain = empty_chain();
    let mut config = config(&directory.path().join("applied"), &chain);
    let charge = estimate_replay_charge(
        &config.state_budget,
        config.maximum_recovery_payload_bytes,
        estimated_clone_ceiling(&config.state_budget).unwrap(),
    )
    .unwrap();
    let control = required_storage_admission_control_reservation().unwrap();
    // The exact minimal fixture includes the real independently retained owner controller.
    config.public_budget.maximum_working_state_bytes = (charge.retained
        + charge.total
        + estimated_repository_read_charge(&config.repository_budget).unwrap()
        + 2 * estimate_pending_metadata(config.public_budget.queue_batches as usize).unwrap()
        + control) as u64;
    let (mut owner, reader) = open_applied_state_service(config, &chain.genesis)
        .ok()
        .unwrap();
    let old = capture_applied_state(&reader).unwrap();
    let initial = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes;
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    let first = capture_applied_state(&reader).unwrap();
    assert_eq!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes,
        initial + charge.retained as u64
    );
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
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes,
        initial
    );
    assert_eq!(
        try_apply_recovery_bytes(&mut owner, &chain.records[1])
            .unwrap()
            .applied
            .0,
        2
    );
    let shutdown = finish_applied_state_service(owner);
    drop(first);
    assert_eq!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes,
        initial
    );
    drop(shutdown);
    assert_eq!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes,
        (charge.retained + control) as u64
    );
    let limit = observe_estimated_working(&reader).unwrap().limit as usize;
    let working = Arc::clone(&reader.working);
    let last = capture_applied_state(&reader).unwrap();
    drop(reader);
    // Reader drop releases the independent controller; only the captured generation remains.
    let remainder = reserve_estimated_working(&working, limit - charge.retained).unwrap();
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
