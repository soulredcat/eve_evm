// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_finality_verifier::{
    ImportError, decode_authenticated_import_wire, encode_authenticated_import_wire,
    import_wire_budget, import_wire_bytes, import_wire_slices, import_wire_stats,
    imported_state_commit, imported_transition_state, initialize_authenticated_import,
    measure_authenticated_import_wire, preflight_authenticated_import_wire,
    prepare_authenticated_import,
};
use eve_state::{B256, development_state_budget, encode_state_journal};

use super::support::DOMAIN;
use crate::recovery_import::support::input;
use crate::recovery_support::{self as support, CLONE_BYTES};

#[test]
fn honest_nonempty_signed_import_wire_roundtrips_and_prepares_the_same_canonical_outcome() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let source = input(&chain, 1);
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    assert!(bytes.starts_with(DOMAIN));
    assert_eq!(
        measure_authenticated_import_wire(&source, &budget).unwrap(),
        bytes.len()
    );
    let preflight = preflight_authenticated_import_wire(&bytes, &budget).unwrap();
    assert_eq!(import_wire_bytes(&preflight).as_ptr(), bytes.as_ptr());
    let slices = import_wire_slices(&preflight);
    assert_eq!(
        slices.journal,
        encode_state_journal(&source.journal, &budget).unwrap()
    );
    assert_eq!(slices.journal.as_ptr(), bytes[DOMAIN.len() + 4..].as_ptr());
    let stats = import_wire_stats(&preflight);
    assert_eq!(stats.encoded_bytes, bytes.len());
    assert_eq!(
        stats.journal.operation_count,
        source.journal.operations.len()
    );
    assert_eq!(stats.journal.encoded_bytes, slices.journal.len());
    assert_eq!(stats.execution.encoded_bytes, slices.execution.len());
    assert_eq!(stats.execution.transaction_count, 1);
    assert_eq!(
        stats.execution.transaction_bytes,
        source.execution.transactions[0].len()
    );
    assert_eq!(stats.execution.receipt_count, 1);
    assert_eq!(
        stats.execution.receipt_bytes,
        source.execution.receipts[0].len()
    );
    assert_eq!(
        stats.finalized.signature_count,
        source.finalized.commit.signatures.len()
    );
    assert_eq!(
        stats.lookahead.frame.signature_count,
        source.lookahead.frame.commit.signatures.len()
    );
    assert_eq!(stats.lookahead.transaction_count, 0);
    assert_eq!(stats.lookahead.transaction_bytes, 0);
    let decoded = decode_authenticated_import_wire(&preflight).unwrap();
    assert_eq!(decoded, source);
    assert_eq!(
        encode_authenticated_import_wire(&decoded, &budget).unwrap(),
        bytes
    );
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let transition =
        prepare_authenticated_import(&parent, Arc::new(decoded), &budget, CLONE_BYTES).unwrap();
    assert_eq!(
        imported_state_commit(imported_transition_state(&transition)),
        &chain.commits[1]
    );
}

#[test]
fn frozen_wire_budget_and_immutable_component_binding_survive_detached_copy_changes() {
    let chain = support::recovery_chain();
    let source = input(&chain, 1);
    let mut budget = development_state_budget();
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    let preflight = preflight_authenticated_import_wire(&bytes, &budget).unwrap();
    let original = budget;
    budget.maximum_journal_operations = 1;
    let mut detached = import_wire_slices(&preflight);
    detached.journal = b"substituted bytes";
    let mut stats = import_wire_stats(&preflight);
    stats.journal.operation_count = 0;
    assert_eq!(import_wire_budget(&preflight), original);
    assert_ne!(import_wire_budget(&preflight), budget);
    assert_ne!(import_wire_slices(&preflight).journal, detached.journal);
    assert_ne!(
        import_wire_stats(&preflight).journal.operation_count,
        stats.journal.operation_count
    );
    assert_eq!(
        decode_authenticated_import_wire(&preflight).unwrap(),
        source
    );
}

#[test]
fn wire_preserves_exact_auxiliary_parent_but_does_not_turn_it_into_import_authority() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let mut source = input(&chain, 1);
    source.journal.parent.content_digest = B256::repeat_byte(0x31);
    source.journal.parent.timestamp += 7;
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    let preflight = preflight_authenticated_import_wire(&bytes, &budget).unwrap();
    let decoded = decode_authenticated_import_wire(&preflight).unwrap();
    assert_eq!(decoded, source);
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    assert_eq!(
        prepare_authenticated_import(&parent, Arc::new(decoded), &budget, CLONE_BYTES).unwrap_err(),
        ImportError::WrongParent,
    );
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn ordered_lookahead_data_has_exact_count_bytes_and_distinct_canonical_wire_identity() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let mut source = input(&chain, 1);
    source.lookahead.transactions = vec![vec![0x11, 0x22].into(), vec![0x33].into()];
    let first = encode_authenticated_import_wire(&source, &budget).unwrap();
    let preflight = preflight_authenticated_import_wire(&first, &budget).unwrap();
    assert_eq!(import_wire_stats(&preflight).lookahead.transaction_count, 2);
    assert_eq!(import_wire_stats(&preflight).lookahead.transaction_bytes, 3);
    assert_eq!(
        decode_authenticated_import_wire(&preflight).unwrap(),
        source
    );
    source.lookahead.transactions.reverse();
    assert_ne!(
        encode_authenticated_import_wire(&source, &budget).unwrap(),
        first
    );
}
