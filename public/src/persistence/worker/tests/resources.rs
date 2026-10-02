// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{fixture, payload};
use crate::persistence::{handoff::*, worker::*};
use eve_storage::records::*;
use std::sync::Arc;

#[test]
fn scratch_rejection_occurs_before_io_without_failing_healthy_worker() {
    let fixture = fixture();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    let worker = start_record_worker(fixture.repository, Arc::clone(&fixture.pool), 1)
        .ok()
        .unwrap();
    let rejected = try_submit_record(&worker, parent, payload(&fixture.pool, b"small"))
        .err()
        .unwrap();
    assert_eq!(rejected.error, RecordWorkerError::ScratchLimit);
    assert_eq!(recovery_payload_bytes(&rejected.payload), b"small");
    assert!(!observe_record_worker(&worker).storage_failed);
    assert_eq!(observe_record_worker(&worker).active_scratch_bytes, 0);
    drop(rejected);
    assert_eq!(
        opaque_record_cursor(&finish_record_worker(worker).ok().unwrap()).unwrap(),
        parent
    );
}

#[test]
fn worker_rejects_scratch_above_profile_slack_and_incompatible_db_knobs() {
    let first = fixture();
    assert!(matches!(
        start_record_worker(first.repository, first.pool, 512 * 1_048_576),
        Err(RecordWorkerError::InvalidConfiguration)
    ));
    let second = fixture();
    drop(second.repository);
    let mut budget = second.budget;
    budget.maximum_open_files = 64;
    let repository = open_opaque_record_repository(&second.path, second.identity, budget).unwrap();
    assert!(matches!(
        start_record_worker(repository, second.pool, 40 * 1_048_576),
        Err(RecordWorkerError::InvalidConfiguration)
    ));
}

#[test]
fn full_public_payload_is_rejected_when_actual_repository_record_limit_is_smaller() {
    let fixture = fixture();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    let worker = start_record_worker(
        fixture.repository,
        Arc::clone(&fixture.pool),
        40 * 1_048_576,
    )
    .ok()
    .unwrap();
    let mut reservation = reserve_recovery_payload(&fixture.pool, 8 * 1_048_576)
        .ok()
        .unwrap();
    let chunk = [0_u8; 4096];
    for _ in 0..2048 {
        write_reserved_payload(&mut reservation, &chunk).unwrap();
    }
    let rejected = try_submit_record(
        &worker,
        parent,
        seal_recovery_payload(reservation).ok().unwrap(),
    )
    .err()
    .unwrap();
    assert_eq!(rejected.error, RecordWorkerError::RecordLimit);
    assert_eq!(
        observe_handoff(&fixture.pool).unwrap().retained_bytes,
        8 * 1_048_576
    );
    drop(rejected);
    assert_eq!(
        opaque_record_cursor(&finish_record_worker(worker).ok().unwrap()).unwrap(),
        parent
    );
    assert_eq!(observe_handoff(&fixture.pool).unwrap().retained_bytes, 0);
}
