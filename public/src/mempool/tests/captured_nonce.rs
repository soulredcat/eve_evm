// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::capture;
use crate::mempool::{MempoolLimits, start_applied_mempool};
use crate::sync::applied::{
    AppliedMode, capture_applied_state, finish_applied_state_service,
    open_applied_state_service_with_mode,
    tests::import_fixtures::{import_chain, import_config, signed_transaction},
    try_apply_recovery_bytes,
};
use eve_evm::decode_signed_transaction;

#[tokio::test]
async fn applied_refresh_revalidates_pending_nonce_against_new_actual_publication() {
    let temporary = tempfile::tempdir().unwrap();
    let chain = import_chain();
    let (mut owner, reader) = open_applied_state_service_with_mode(
        import_config(&temporary.path().join("applied"), &chain),
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    let pool = start_applied_mempool(
        capture_applied_state(&reader).unwrap(),
        MempoolLimits::default(),
    );
    let raw = signed_transaction();
    let validated = decode_signed_transaction(&raw, 31_337, 131_072).unwrap();
    let sender = validated.sender();
    assert_eq!(pool.pending_nonce(sender).await.unwrap(), 0);
    pool.admit(raw.clone(), validated).await.unwrap();
    assert_eq!(pool.pending_nonce(sender).await.unwrap(), 1);
    assert_eq!(pool.select().await.unwrap().len(), 1);
    let (old, _) = capture(&pool).await;
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    pool.applied_committed(capture_applied_state(&reader).unwrap())
        .await
        .unwrap();
    assert_eq!(old.state.accounts[&sender].nonce, 0);
    let (new, _) = capture(&pool).await;
    assert_eq!(new.state.accounts[&sender].nonce, 1);
    assert_eq!(pool.pending_nonce(sender).await.unwrap(), 1);
    assert!(pool.select().await.unwrap().is_empty());
    assert!(
        pool.admit(
            raw.clone(),
            decode_signed_transaction(&raw, 31_337, 131_072).unwrap()
        )
        .await
        .is_err()
    );
    drop(old);
    drop(new);
    drop(pool);
    tokio::task::yield_now().await;
    drop(finish_applied_state_service(owner));
}
