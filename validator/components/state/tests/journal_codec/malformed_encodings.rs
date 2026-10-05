// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_state::{
    Address, B256, StateError, decode_state_journal, development_state_budget,
    encode_state_journal, encode_system_record, preflight_state_journal,
};

#[test]
fn every_truncated_prefix_and_trailing_top_level_bytes_reject() {
    let budget = development_state_budget();
    let mut bytes = encode_state_journal(&fixtures::every_operation(), &budget).unwrap();
    for end in 0..bytes.len() {
        assert!(
            preflight_state_journal(&bytes[..end], &budget).is_err(),
            "prefix {end}"
        );
        assert!(
            decode_state_journal(&bytes[..end], &budget).is_err(),
            "prefix {end}"
        );
    }
    bytes.push(0x80);
    assert!(preflight_state_journal(&bytes, &budget).is_err());
    assert!(decode_state_journal(&bytes, &budget).is_err());
}

#[test]
fn unknown_tags_wrong_widths_missing_and_trailing_operation_fields_reject() {
    let address = alloy_rlp::encode(Address::ZERO);
    let cases = [
        fixtures::list(&[alloy_rlp::encode(0_u8)]),
        fixtures::list(&[alloy_rlp::encode(11_u8)]),
        fixtures::list(&[alloy_rlp::encode(2_u8)]),
        fixtures::list(&[
            alloy_rlp::encode(2_u8),
            alloy_rlp::encode([0_u8; 19].as_slice()),
        ]),
        fixtures::list(&[
            alloy_rlp::encode(2_u8),
            address.clone(),
            alloy_rlp::encode(1_u8),
        ]),
        fixtures::list(&[
            alloy_rlp::encode(7_u8),
            alloy_rlp::encode([0_u8; 31].as_slice()),
        ]),
        fixtures::list(&[
            alloy_rlp::encode(1_u8),
            address.clone(),
            alloy_rlp::encode([1_u8; 9].as_slice()),
            alloy_rlp::encode(0_u8),
            alloy_rlp::encode(B256::ZERO),
        ]),
        fixtures::list(&[
            alloy_rlp::encode(3_u8),
            address,
            alloy_rlp::encode([1_u8; 33].as_slice()),
            alloy_rlp::encode(0_u8),
        ]),
    ];
    let budget = development_state_budget();
    for operation in cases {
        let bytes = fixtures::raw_journal(&[operation]);
        assert!(preflight_state_journal(&bytes, &budget).is_err());
        assert!(decode_state_journal(&bytes, &budget).is_err());
    }
}

#[test]
fn nonminimal_integer_single_byte_and_list_length_forms_reject() {
    let address = alloy_rlp::encode(Address::ZERO);
    let cases = [
        fixtures::list(&[vec![0x81, 2], address.clone()]),
        fixtures::list(&[
            alloy_rlp::encode(1_u8),
            address.clone(),
            vec![0],
            alloy_rlp::encode(0_u8),
            alloy_rlp::encode(B256::ZERO),
        ]),
        fixtures::list(&[
            alloy_rlp::encode(1_u8),
            address.clone(),
            vec![0x82, 0, 1],
            alloy_rlp::encode(0_u8),
            alloy_rlp::encode(B256::ZERO),
        ]),
        fixtures::list(&[
            alloy_rlp::encode(6_u8),
            alloy_rlp::encode(B256::ZERO),
            vec![0x81, 1],
        ]),
    ];
    let budget = development_state_budget();
    for operation in cases {
        assert!(preflight_state_journal(&fixtures::raw_journal(&[operation]), &budget).is_err());
    }
    let canonical = fixtures::list(&[alloy_rlp::encode(2_u8), address]);
    let mut nonminimal = vec![0xf8, u8::try_from(canonical.len() - 1).unwrap()];
    nonminimal.extend_from_slice(&canonical[1..]);
    assert!(preflight_state_journal(&fixtures::raw_journal(&[nonminimal]), &budget).is_err());
}

#[test]
fn malformed_system_semantics_are_rejected_by_the_maintained_decoder() {
    let mut record = fixtures::parameter_record();
    let mut encoded = encode_system_record(&record).unwrap();
    // The first payload field is canonical schema version 1; version 2 is unsupported.
    let mut payload = encoded.as_slice();
    alloy_rlp::Header::decode(&mut payload).unwrap();
    let schema_offset = encoded.len() - payload.len();
    encoded[schema_offset] = 2;
    let operation = fixtures::list(&[
        alloy_rlp::encode(8_u8),
        alloy_rlp::encode(B256::ZERO),
        encoded,
    ]);
    let bytes = fixtures::raw_journal(&[operation]);
    let budget = development_state_budget();
    assert!(preflight_state_journal(&bytes, &budget).is_ok());
    assert!(decode_state_journal(&bytes, &budget).is_err());
    record.schema_version = 2;
    assert!(encode_system_record(&record).is_err());
}

#[test]
fn oversized_records_and_excessive_nested_lists_reject_without_recursive_scanning() {
    let budget = development_state_budget();
    let oversized = fixtures::list(&[alloy_rlp::encode(vec![0_u8; 4_097].as_slice())]);
    let mut nested = fixtures::list(&[alloy_rlp::encode(1_u8)]);
    for _ in 0..512 {
        nested = fixtures::list(&[nested]);
    }
    for record in [oversized, nested] {
        let operation = fixtures::list(&[
            alloy_rlp::encode(8_u8),
            alloy_rlp::encode(B256::ZERO),
            record,
        ]);
        let bytes = fixtures::raw_journal(&[operation]);
        assert!(preflight_state_journal(&bytes, &budget).is_err());
    }
    assert_eq!(
        preflight_state_journal(&[0xff; 9], &budget),
        Err(StateError::MalformedEncoding)
    );
}
