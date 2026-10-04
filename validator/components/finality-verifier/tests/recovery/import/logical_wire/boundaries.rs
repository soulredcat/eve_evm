// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{DOMAIN, replace};
use crate::{import_support::input, recovery_support::recovery_chain};
use eve_finality_verifier::{
    ImportWireError, MAXIMUM_IMPORT_WIRE_BYTES, MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES, RecoveryError,
    encode_logical_import_wire, measure_logical_import_wire, preflight_logical_import_wire,
};
use eve_state::{Bytes, development_state_budget};

#[test]
fn exact_aggregate_upper_gate_and_next_byte_are_distinct_without_claiming_valid_maxima() {
    assert_eq!(DOMAIN.len(), 13);
    assert_eq!(MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES, 21_025_569);
    assert_eq!(MAXIMUM_IMPORT_WIRE_BYTES, 4_198_312);
    let budget = development_state_budget();
    let mut bytes = vec![0; MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES];
    bytes[..DOMAIN.len()].copy_from_slice(DOMAIN);
    // Empty component declarations reject; the exact-size buffer passes only the
    // enclosing length gate. No simultaneous execution-valid maximum is claimed.
    assert!(matches!(
        preflight_logical_import_wire(&bytes, &budget),
        Err(ImportWireError::NonCanonicalEncoding)
    ));
    bytes.push(0);
    assert!(matches!(
        preflight_logical_import_wire(&bytes, &budget),
        Err(ImportWireError::BudgetExceeded)
    ));
}

#[test]
fn each_component_ceiling_and_overflowing_declared_lengths_reject_before_decode() {
    let source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_logical_import_wire(&source, &budget).unwrap();
    for (field, ceiling) in [8_388_608, 8_404_140, 16_384, 4_216_408]
        .into_iter()
        .enumerate()
    {
        let oversized = replace(&bytes, field, |_| vec![0; ceiling + 1]);
        assert!(matches!(
            preflight_logical_import_wire(&oversized, &budget),
            Err(ImportWireError::Recovery(RecoveryError::BudgetExceeded))
        ));
    }
    let mut overflowing = DOMAIN.to_vec();
    overflowing.extend_from_slice(&u32::MAX.to_be_bytes());
    assert!(matches!(
        preflight_logical_import_wire(&overflowing, &budget),
        Err(ImportWireError::Recovery(RecoveryError::BudgetExceeded))
    ));
}

#[test]
fn local_count_and_byte_policies_remain_effective_for_v2() {
    let source = input(&recovery_chain(), 1);
    let budget = development_state_budget();
    let bytes = encode_logical_import_wire(&source, &budget).unwrap();
    for field in 0..3 {
        let mut local = budget;
        match field {
            0 => local.maximum_journal_operations = 0,
            1 => local.maximum_journal_bytes = 1,
            2 => local.maximum_commit_bytes = 1,
            _ => unreachable!(),
        }
        assert!(preflight_logical_import_wire(&bytes, &local).is_err());
        assert!(measure_logical_import_wire(&source, &local).is_err());
        assert!(encode_logical_import_wire(&source, &local).is_err());
    }
}

#[test]
fn raw_transaction_and_receipt_limits_are_preserved_in_full_transport() {
    let budget = development_state_budget();
    let mut source = input(&recovery_chain(), 1);
    source.execution.transactions[0] = Bytes::from(vec![0x11; 131_073]);
    assert!(measure_logical_import_wire(&source, &budget).is_err());
    source = input(&recovery_chain(), 1);
    source.execution.receipts[0] = Bytes::from(vec![0x22; 4_194_305]);
    assert!(measure_logical_import_wire(&source, &budget).is_err());
    source = input(&recovery_chain(), 1);
    source.lookahead.transactions = vec![Bytes::from(vec![0x33; 131_072]); 33];
    assert!(measure_logical_import_wire(&source, &budget).is_err());
}
