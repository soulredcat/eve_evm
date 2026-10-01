// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_native_block, decode_native_commit, decode_native_header, decode_native_validators,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use eve_consensus_comet::consensus::certificates::{
    hash_consensus_header, hash_transaction_data, validator_address,
};
use serde_json::{Value, json};

fn header() -> Value {
    let fixture:Value=serde_json::from_str(include_str!("../../../../../../validator/components/consensus-comet/tests/fixtures/native-classical/headers/header_cases-supported_block_protocol_11.json")).unwrap();
    let mut header = fixture["header_cases"][0]["header"].clone();
    header["height"] = header["height"].as_i64().unwrap().to_string().into();
    for field in ["app", "block"] {
        header["version"][field] = header["version"][field]
            .as_u64()
            .unwrap()
            .to_string()
            .into();
    }
    header["time"] = prost_types::Timestamp {
        seconds: 1570983284,
        nanos: 0,
    }
    .to_string()
    .into();
    header
}

#[test]
fn actual_native_json_header_shape_preserves_pinned_upstream_header_hash() {
    let value = header();
    let decoded = decode_native_header(&value).unwrap();
    assert_eq!(
        hex::encode(hash_consensus_header(&decoded).unwrap()),
        "7fd9a60e9175500dd554eb5023dd6164b82a0edfc77b10f9fefe8791de49b2f5"
    );
    assert_eq!(decoded.time.unwrap().seconds, 1570983284);
    let mut invalid = value;
    invalid["height"] = "9223372036854775808".into();
    assert!(decode_native_header(&invalid).is_err());
    invalid["height"] = "3".into();
    invalid["time"] = "not-a-date".into();
    assert!(decode_native_header(&invalid).is_err());
}

#[test]
fn native_commit_absent_signature_zero_time_and_validator_identity_decode_strictly() {
    let empty_id = json!({"hash":"","parts":{"total":0,"hash":""}});
    let commit = json!({"height":"1","round":0,"block_id":empty_id,"signatures":[{"block_id_flag":1,"validator_address":"","timestamp":"0001-01-01T00:00:00Z","signature":null}]});
    let decoded = decode_native_commit(&commit).unwrap();
    assert_eq!(
        decoded.signatures[0].timestamp.unwrap().seconds,
        -62_135_596_800
    );
    assert!(decoded.signatures[0].signature.is_empty());
    let public_key = ed25519_dalek::SigningKey::from_bytes(&[1; 32])
        .verifying_key()
        .to_bytes();
    let validators = json!([{"address":hex::encode_upper(validator_address(&public_key)),"pub_key":{"type":"tendermint/PubKeyEd25519","value":STANDARD.encode(public_key)},"voting_power":"10"}]);
    assert_eq!(
        decode_native_validators(&validators).unwrap()[0].public_key,
        public_key
    );
    let mut invalid = validators.clone();
    invalid[0]["address"] = "00".repeat(20).into();
    assert!(decode_native_validators(&invalid).is_err());
    let duplicates = json!([validators[0], validators[0]]);
    assert!(decode_native_validators(&duplicates).is_err());
    let mut invalid = commit;
    invalid["signatures"][0]["signature"] = STANDARD.encode([0; 64]).into();
    assert!(decode_native_commit(&invalid).is_err());
}

#[test]
fn explicit_native_block_container_binds_actual_header_transactions_and_rejects_evidence() {
    let transactions = vec![vec![1_u8, 2, 3]];
    let mut header = header();
    header["data_hash"] = hex::encode_upper(hash_transaction_data(&transactions).unwrap()).into();
    let hash = hash_consensus_header(&decode_native_header(&header).unwrap()).unwrap();
    let result = json!({"block_id":{"hash":hex::encode_upper(hash),"parts":{"total":1,"hash":"ab".repeat(32)}},"block":{"header":header,"data":{"txs":[STANDARD.encode(&transactions[0])]},"evidence":{"evidence":[]},"last_commit":null}});
    let decoded = decode_native_block(&result).unwrap();
    assert_eq!(decoded.transactions, transactions);
    assert_eq!(decoded.block_id.hash, hash);
    assert_eq!(decoded.header.height, 3);
    assert!(decoded.last_commit.is_none());
    let mut invalid = result.clone();
    invalid["block"]["data"]["txs"][0] = STANDARD.encode([9]).into();
    assert!(decode_native_block(&invalid).is_err());
    invalid = result.clone();
    invalid["block_id"]["hash"] = "00".repeat(32).into();
    assert!(decode_native_block(&invalid).is_err());
    invalid = result;
    invalid["block"]["evidence"]["evidence"] = json!([{"unhandled":"evidence"}]);
    assert!(decode_native_block(&invalid).is_err());
}
