// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::DOMAIN;
use crate::{
    import_support::input,
    recovery_support::{CLONE_BYTES, recovery_chain},
};
use eve_finality_verifier::{
    ImportError, decode_authenticated_import_wire, decode_logical_import_wire,
    encode_authenticated_import_wire, encode_logical_import_wire, imported_state_commit,
    imported_transition_state, initialize_authenticated_import, logical_import_wire_budget,
    logical_import_wire_bytes, logical_import_wire_slices, logical_import_wire_stats,
    measure_logical_import_wire, preflight_authenticated_import_wire,
    preflight_logical_import_wire, prepare_authenticated_import,
};
use eve_state::{B256, development_state_budget, encode_state_journal};
use std::sync::Arc;

#[test]
fn normal_v1_and_v2_decode_and_prepare_the_same_canonical_import() {
    let chain = recovery_chain();
    let source = input(&chain, 1);
    let budget = development_state_budget();
    let compact = encode_authenticated_import_wire(&source, &budget).unwrap();
    let logical = encode_logical_import_wire(&source, &budget).unwrap();
    assert_eq!(logical.len(), compact.len());
    assert!(logical.starts_with(DOMAIN));
    assert_eq!(&logical[DOMAIN.len()..], &compact[DOMAIN.len()..]);
    assert_eq!(
        measure_logical_import_wire(&source, &budget).unwrap(),
        logical.len()
    );
    let compact_input = decode_authenticated_import_wire(
        &preflight_authenticated_import_wire(&compact, &budget).unwrap(),
    )
    .unwrap();
    let preflight = preflight_logical_import_wire(&logical, &budget).unwrap();
    assert_eq!(
        logical_import_wire_bytes(&preflight).as_ptr(),
        logical.as_ptr()
    );
    let slices = logical_import_wire_slices(&preflight);
    assert_eq!(
        slices.journal,
        encode_state_journal(&source.journal, &budget).unwrap()
    );
    assert_eq!(
        slices.journal.as_ptr(),
        logical[DOMAIN.len() + 4..].as_ptr()
    );
    let stats = logical_import_wire_stats(&preflight);
    assert_eq!(stats.encoded_bytes, logical.len());
    assert_eq!(
        (
            stats.execution.transaction_count,
            stats.execution.receipt_count
        ),
        (1, 1)
    );
    let decoded = decode_logical_import_wire(&preflight).unwrap();
    assert_eq!(decoded, compact_input);
    assert_eq!(decoded, source);
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let transition =
        prepare_authenticated_import(&parent, Arc::new(decoded), &budget, CLONE_BYTES).unwrap();
    assert_eq!(
        imported_state_commit(imported_transition_state(&transition)),
        &chain.commits[1]
    );
}

#[test]
fn frozen_budget_and_bound_bytes_survive_detached_stats_and_slice_changes() {
    let source = input(&recovery_chain(), 1);
    let mut budget = development_state_budget();
    let original = budget;
    let bytes = encode_logical_import_wire(&source, &budget).unwrap();
    let preflight = preflight_logical_import_wire(&bytes, &budget).unwrap();
    budget.maximum_journal_operations = 0;
    let mut slices = logical_import_wire_slices(&preflight);
    slices.journal = b"substituted";
    let mut stats = logical_import_wire_stats(&preflight);
    stats.journal.operation_count = 0;
    assert_eq!(logical_import_wire_budget(&preflight), original);
    assert_ne!(logical_import_wire_budget(&preflight), budget);
    assert_ne!(
        logical_import_wire_slices(&preflight).journal,
        slices.journal
    );
    assert_ne!(
        logical_import_wire_stats(&preflight)
            .journal
            .operation_count,
        stats.journal.operation_count
    );
    assert_eq!(decode_logical_import_wire(&preflight).unwrap(), source);
}

#[test]
fn exact_auxiliary_parent_survives_wire_but_wrong_local_parent_still_rejects() {
    let chain = recovery_chain();
    let budget = development_state_budget();
    let mut source = input(&chain, 1);
    source.journal.parent.content_digest = B256::repeat_byte(0x31);
    source.journal.parent.timestamp += 7;
    let bytes = encode_logical_import_wire(&source, &budget).unwrap();
    let decoded =
        decode_logical_import_wire(&preflight_logical_import_wire(&bytes, &budget).unwrap())
            .unwrap();
    assert_eq!(decoded, source);
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    assert_eq!(
        prepare_authenticated_import(&parent, Arc::new(decoded), &budget, CLONE_BYTES).unwrap_err(),
        ImportError::WrongParent
    );
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}
