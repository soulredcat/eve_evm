// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{create_estimated_working_pool, merge_estimated_working, reserve_estimated_working};
use crate::sync::applied::AppliedError;

#[test]
fn merged_generation_charge_keeps_capacity_reserved_until_the_last_owner_drops() {
    let pool = create_estimated_working_pool(300).unwrap();
    let first = reserve_estimated_working(&pool, 100).unwrap();
    let second = reserve_estimated_working(&pool, 200).unwrap();
    let merged = merge_estimated_working(first, second).unwrap();
    assert_eq!(merged.bytes, 300);
    assert_eq!(pool.accounting.lock().unwrap().bytes, 300);
    assert!(matches!(
        reserve_estimated_working(&pool, 1),
        Err(AppliedError::EstimatedCapacity)
    ));
    drop(merged);
    assert_eq!(pool.accounting.lock().unwrap().bytes, 0);
    let replacement = reserve_estimated_working(&pool, 300).unwrap();
    drop(replacement);
    assert_eq!(pool.accounting.lock().unwrap().bytes, 0);
}

#[test]
fn foreign_pool_merge_refuses_and_releases_each_original_pool_charge() {
    let first_pool = create_estimated_working_pool(100).unwrap();
    let second_pool = create_estimated_working_pool(100).unwrap();
    let first = reserve_estimated_working(&first_pool, 100).unwrap();
    let second = reserve_estimated_working(&second_pool, 100).unwrap();
    assert!(matches!(
        merge_estimated_working(first, second),
        Err(AppliedError::InvalidConfiguration)
    ));
    assert_eq!(first_pool.accounting.lock().unwrap().bytes, 0);
    assert_eq!(second_pool.accounting.lock().unwrap().bytes, 0);
}
