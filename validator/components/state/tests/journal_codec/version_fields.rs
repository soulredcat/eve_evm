// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_state::{B256, development_state_budget, encode_state_version, preflight_state_journal};

fn fields(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut remaining = bytes;
    let mut list = alloy_rlp::Header::decode_bytes(&mut remaining, true).unwrap();
    assert!(remaining.is_empty());
    let mut fields = Vec::new();
    while !list.is_empty() {
        let before = list;
        let header = alloy_rlp::Header::decode(&mut list).unwrap();
        list = &list[header.payload_length..];
        fields.push(before[..before.len() - list.len()].to_vec());
    }
    fields
}

fn journal_with_parent(parent: Vec<u8>) -> Vec<u8> {
    fixtures::list(&[
        alloy_rlp::encode(b"EVE_STATE_JOURNAL_V1".as_slice()),
        parent,
        alloy_rlp::encode(1_u64),
        fixtures::list(&[]),
    ])
}

#[test]
fn borrowed_parent_scan_rejects_network_encoding_identity_widths_and_profile_tags() {
    let canonical = encode_state_version(&super::support::genesis().target).unwrap();
    let version = fields(&canonical);
    let identity = fields(&version[0]);
    let cases = [
        (0, alloy_rlp::encode([0_u8; 31].as_slice())),
        (1, alloy_rlp::encode([0xff_u8].as_slice())),
        (1, alloy_rlp::encode([b'a'; 65].as_slice())),
        (2, alloy_rlp::encode([1_u8; 9].as_slice())),
        (3, alloy_rlp::encode([1_u8; 5].as_slice())),
        (4, alloy_rlp::encode(0_u8)),
        (5, vec![0x82, 0, 1]),
        (6, alloy_rlp::encode([0_u8; 33].as_slice())),
    ];
    for (index, value) in cases {
        let mut identity = identity.clone();
        identity[index] = value;
        let mut version = version.clone();
        version[0] = fixtures::list(&identity);
        let bytes = journal_with_parent(fixtures::list(&version));
        assert!(preflight_state_journal(&bytes, &development_state_budget()).is_err());
    }
}

#[test]
fn borrowed_parent_scan_rejects_optional_tags_widths_and_extra_fields() {
    let canonical = encode_state_version(&super::support::genesis().target).unwrap();
    let fields = fields(&canonical);
    let optionals = [
        fixtures::list(&[alloy_rlp::encode(2_u8)]),
        fixtures::list(&[alloy_rlp::encode(1_u8)]),
        fixtures::list(&[
            alloy_rlp::encode(1_u8),
            alloy_rlp::encode([0_u8; 31].as_slice()),
        ]),
        fixtures::list(&[alloy_rlp::encode(0_u8), alloy_rlp::encode(B256::ZERO)]),
    ];
    let budget = development_state_budget();
    for optional in optionals {
        let mut version = fields.clone();
        version[6] = optional;
        assert!(
            preflight_state_journal(&journal_with_parent(fixtures::list(&version)), &budget)
                .is_err()
        );
    }
    let mut version = fields;
    version.push(alloy_rlp::encode(0_u8));
    assert!(
        preflight_state_journal(&journal_with_parent(fixtures::list(&version)), &budget).is_err()
    );
}

#[test]
fn schema_marker_wrong_root_shape_and_extra_journal_fields_reject() {
    let canonical = fixtures::raw_journal(&[]);
    let budget = development_state_budget();
    let mut top = fields(&canonical);
    top[0] = alloy_rlp::encode(b"EVE_STATE_JOURNAL_V2".as_slice());
    assert!(preflight_state_journal(&fixtures::list(&top), &budget).is_err());
    top = fields(&canonical);
    top.push(alloy_rlp::encode(0_u8));
    assert!(preflight_state_journal(&fixtures::list(&top), &budget).is_err());
    assert!(preflight_state_journal(&alloy_rlp::encode(canonical.as_slice()), &budget).is_err());
}
