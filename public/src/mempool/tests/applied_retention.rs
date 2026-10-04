// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::capture;
use crate::mempool::{MempoolLimits, start_applied_mempool};
use crate::sync::applied::{
    AppliedMode, capture_applied_state, finish_applied_state_service, observe_estimated_working,
    open_applied_state_service_with_mode,
    tests::import_fixtures::{import_chain, import_config},
    try_apply_recovery_bytes,
};
use std::sync::Arc;

#[tokio::test]
async fn old_mempool_capture_retains_actual_applied_generation_charge_after_pool_refresh() {
    let temporary = tempfile::tempdir().unwrap();
    let chain = import_chain();
    let (mut owner, reader) = open_applied_state_service_with_mode(
        import_config(&temporary.path().join("applied"), &chain),
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    let old_publication = capture_applied_state(&reader).unwrap();
    let old_weak = Arc::downgrade(&old_publication);
    let pool = start_applied_mempool(Arc::clone(&old_publication), MempoolLimits::default());
    let (old_capture, selected) = capture(&pool).await;
    assert!(selected.unwrap().is_empty());
    assert!(std::ptr::eq(
        old_capture.commit(),
        crate::sync::applied::applied_commit(&old_publication)
    ));
    drop(old_publication);
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    pool.applied_committed(capture_applied_state(&reader).unwrap())
        .await
        .unwrap();
    assert_eq!(old_capture.target, chain.commits[0].target);
    assert!(old_weak.upgrade().is_some());
    let held = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes;
    drop(old_capture);
    assert!(old_weak.upgrade().is_none());
    assert!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes
            < held
    );
    let (current, _) = capture(&pool).await;
    assert_eq!(current.target, chain.commits[1].target);
    drop(current);
    drop(pool);
    tokio::task::yield_now().await;
    drop(finish_applied_state_service(owner));
}
