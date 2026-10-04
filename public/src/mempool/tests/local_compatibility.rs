// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::capture;
use crate::mempool::{MempoolHead, MempoolLimits, start_mempool};
use crate::sync::applied::tests::import_fixtures::{import_chain, signed_transaction};
use eve_evm::decode_signed_transaction;
use std::sync::Arc;

#[tokio::test]
async fn existing_local_constructor_and_commit_api_keep_same_shared_state_identity() {
    let chain = import_chain();
    let initial = Arc::new(chain.commits[0].clone());
    let pool = start_mempool(Arc::clone(&initial), MempoolLimits::default());
    let (captured, _) = capture(&pool).await;
    assert!(matches!(captured.as_ref(), MempoolHead::Local(_)));
    assert!(std::ptr::eq(captured.commit(), initial.as_ref()));
    let next = Arc::new(chain.commits[1].clone());
    pool.committed(Arc::clone(&next)).await.unwrap();
    let (latest, _) = capture(&pool).await;
    assert!(std::ptr::eq(latest.commit(), next.as_ref()));
    assert_eq!(captured.target.height, 0);
    let sender = decode_signed_transaction(&signed_transaction(), 31_337, 131_072)
        .unwrap()
        .sender();
    assert_eq!(pool.pending_nonce(sender).await.unwrap(), 1);
    drop(captured);
    drop(latest);
    drop(pool);
    tokio::task::yield_now().await;
}
