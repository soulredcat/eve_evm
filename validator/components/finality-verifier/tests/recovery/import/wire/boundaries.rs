// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_finality_verifier::{
    ImportWireError, MAXIMUM_IMPORT_WIRE_BYTES, decode_authenticated_import_wire,
    encode_authenticated_import_wire, measure_authenticated_import_wire,
    preflight_authenticated_import_wire,
};
use eve_state::{Bytes, development_state_budget, encode_block_payload};

use super::support::replace;
use crate::recovery_import::support::input;
use crate::recovery_support::recovery_chain;

#[test]
fn complete_compact_limit_accepts_exact_boundary_and_rejects_one_more_byte() {
    let mut source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    // Transport-only byte payloads exercise the exact limit; they claim no receipt
    // or certificate validity. Real signed/receipt import has a separate test.
    source.lookahead.transactions = vec![Bytes::from(vec![0x11; 131_072]); 16];
    source.execution.receipts[0] = vec![0x22; 1_048_576].into();
    let initial = measure_authenticated_import_wire(&source, &budget).unwrap();
    let receipt_length = 1_048_576 + MAXIMUM_IMPORT_WIRE_BYTES - initial;
    source.execution.receipts[0] = vec![0x22; receipt_length].into();
    assert_eq!(
        measure_authenticated_import_wire(&source, &budget).unwrap(),
        MAXIMUM_IMPORT_WIRE_BYTES
    );
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    assert_eq!(bytes.len(), MAXIMUM_IMPORT_WIRE_BYTES);
    let preflight = preflight_authenticated_import_wire(&bytes, &budget).unwrap();
    assert_eq!(
        decode_authenticated_import_wire(&preflight).unwrap(),
        source
    );
    source.execution.receipts[0] = vec![0x22; receipt_length + 1].into();
    assert_eq!(
        measure_authenticated_import_wire(&source, &budget),
        Err(ImportWireError::BudgetExceeded)
    );
    assert_eq!(
        encode_authenticated_import_wire(&source, &budget),
        Err(ImportWireError::BudgetExceeded)
    );
    let oversized = replace(&bytes, 1, |_| {
        encode_block_payload(&source.execution, &budget).unwrap()
    });
    assert_eq!(oversized.len(), MAXIMUM_IMPORT_WIRE_BYTES + 1);
    assert!(matches!(
        preflight_authenticated_import_wire(&oversized, &budget),
        Err(ImportWireError::BudgetExceeded)
    ));
}

#[test]
fn complete_aggregate_preflight_refuses_large_components_without_raising_record_cap() {
    let mut source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    source.lookahead.transactions = vec![Bytes::from(vec![0x11; 131_072]); 32];
    source.execution.receipts[0] = vec![0x22; 131_072].into();
    assert_eq!(
        measure_authenticated_import_wire(&source, &budget),
        Err(ImportWireError::BudgetExceeded)
    );
    assert_eq!(
        encode_authenticated_import_wire(&source, &budget),
        Err(ImportWireError::BudgetExceeded)
    );
}

#[test]
fn local_journal_and_execution_limits_are_enforced_before_any_decoded_input() {
    let source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_authenticated_import_wire(&source, &budget).unwrap();
    for field in 0..3 {
        let mut restricted = budget;
        match field {
            0 => restricted.maximum_journal_operations = 1,
            1 => restricted.maximum_journal_bytes = 1,
            2 => restricted.maximum_commit_bytes = 1,
            _ => unreachable!(),
        }
        assert!(preflight_authenticated_import_wire(&bytes, &restricted).is_err());
        assert!(encode_authenticated_import_wire(&source, &restricted).is_err());
    }
}
