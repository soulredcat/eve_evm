// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{DOMAIN, replace};
use crate::{import_support::input, recovery_support::recovery_chain};
use eve_finality_verifier::{
    encode_logical_import_wire, logical_import_wire_slices, preflight_authenticated_import_wire,
    preflight_logical_import_wire,
};
use eve_state::development_state_budget;

#[test]
fn every_truncated_prefix_trailing_bytes_and_wrong_transport_domain_reject() {
    let budget = development_state_budget();
    let mut bytes = encode_logical_import_wire(&input(&recovery_chain(), 1), &budget).unwrap();
    for end in 0..bytes.len() {
        assert!(
            preflight_logical_import_wire(&bytes[..end], &budget).is_err(),
            "prefix {end}"
        );
    }
    assert!(preflight_authenticated_import_wire(&bytes, &budget).is_err());
    bytes[DOMAIN.len() - 1] = b'1';
    assert!(preflight_logical_import_wire(&bytes, &budget).is_err());
    bytes[DOMAIN.len() - 1] = b'2';
    bytes.push(0);
    assert!(preflight_logical_import_wire(&bytes, &budget).is_err());
}

#[test]
fn reordered_components_and_truncated_native_fields_reject() {
    let source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_logical_import_wire(&source, &budget).unwrap();
    let preflight = preflight_logical_import_wire(&bytes, &budget).unwrap();
    let slices = logical_import_wire_slices(&preflight);
    let changed = replace(&bytes, 0, |_| slices.execution.to_vec());
    assert!(preflight_logical_import_wire(&changed, &budget).is_err());
    for field in [2, 3] {
        let changed = replace(&bytes, field, |component| {
            component[..component.len() - 1].to_vec()
        });
        assert!(preflight_logical_import_wire(&changed, &budget).is_err());
    }
}

#[test]
fn mismatched_receipt_counts_reject_in_borrowed_shared_execution_scan() {
    let source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_logical_import_wire(&source, &budget).unwrap();
    let changed = replace(&bytes, 1, |component| {
        let mut remaining = component;
        let mut fields = alloy_rlp::Header::decode_bytes(&mut remaining, true).unwrap();
        let header_start = fields;
        let header = alloy_rlp::Header::decode(&mut fields).unwrap();
        fields = &fields[header.payload_length..];
        let encoded_header = header_start[..header_start.len() - fields.len()].to_vec();
        let tx_start = fields;
        let transactions = alloy_rlp::Header::decode(&mut fields).unwrap();
        fields = &fields[transactions.payload_length..];
        let encoded_transactions = tx_start[..tx_start.len() - fields.len()].to_vec();
        let mut output = Vec::new();
        alloy_rlp::Header {
            list: true,
            payload_length: encoded_header.len() + encoded_transactions.len() + 1,
        }
        .encode(&mut output);
        output.extend_from_slice(&encoded_header);
        output.extend_from_slice(&encoded_transactions);
        output.push(0xc0);
        output
    });
    assert!(preflight_logical_import_wire(&changed, &budget).is_err());
}
