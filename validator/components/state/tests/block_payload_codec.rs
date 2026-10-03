// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#![cfg(test)]

#[path = "block_payload_codec_support/commit_block_field.rs"]
mod commit_block_field;
#[path = "block_payload_codec_support/legacy_block_payload.rs"]
mod legacy_block_payload;
mod support;

use alloy_rlp::Decodable;
use commit_block_field::commit_block_field;
use eve_state::{
    BlockPayload, Bytes, Header, StateError, decode_block_payload, decode_state_commit,
    development_state_budget, encode_block_payload, encode_state_commit,
};
use legacy_block_payload::encode_legacy_block_payload;

#[test]
fn block_codec_roundtrips_existing_states_and_preserves_original_commit_block_wire() {
    let budget = development_state_budget();
    for commit in [support::genesis(), support::with_slots()] {
        let payload = encode_block_payload(&commit.block, &budget).unwrap();
        let original_wire = encode_legacy_block_payload(&commit.block);
        assert_eq!(payload, original_wire);
        assert_eq!(
            decode_block_payload(&payload, &budget).unwrap(),
            commit.block
        );
        let full_commit = encode_state_commit(&commit, &budget).unwrap();
        assert_eq!(commit_block_field(&full_commit), original_wire.as_slice());
        let restored = decode_state_commit(&full_commit, &budget).unwrap();
        assert_eq!(restored, commit);
        assert_eq!(
            encode_state_commit(&restored, &budget).unwrap(),
            full_commit
        );
    }
}

#[test]
fn block_codec_preserves_ordered_opaque_rows_without_execution_authority() {
    let budget = development_state_budget();
    let mut block = support::genesis().block;
    block.transactions = vec![Bytes::from_static(b"first"), Bytes::from_static(b"second")];
    block.receipts = vec![
        Bytes::from_static(b"receipt-one"),
        Bytes::from_static(b"receipt-two"),
    ];
    let encoded = encode_block_payload(&block, &budget).unwrap();
    assert_eq!(encoded, encode_legacy_block_payload(&block));
    assert_eq!(decode_block_payload(&encoded, &budget).unwrap(), block);
    block.transactions.swap(0, 1);
    block.receipts.swap(0, 1);
    assert_ne!(encode_block_payload(&block, &budget).unwrap(), encoded);
}

#[test]
fn block_codec_preserves_existing_literal_header_golden_wire() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../protocol-config/tests/fixtures/protocol-v1.json"
    ))
    .unwrap();
    let header_bytes = hex::decode(fixture["header_rlp"].as_str().unwrap()).unwrap();
    let mut header_input = header_bytes.as_slice();
    let header = Header::decode(&mut header_input).unwrap();
    assert!(header_input.is_empty());
    // Frozen 576-byte header, wrapped with two empty ordered lists: a wire vector,
    // not an execution-valid block or a certificate/finality claim.
    let block = BlockPayload {
        header,
        transactions: Vec::new(),
        receipts: Vec::new(),
    };
    let expected = [
        [0xf9, 0x02, 0x42].as_slice(),
        header_bytes.as_slice(),
        &[0xc0, 0xc0],
    ]
    .concat();
    let budget = development_state_budget();
    assert_eq!(encode_block_payload(&block, &budget).unwrap(), expected);
    assert_eq!(decode_block_payload(&expected, &budget).unwrap(), block);
}

#[test]
fn block_decoder_rejects_malformed_truncated_trailing_and_nonminimal_list_wire() {
    let budget = development_state_budget();
    let encoded = encode_block_payload(&support::genesis().block, &budget).unwrap();
    let mut trailing = encoded.clone();
    trailing.push(0);
    assert_eq!(
        decode_block_payload(&trailing, &budget),
        Err(StateError::MalformedEncoding)
    );
    assert!(decode_block_payload(&encoded[..encoded.len() - 1], &budget).is_err());
    assert!(decode_block_payload(&[0x80], &budget).is_err());
    assert!(decode_block_payload(&[0xf9, 0xff, 0xff], &budget).is_err());
    assert_eq!(encoded[0], 0xf9);
    let mut nonminimal = vec![0xfa, 0, encoded[1], encoded[2]];
    nonminimal.extend_from_slice(&encoded[3..]);
    assert!(decode_block_payload(&nonminimal, &budget).is_err());
}

#[test]
fn block_decoder_rejects_noncanonical_single_byte_payload_string() {
    let budget = development_state_budget();
    let mut block = support::genesis().block;
    // Opaque wire fixture only; transaction/receipt semantic validation is separate.
    block.transactions = vec![Bytes::from_static(&[0])];
    block.receipts = vec![Bytes::from_static(&[0])];
    let mut encoded = encode_block_payload(&block, &budget).unwrap();
    assert_eq!(encoded[0], 0xf9);
    assert!(encoded[2] < u8::MAX);
    let start = encoded.len() - 4;
    assert_eq!(&encoded[start..], &[0xc1, 0, 0xc1, 0]);
    encoded[2] += 1;
    encoded[start] = 0xc2;
    encoded.insert(start + 1, 0x81);
    assert!(decode_block_payload(&encoded, &budget).is_err());
}
