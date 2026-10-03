// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::native_decoding::decode_native_header;
use base64::{Engine, engine::general_purpose::STANDARD};
use eve_consensus_comet::consensus::certificates::{hash_consensus_header, hash_transaction_data};
use serde_json::{Value, json};

/// Synthetic RPC shape using the existing public header fixture; no certificate/finality claim.
#[cfg(test)]
pub(in super::super) fn build_native_block_response(
    height: i64,
    transactions: &[Vec<u8>],
) -> Value {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../../../../validator/components/consensus-comet/tests/fixtures/native-classical/headers/header_cases-supported_block_protocol_11.json"
    )).unwrap();
    let mut header = fixture["header_cases"][0]["header"].clone();
    header["height"] = height.to_string().into();
    for field in ["app", "block"] {
        header["version"][field] = header["version"][field]
            .as_u64()
            .unwrap()
            .to_string()
            .into();
    }
    header["time"] = prost_types::Timestamp {
        seconds: 1_570_983_284,
        nanos: 0,
    }
    .to_string()
    .into();
    header["data_hash"] = hex::encode_upper(hash_transaction_data(transactions).unwrap()).into();
    let hash = hash_consensus_header(&decode_native_header(&header).unwrap()).unwrap();
    let encoded: Vec<_> = transactions
        .iter()
        .map(|raw| STANDARD.encode(raw))
        .collect();
    json!({
        "block_id": {"hash": hex::encode_upper(hash), "parts": {"total": 1, "hash": "ab".repeat(32)}},
        "block": {"header": header, "data": {"txs": encoded}, "evidence": {"evidence": []}, "last_commit": null}
    })
}
