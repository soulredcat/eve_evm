// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{development_fixture::development_fixture, sign_type_two::sign_type_two};
use alloy_primitives::{Bytes, U256};
use eve_evm::decode_signed_transaction;
use eve_public::mempool::{MempoolLimits, start_mempool};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

#[tokio::test]
async fn ta04_count_sender_and_exact_byte_caps_reject_without_eviction() {
    let fixture = development_fixture(Bytes::new());
    let first = sign_type_two(
        &fixture.key,
        0,
        fixture.contract,
        2_000_000_000,
        1000,
        U256::ZERO,
    );
    let second = sign_type_two(
        &fixture.key,
        1,
        fixture.contract,
        2_000_000_000,
        1000,
        U256::ZERO,
    );
    let third = sign_type_two(
        &fixture.key,
        2,
        fixture.contract,
        2_000_000_000,
        1000,
        U256::ZERO,
    );
    let other = sign_type_two(
        &fixture.secondary_key,
        0,
        fixture.contract,
        2_000_000_000,
        1000,
        U256::ZERO,
    );
    for limits in [
        MempoolLimits {
            maximum_transactions: 2,
            maximum_per_sender: 64,
            ..MempoolLimits::default()
        },
        MempoolLimits {
            maximum_transactions: 100,
            maximum_per_sender: 2,
            ..MempoolLimits::default()
        },
    ] {
        let pool = start_mempool(Arc::new(fixture.parent.clone()), limits);
        for raw in [&first, &second] {
            pool.admit(
                raw.clone(),
                decode_signed_transaction(raw, 31_337, 131_072).unwrap(),
            )
            .await
            .unwrap();
        }
        assert!(
            pool.admit(
                third.clone(),
                decode_signed_transaction(&third, 31_337, 131_072).unwrap()
            )
            .await
            .is_err()
        );
        assert_eq!(pool.select().await.unwrap().len(), 2);
        assert_eq!(pool.pending_nonce(fixture.sender).await.unwrap(), 2);
        let admitted_other = pool
            .admit(
                other.clone(),
                decode_signed_transaction(&other, 31_337, 131_072).unwrap(),
            )
            .await;
        assert_eq!(
            admitted_other.is_ok(),
            limits.maximum_transactions > 2,
            "A sender cap cannot become a global cap"
        );
    }
    for (maximum_bytes, accepted) in [(first.len() - 1, false), (first.len(), true)] {
        let pool = start_mempool(
            Arc::new(fixture.parent.clone()),
            MempoolLimits {
                maximum_bytes,
                ..MempoolLimits::default()
            },
        );
        assert_eq!(
            pool.admit(
                first.clone(),
                decode_signed_transaction(&first, 31_337, 131_072).unwrap()
            )
            .await
            .is_ok(),
            accepted
        );
        assert_eq!(pool.select().await.unwrap().len(), usize::from(accepted));
    }
}

#[tokio::test]
async fn ta04_local_ttl_expires_at_exact_monotonic_boundary_without_changing_consensus() {
    let fixture = development_fixture(Bytes::new());
    let limits = MempoolLimits::default();
    assert_eq!(
        (
            limits.maximum_transactions,
            limits.maximum_bytes,
            limits.maximum_per_sender
        ),
        (10_000, 64 * 1_048_576, 64)
    );
    assert_eq!(limits.ttl, Duration::from_secs(300));
    let pool = start_mempool(Arc::new(fixture.parent.clone()), limits);
    let raw = sign_type_two(
        &fixture.key,
        0,
        fixture.contract,
        2_000_000_000,
        1000,
        U256::ZERO,
    );
    let validated = decode_signed_transaction(&raw, 31_337, 131_072).unwrap();
    let hash = validated.hash();
    pool.admit(raw, validated).await.unwrap();
    let admitted = pool.find(hash).await.unwrap().unwrap().admitted_at;
    assert!(admitted <= Instant::now());
    assert_eq!(
        pool.evict_at(admitted + limits.ttl - Duration::from_nanos(1))
            .await
            .unwrap(),
        0
    );
    assert!(pool.find(hash).await.unwrap().is_some());
    assert_eq!(pool.evict_at(admitted + limits.ttl).await.unwrap(), 1);
    assert!(pool.find(hash).await.unwrap().is_none());
    assert_eq!(pool.pending_nonce(fixture.sender).await.unwrap(), 0);
    assert_eq!(fixture.parent.target.height, 0);
    assert_eq!(fixture.parent.state.accounts[&fixture.sender].nonce, 0);
}
