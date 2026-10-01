// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::corpus;
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::U256;
use eve_evm::decode_signed_transaction;

#[test]
fn te01_pinned_transaction_nonce_overflow_is_rejected_at_deserialization() {
    let inventory: corpus::types::CorpusInventory = serde_json::from_slice(
        &std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("fixtures/corpus/shanghai/transaction_validation.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(inventory.files.len(), 1);
    let (relative, expected_digest) = inventory.files.into_iter().next().unwrap();
    let path =
        std::path::PathBuf::from(std::env::var_os("EVE_SHANGHAI_FIXTURES").unwrap()).join(relative);
    let bytes = std::fs::read(path).unwrap();
    use sha2::{Digest, Sha256};
    assert_eq!(hex::encode(Sha256::digest(&bytes)), expected_digest);
    let document: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let object = document.as_object().unwrap();
    assert_eq!(object.len(), 1);
    let case = object.values().next().unwrap();
    assert_eq!(
        case["result"]["Shanghai"]["exception"],
        "TransactionException.NONCE_OVERFLOW"
    );
    let raw: alloy_primitives::Bytes = serde_json::from_value(case["txbytes"].clone()).unwrap();
    assert_eq!(
        raw[2], 0x89,
        "Literal canonical nine-byte nonce is beyond u64"
    );
    assert!(alloy_consensus::TxEnvelope::decode_2718(&mut raw.as_ref()).is_err());
    assert!(decode_signed_transaction(&raw, 1, 128 * 1024).is_err());
    assert!(U256::from(1_u128 << 64) > U256::from(u64::MAX));
}
