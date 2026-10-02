// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#![cfg(test)]

use super::*;
use eve_node_policy::development_public_budget;
use std::sync::{Arc, Barrier};

#[cfg(test)]
fn pool() -> Arc<HandoffPool> {
    let mut budget = development_public_budget();
    budget.maximum_record_bytes = 8;
    budget.maximum_batch_bytes = 8;
    budget.queue_bytes = 32;
    budget.queue_batches = 4;
    create_handoff_pool(budget)
        .ok()
        .expect("valid bounded test pool")
}

#[test]
fn payload_limit_checks_actual_allocation_and_releases_cancelled_reservations() {
    let pool = pool();
    assert!(matches!(
        reserve_recovery_payload(&pool, 0),
        Err(HandoffError::InvalidLength)
    ));
    assert!(matches!(
        reserve_recovery_payload(&pool, 9),
        Err(HandoffError::PayloadLimit)
    ));
    let reservation = reserve_recovery_payload(&pool, 8).ok().unwrap();
    assert_eq!(reservation.bytes.capacity(), 8);
    assert_eq!(observe_handoff(&pool).unwrap().retained_bytes, 8);
    drop(reservation);
    assert_eq!(observe_handoff(&pool).unwrap().retained_bytes, 0);
    assert_eq!(observe_handoff(&pool).unwrap().retained_batches, 0);
}

#[test]
fn count_limit_rejects_one_more_even_when_bytes_remain() {
    let pool = pool();
    let reservations = (0..4)
        .map(|_| reserve_recovery_payload(&pool, 1).ok().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(observe_handoff(&pool).unwrap().retained_bytes, 4);
    assert!(matches!(
        reserve_recovery_payload(&pool, 1),
        Err(HandoffError::QueueLimit)
    ));
    drop(reservations);
    assert_eq!(observe_handoff(&pool).unwrap().retained_batches, 0);
}

#[test]
fn encoded_overflow_is_atomic_and_incomplete_seal_releases_charge() {
    let pool = pool();
    let mut reservation = reserve_recovery_payload(&pool, 8).ok().unwrap();
    write_reserved_payload(&mut reservation, b"abc").unwrap();
    assert_eq!(
        write_reserved_payload(&mut reservation, b"123456"),
        Err(HandoffError::PayloadLimit)
    );
    assert_eq!(reservation.bytes.as_slice(), b"abc");
    assert_eq!(reservation.bytes.capacity(), 8);
    assert!(matches!(
        seal_recovery_payload(reservation),
        Err(HandoffError::IncompletePayload)
    ));
    assert_eq!(observe_handoff(&pool).unwrap().retained_bytes, 0);
}

#[test]
fn cloned_inflight_payload_retains_charge_until_last_owner_drops() {
    let pool = pool();
    let mut reservation = reserve_recovery_payload(&pool, 8).ok().unwrap();
    write_reserved_payload(&mut reservation, b"12345678").unwrap();
    let address = reservation.bytes.as_ptr();
    let payload = seal_recovery_payload(reservation).ok().unwrap();
    assert_eq!(recovery_payload_bytes(&payload).as_ptr(), address);
    let inflight = payload.clone();
    let retained_reader = inflight.clone();
    drop(payload);
    drop(inflight);
    assert_eq!(recovery_payload_bytes(&retained_reader), b"12345678");
    assert_eq!(observe_handoff(&pool).unwrap().retained_bytes, 8);
    assert_eq!(observe_handoff(&pool).unwrap().retained_batches, 1);
    drop(retained_reader);
    assert_eq!(observe_handoff(&pool).unwrap().retained_bytes, 0);
}

#[test]
fn concurrent_reservations_never_overcommit_count_or_bytes() {
    let pool = pool();
    let phases = Arc::new(Barrier::new(9));
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let pool = Arc::clone(&pool);
            let phases = Arc::clone(&phases);
            scope.spawn(move || {
                phases.wait();
                let reservation = reserve_recovery_payload(&pool, 8);
                phases.wait();
                phases.wait();
                drop(reservation);
            });
        }
        phases.wait();
        phases.wait();
        let observation = observe_handoff(&pool);
        let overflow = reserve_recovery_payload(&pool, 1);
        // Release waiting workers before any assertion can panic and strand the scope.
        phases.wait();
        let observation = observation.unwrap();
        assert_eq!(observation.retained_bytes, 32);
        assert_eq!(observation.retained_batches, 4);
        assert!(matches!(overflow, Err(HandoffError::QueueLimit)));
    });
    assert_eq!(observe_handoff(&pool).unwrap().retained_bytes, 0);
    assert_eq!(observe_handoff(&pool).unwrap().retained_batches, 0);
}

#[test]
fn invalid_budget_fails_before_reservation_and_age_covers_construction() {
    let mut budget = development_public_budget();
    budget.queue_bytes = 0;
    assert!(matches!(
        create_handoff_pool(budget),
        Err(HandoffError::InvalidBudget)
    ));
    let pool = pool();
    let reservation = reserve_recovery_payload(&pool, 8).ok().unwrap();
    let earlier = observe_handoff(&pool).unwrap().oldest_age;
    let later = observe_handoff(&pool).unwrap().oldest_age;
    assert!(later >= earlier);
    drop(reservation);
    assert_eq!(
        observe_handoff(&pool).unwrap().oldest_age,
        std::time::Duration::ZERO
    );
}

#[test]
fn allocation_capacity_error_releases_the_reserved_item_and_bytes() {
    // Internal-only impossible production budget forces Vec's deterministic capacity
    // rejection without attempting an enormous allocation or weakening the constructor.
    let mut budget = development_public_budget();
    budget.maximum_record_bytes = u64::MAX;
    budget.maximum_batch_bytes = u64::MAX;
    budget.queue_bytes = u64::MAX;
    let pool = Arc::new(HandoffPool {
        budget,
        accounting: std::sync::Mutex::new(super::types::Accounting {
            bytes: 0,
            next_id: 0,
            leases: std::collections::BTreeMap::new(),
        }),
    });
    assert!(matches!(
        reserve_recovery_payload(&pool, usize::MAX),
        Err(HandoffError::AllocationFailed)
    ));
    assert_eq!(observe_handoff(&pool).unwrap().retained_bytes, 0);
    assert_eq!(observe_handoff(&pool).unwrap().retained_batches, 0);
}
