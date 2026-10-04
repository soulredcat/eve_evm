// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{config, empty_chain};
use crate::{
    persistence::handoff::{
        observe_handoff, recovery_payload_bytes, reserve_recovery_payload, seal_recovery_payload,
        write_reserved_payload,
    },
    sync::applied::{
        admission::admit_prepared_publication::admit_prepared_publication,
        publication::build_applied_markers,
        recovery::prepare_empty_recovery,
        resources::{estimate_replay_charge, reserve_estimated_working, split_estimated_working},
        state::{AppliedState, applied_state_commit},
        types::ChargedAppliedState,
        *,
    },
};
use eve_evm::estimate_clone_reservation;
use eve_finality_verifier::{into_recovery_state, recovery_state_commit};
use eve_storage::records::prospective_opaque_record_cursor;
use std::{sync::Arc, thread, time::Duration};

#[test]
fn queue_age_expiring_after_actual_preparation_rejects_final_admission_without_publication() {
    let directory = tempfile::tempdir().unwrap();
    let chain = empty_chain();
    let mut config = config(&directory.path().join("applied"), &chain);
    config.public_budget.queue_age_ms = 1;
    let (mut owner, reader) = open_applied_state_service(config, &chain.genesis)
        .ok()
        .unwrap();
    let original = capture_applied_state(&reader).unwrap();
    let initial_charge = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes;
    let cursor = owner.admitted_cursor;
    let oracle =
        estimate_clone_reservation(&applied_state_commit(&original.generation.state).state)
            .unwrap();
    let charge = estimate_replay_charge(
        &owner.config.state_budget,
        owner.config.maximum_recovery_payload_bytes,
        oracle,
    )
    .unwrap();
    let lease = reserve_estimated_working(&owner.reader.working, charge.total).unwrap();
    let mut reservation = reserve_recovery_payload(
        crate::sync::applied::admission::compact_pool(&owner).unwrap(),
        chain.records[0].len(),
    )
    .unwrap();
    write_reserved_payload(&mut reservation, &chain.records[0]).unwrap();
    let payload = seal_recovery_payload(reservation).unwrap();
    let AppliedState::EmptyReplay(parent) = &original.generation.state else {
        panic!("explicit replay fixture");
    };
    let prepared = prepare_empty_recovery(
        parent,
        recovery_payload_bytes(&payload),
        &owner.config.state_budget,
        oracle,
    )
    .unwrap();
    let recovery = into_recovery_state(prepared);
    let target = recovery_state_commit(&recovery).target.clone();
    let state = AppliedState::EmptyReplay(recovery);
    let markers = build_applied_markers(&state, 0).unwrap();
    let next = prospective_opaque_record_cursor(
        owner.config.identity,
        cursor,
        recovery_payload_bytes(&payload),
        owner.config.repository_budget.maximum_record_bytes,
    )
    .unwrap();
    let (retained, transient) = split_estimated_working(lease, charge.retained).unwrap();
    let publication = Arc::new(AppliedPublication {
        generation: Arc::new(ChargedAppliedState {
            state,
            _lease: retained,
        }),
        markers,
        durable_cursor: owner.durable_cursor,
        admitted_cursor: next,
        storage_failed: false,
        segmented_position: None,
    });
    drop(transient);
    thread::sleep(Duration::from_millis(5));
    let rejected = admit_prepared_publication(&mut owner, publication, payload, next, target);
    assert!(matches!(rejected, Err(AppliedError::QueueLimit)));
    assert!(Arc::ptr_eq(
        &original,
        &capture_applied_state(&reader).unwrap()
    ));
    assert_eq!(owner.admitted_cursor, cursor);
    assert!(owner.pending.is_empty());
    assert_eq!(
        observe_handoff(crate::sync::applied::admission::compact_pool(&owner).unwrap())
            .unwrap()
            .retained_bytes,
        0
    );
    assert_eq!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes,
        initial_charge
    );
    drop(finish_applied_state_service(owner));
}
