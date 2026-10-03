// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_finality_verifier::{
    decode_authenticated_import_wire, encode_authenticated_import_wire,
    preflight_authenticated_import_wire,
};
use eve_state::development_state_budget;

use super::support::{DOMAIN, journal_with_operation, replace, rlp_list};
use crate::recovery_import::support::input;
use crate::recovery_support::recovery_chain;

#[test]
fn import_wire_rejects_wrong_domain_every_truncation_and_trailing_bytes() {
    let source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    for length in 0..bytes.len() {
        assert!(preflight_authenticated_import_wire(&bytes[..length], &budget).is_err());
    }
    let mut changed = bytes.clone();
    changed[0] ^= 1;
    assert!(preflight_authenticated_import_wire(&changed, &budget).is_err());
    changed = bytes.clone();
    changed.extend_from_slice(&[0]);
    assert!(preflight_authenticated_import_wire(&changed, &budget).is_err());
    changed = bytes;
    changed[DOMAIN.len()..DOMAIN.len() + 4].copy_from_slice(&u32::MAX.to_be_bytes());
    assert!(preflight_authenticated_import_wire(&changed, &budget).is_err());
}

#[test]
fn journal_unknown_tags_nonminimal_tag_and_wrong_domain_are_rejected_before_decode() {
    let source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    let unknown = journal_with_operation(
        &source.journal.parent,
        rlp_list(&[alloy_rlp::encode(11_u8)]),
    );
    let nonminimal = journal_with_operation(&source.journal.parent, rlp_list(&[vec![0x81, 0x01]]));
    for journal in [unknown, nonminimal] {
        let changed = replace(&bytes, 0, |_| journal);
        assert!(preflight_authenticated_import_wire(&changed, &budget).is_err());
    }
    let changed = replace(&bytes, 0, |journal| {
        let mut output = journal.to_vec();
        let offset = output
            .windows(b"EVE_STATE_JOURNAL_V1".len())
            .position(|window| window == b"EVE_STATE_JOURNAL_V1")
            .unwrap();
        output[offset] ^= 1;
        output
    });
    assert!(preflight_authenticated_import_wire(&changed, &budget).is_err());
}

#[test]
fn each_nested_component_must_be_complete_and_canonical() {
    let source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    for field in 0..4 {
        let changed = replace(&bytes, field, |component| {
            component[..component.len() - 1].to_vec()
        });
        assert!(preflight_authenticated_import_wire(&changed, &budget).is_err());
        let changed = replace(&bytes, field, |component| {
            let mut output = component.to_vec();
            output.push(0);
            output
        });
        assert!(preflight_authenticated_import_wire(&changed, &budget).is_err());
    }
}

#[test]
fn bounded_component_preflight_does_not_claim_a_complete_typed_header_is_valid() {
    let source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    let transactions = source
        .execution
        .transactions
        .iter()
        .map(|bytes| alloy_rlp::encode(bytes.as_ref()))
        .collect::<Vec<_>>();
    let receipts = source
        .execution
        .receipts
        .iter()
        .map(|bytes| alloy_rlp::encode(bytes.as_ref()))
        .collect::<Vec<_>>();
    let changed = replace(&bytes, 1, |_| {
        rlp_list(&[vec![0xc0], rlp_list(&transactions), rlp_list(&receipts)])
    });
    let preflight = preflight_authenticated_import_wire(&changed, &budget).unwrap();
    assert!(decode_authenticated_import_wire(&preflight).is_err());
}
