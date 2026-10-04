// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::*;
use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::Arc,
};
fn working() -> Arc<crate::sync::applied::resources::EstimatedWorkingPool> {
    crate::sync::applied::resources::create_estimated_working_pool(256 * 1_048_576).unwrap()
}

#[test]
fn actual_read_slots_saturate_at_frozen_limit_and_release_after_job_drop() {
    let pool =
        create_storage_admission_pool(eve_node_policy::development_public_budget(), &working())
            .unwrap();
    let first = reserve_storage_read(&pool).unwrap();
    let second = reserve_storage_read(&pool).unwrap();
    assert!(matches!(
        reserve_storage_read(&pool),
        Err(StorageAdmissionError::ReadCapacity)
    ));
    let observed = observe_storage_admission(&pool).unwrap();
    assert_eq!(observed.active_reads, 2);
    assert_eq!(observed.peak_reads, 2);
    drop(first);
    assert!(reserve_storage_read(&pool).is_ok());
    drop(second);
    assert_eq!(observe_storage_admission(&pool).unwrap().active_reads, 0);
}
#[test]
fn independent_staging_byte_capacity_is_retained_with_owned_buffers_across_cancelled_observers() {
    let pool =
        create_storage_admission_pool(eve_node_policy::development_public_budget(), &working())
            .unwrap();
    let limit = usize::try_from(pool.staging_limit).unwrap();
    let lease = reserve_snapshot_staging(&pool, limit).unwrap();
    let retained = Arc::new((vec![0_u8; limit], lease));
    let cancelled_observer = Arc::clone(&retained);
    drop(cancelled_observer);
    assert!(matches!(
        reserve_snapshot_staging(&pool, 1),
        Err(StorageAdmissionError::StagingCapacity)
    ));
    assert_eq!(
        observe_storage_admission(&pool)
            .unwrap()
            .reserved_staging_bytes,
        limit as u64
    );
    drop(retained);
    assert_eq!(
        observe_storage_admission(&pool)
            .unwrap()
            .reserved_staging_bytes,
        0
    );
}
#[test]
fn failed_combined_admission_releases_read_slot_before_any_io_and_panic_keeps_retained_buffer_charge()
 {
    let pool =
        create_storage_admission_pool(eve_node_policy::development_public_budget(), &working())
            .unwrap();
    let retained = Arc::new((vec![0_u8; 32], reserve_snapshot_staging(&pool, 32).unwrap()));
    assert!(reserve_checkpoint_storage(&pool, pool.staging_limit as usize).is_err());
    assert_eq!(observe_storage_admission(&pool).unwrap().active_reads, 0);
    let survivor = Arc::clone(&retained);
    let panicked = catch_unwind(AssertUnwindSafe(|| {
        let local_owner = survivor;
        let _read = reserve_storage_read(&pool).unwrap();
        assert_eq!(local_owner.0.len(), 32);
        panic!("unit-only cancelled storage job");
    }));
    assert!(panicked.is_err());
    let observed = observe_storage_admission(&pool).unwrap();
    assert_eq!(observed.active_reads, 0);
    assert_eq!(observed.reserved_staging_bytes, 32);
    drop(retained);
    assert_eq!(
        observe_storage_admission(&pool)
            .unwrap()
            .reserved_staging_bytes,
        0
    );
}
#[test]
fn invalid_budget_and_poisoned_accounting_fail_closed_without_rewriting_limits() {
    let mut budget = eve_node_policy::development_public_budget();
    budget.concurrent_storage_reads = 0;
    assert!(matches!(
        create_storage_admission_pool(budget, &working()),
        Err(StorageAdmissionError::InvalidConfiguration)
    ));
    let pool =
        create_storage_admission_pool(eve_node_policy::development_public_budget(), &working())
            .unwrap();
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _locked = pool.accounting.lock().unwrap();
        panic!("unit-only accounting poison");
    }));
    assert!(result.is_err());
    assert!(matches!(
        reserve_storage_read(&pool),
        Err(StorageAdmissionError::AccountingUnavailable)
    ));
    assert!(matches!(
        reserve_snapshot_staging(&pool, 1),
        Err(StorageAdmissionError::AccountingUnavailable)
    ));
}
