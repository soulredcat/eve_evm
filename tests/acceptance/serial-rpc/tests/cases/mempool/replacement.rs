// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{development_fixture::development_fixture, sign_type_two::sign_type_two};
use alloy_primitives::{Bytes, U256};
use eve_evm::decode_signed_transaction;
use eve_public::mempool::{MempoolLimits, start_mempool};
use std::sync::Arc;

#[tokio::test]
async fn ta04_type_two_replacement_requires_ceiling_ten_percent_on_both_caps() {
    let fixture = development_fixture(Bytes::new());
    let pool = start_mempool(Arc::new(fixture.parent), MempoolLimits::default());
    let original = sign_type_two(
        &fixture.key,
        0,
        fixture.contract,
        1_000_000_001,
        11,
        U256::ZERO,
    );
    let validated = decode_signed_transaction(&original, 31_337, 131_072).unwrap();
    let original_hash = validated.hash();
    pool.admit(original, validated).await.unwrap();
    for (fee, tip) in [(1_100_000_001, 13), (1_100_000_002, 12)] {
        let raw = sign_type_two(&fixture.key, 0, fixture.contract, fee, tip, U256::ZERO);
        assert!(
            pool.admit(
                raw.clone(),
                decode_signed_transaction(&raw, 31_337, 131_072).unwrap()
            )
            .await
            .is_err()
        );
        assert!(
            pool.find(original_hash).await.unwrap().is_some(),
            "Failed replacements cannot erase the accepted original"
        );
    }
    let replacement = sign_type_two(
        &fixture.key,
        0,
        fixture.contract,
        1_100_000_002,
        13,
        U256::ZERO,
    );
    let validated = decode_signed_transaction(&replacement, 31_337, 131_072).unwrap();
    let replacement_hash = validated.hash();
    pool.admit(replacement, validated).await.unwrap();
    assert!(pool.find(original_hash).await.unwrap().is_none());
    assert!(pool.find(replacement_hash).await.unwrap().is_some());
    assert_eq!(pool.select().await.unwrap().len(), 1);
}

#[tokio::test]
async fn ta04_zero_priority_fee_replacement_still_requires_one_wei_increase() {
    let fixture = development_fixture(Bytes::new());
    let pool = start_mempool(Arc::new(fixture.parent), MempoolLimits::default());
    let original = sign_type_two(
        &fixture.key,
        0,
        fixture.contract,
        1_000_000_000,
        0,
        U256::ZERO,
    );
    pool.admit(
        original.clone(),
        decode_signed_transaction(&original, 31_337, 131_072).unwrap(),
    )
    .await
    .unwrap();
    let insufficient = sign_type_two(
        &fixture.key,
        0,
        fixture.contract,
        1_100_000_000,
        0,
        U256::ZERO,
    );
    assert!(
        pool.admit(
            insufficient.clone(),
            decode_signed_transaction(&insufficient, 31_337, 131_072).unwrap()
        )
        .await
        .is_err()
    );
    let sufficient = sign_type_two(
        &fixture.key,
        0,
        fixture.contract,
        1_100_000_000,
        1,
        U256::ZERO,
    );
    pool.admit(
        sufficient.clone(),
        decode_signed_transaction(&sufficient, 31_337, 131_072).unwrap(),
    )
    .await
    .unwrap();
    assert_eq!(pool.select().await.unwrap()[0].raw, sufficient);
}
