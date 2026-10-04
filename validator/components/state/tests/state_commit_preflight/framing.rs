// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::fixtures::{fields, fixture, list};
use eve_state::{
    StateError, decode_preflight_state_commit, preflight_state_commit, state_commit_preflight_stats,
};

#[test]
fn truncated_trailing_schema_and_parent_flag_inputs_refuse_borrowed_admission() {
    let (_, bytes, budget) = fixture();
    for cut in [0, 1, bytes.len() / 2, bytes.len() - 1] {
        assert!(preflight_state_commit(&bytes[..cut], &budget).is_err());
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(preflight_state_commit(&trailing, &budget).is_err());
    let mut commit = fields(&bytes);
    commit[0] = alloy_rlp::encode(b"unknown-commit-domain".as_slice());
    assert!(matches!(
        preflight_state_commit(&list(&commit), &budget),
        Err(StateError::InvalidSchema)
    ));
    let mut commit = fields(&bytes);
    commit[1] = list(&[alloy_rlp::encode(2_u8)]);
    assert!(preflight_state_commit(&list(&commit), &budget).is_err());
}

#[test]
fn malformed_fixed_width_and_utf8_identity_refuse_before_string_or_map_materialization() {
    let (_, bytes, budget) = fixture();
    let mut commit = fields(&bytes);
    let mut state = fields(&commit[3]);
    let mut identity = fields(&state[0]);
    identity[0] = alloy_rlp::encode([0x11_u8; 31].as_slice());
    state[0] = list(&identity);
    commit[3] = list(&state);
    assert!(preflight_state_commit(&list(&commit), &budget).is_err());
    let mut commit = fields(&bytes);
    let mut state = fields(&commit[3]);
    let mut identity = fields(&state[0]);
    identity[1] = alloy_rlp::encode([0xff_u8].as_slice());
    state[0] = list(&identity);
    commit[3] = list(&state);
    assert!(preflight_state_commit(&list(&commit), &budget).is_err());
}

#[test]
fn execution_byte_lists_report_actual_counts_and_mismatched_receipts_refuse_before_decode() {
    let (_, bytes, budget) = fixture();
    let mut commit = fields(&bytes);
    let mut block = fields(&commit[4]);
    block[1] = list(&[
        alloy_rlp::encode([0xc0_u8].as_slice()),
        alloy_rlp::encode([0x01_u8, 0x02].as_slice()),
    ]);
    block[2] = list(&[
        alloy_rlp::encode([0xc0_u8].as_slice()),
        alloy_rlp::encode([0xc0_u8].as_slice()),
    ]);
    commit[4] = list(&block);
    let altered = list(&commit);
    let preflight = preflight_state_commit(&altered, &budget).unwrap();
    let stats = state_commit_preflight_stats(&preflight);
    assert_eq!(
        (
            stats.transaction_count,
            stats.transaction_bytes,
            stats.receipt_count,
            stats.receipt_bytes
        ),
        (2, 3, 2, 2)
    );
    // These are merely structurally bounded bytes, not valid transaction/receipt or roots.
    assert!(decode_preflight_state_commit(&preflight).is_err());
    block[2] = list(&[alloy_rlp::encode([0xc0_u8].as_slice())]);
    commit[4] = list(&block);
    assert!(preflight_state_commit(&list(&commit), &budget).is_err());
}

#[test]
fn oversize_header_and_nested_system_record_never_enter_owned_materialization() {
    let (_, bytes, budget) = fixture();
    let mut commit = fields(&bytes);
    let mut block = fields(&commit[4]);
    let mut header = fields(&block[0]);
    header[12] = alloy_rlp::encode(vec![0x31_u8; 4_097].as_slice());
    block[0] = list(&header);
    commit[4] = list(&block);
    assert!(preflight_state_commit(&list(&commit), &budget).is_err());
    let mut commit = fields(&bytes);
    let mut state = fields(&commit[3]);
    let mut records = fields(&state[3]);
    let mut entry = fields(&records[0]);
    entry[1] = list(&[list(&[list(&[list(&[])])])]);
    records[0] = list(&entry);
    state[3] = list(&records);
    commit[3] = list(&state);
    assert!(preflight_state_commit(&list(&commit), &budget).is_err());
}
