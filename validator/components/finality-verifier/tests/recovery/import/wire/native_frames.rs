// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_consensus_comet::consensus::certificates::MAX_DEVELOPMENT_VALIDATORS;
use eve_finality_verifier::{
    decode_authenticated_import_wire, encode_authenticated_import_wire, imported_state_commit,
    initialize_authenticated_import, preflight_authenticated_import_wire,
    prepare_authenticated_import,
};
use eve_state::development_state_budget;
use prost::Message;

use super::support::replace;
use crate::recovery_import::support::input;
use crate::recovery_support::{self as support, CLONE_BYTES};

#[test]
fn native_unknown_duplicate_or_nonminimal_fields_are_rejected_by_existing_scanner() {
    let source = input(&support::recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    for suffix in [vec![0xf8, 0x07, 0x01], vec![0x12, 0x01, 0x61], vec![0x80]] {
        let changed = replace(&bytes, 2, |frame| {
            crate::codec_mutation::replace_field(frame, 0, 1, |header| {
                let mut output = header.to_vec();
                output.extend_from_slice(&suffix);
                output
            })
        });
        assert!(preflight_authenticated_import_wire(&changed, &budget).is_err());
    }
}

#[test]
fn excessive_native_signatures_and_lookahead_counts_reject_before_materialization() {
    let mut source = input(&support::recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    source.finalized.commit.signatures =
        vec![source.finalized.commit.signatures[0].clone(); MAX_DEVELOPMENT_VALIDATORS + 1];
    let changed = replace(&bytes, 2, |frame| {
        crate::codec_mutation::replace_field(frame, 0, 2, |_| {
            source.finalized.commit.encode_to_vec()
        })
    });
    assert!(preflight_authenticated_import_wire(&changed, &budget).is_err());
    assert!(encode_authenticated_import_wire(&source, &budget).is_err());
    let changed = replace(&bytes, 3, |lookahead| {
        let mut output = lookahead.to_vec();
        let frame_length = u32::from_be_bytes(output[..4].try_into().unwrap()) as usize;
        output[4 + frame_length..8 + frame_length].copy_from_slice(&u32::MAX.to_be_bytes());
        output
    });
    assert!(preflight_authenticated_import_wire(&changed, &budget).is_err());
}

#[test]
fn canonical_transport_bytes_do_not_authenticate_altered_certificate_or_native_data() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    for field in 0..2 {
        let mut source = input(&chain, 1);
        if field == 0 {
            source.finalized.commit.signatures[0].signature[0] ^= 1;
        } else {
            source.lookahead.transactions.push(vec![0x11].into());
        }
        let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
        let preflight = preflight_authenticated_import_wire(&bytes, &budget).unwrap();
        let decoded = decode_authenticated_import_wire(&preflight).unwrap();
        assert_eq!(decoded, source);
        assert!(
            prepare_authenticated_import(&parent, Arc::new(decoded), &budget, CLONE_BYTES).is_err()
        );
        assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
    }
}
