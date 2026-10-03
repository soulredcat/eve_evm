// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{import_support::input, recovery_support::recovery_chain};
use eve_finality_verifier::{
    ImportWireError, MAXIMUM_IMPORT_WIRE_BYTES, MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES,
    decode_logical_import_wire, encode_authenticated_import_wire, encode_logical_import_wire,
    logical_import_wire_stats, measure_logical_import_wire, preflight_logical_import_wire,
};
use eve_state::{B256, Bytes, JournalOperation, development_state_budget};

#[test]
fn full_journal_and_execution_components_exceed_compact_cap_without_raising_data_limits() {
    let mut source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    // Transport-only bytes exercise component admission and canonical decoding.
    // They claim no valid receipts/transactions, execution result, or certificate.
    source.journal.operations = vec![
        JournalOperation::PutCode {
            code_hash: B256::ZERO,
            code: Bytes::from(vec![0x60; 24_576]),
        };
        230
    ];
    source.execution.transactions = vec![Bytes::from(vec![0x11; 131_072]); 32];
    source.execution.receipts = vec![Bytes::new(); 32];
    source.execution.receipts[0] = Bytes::from(vec![0x22; 4_194_304]);
    source.lookahead.transactions = vec![Bytes::from(vec![0x33; 131_072]); 32];
    assert_eq!(
        encode_authenticated_import_wire(&source, &budget),
        Err(ImportWireError::BudgetExceeded)
    );
    let bytes = encode_logical_import_wire(&source, &budget).unwrap();
    assert!(bytes.len() > MAXIMUM_IMPORT_WIRE_BYTES);
    assert!(bytes.len() < MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES);
    assert_eq!(
        measure_logical_import_wire(&source, &budget).unwrap(),
        bytes.len()
    );
    let preflight = preflight_logical_import_wire(&bytes, &budget).unwrap();
    let stats = logical_import_wire_stats(&preflight);
    assert!(stats.journal.encoded_bytes > MAXIMUM_IMPORT_WIRE_BYTES);
    assert!(stats.execution.encoded_bytes > MAXIMUM_IMPORT_WIRE_BYTES);
    assert_eq!(stats.execution.transaction_bytes, 4_194_304);
    assert_eq!(stats.execution.receipt_bytes, 4_194_304);
    assert_eq!(
        (
            stats.execution.transaction_count,
            stats.execution.receipt_count
        ),
        (32, 32)
    );
    assert_eq!(stats.lookahead.transaction_bytes, 4_194_304);
    assert_eq!(decode_logical_import_wire(&preflight).unwrap(), source);
}
