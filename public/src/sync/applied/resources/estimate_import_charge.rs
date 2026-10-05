// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::EstimatedImportCharge;
use crate::sync::applied::AppliedError;
use eve_finality_verifier::ImportWireStats;
use eve_state::Bytes;

/// Actual wire counts and canonical candidate estimate; never a REVM/max-StateBudget charge.
/// Logical estimates include codec/proof copies and retained block/lookahead allocations.
pub(in crate::sync::applied) fn estimate_import_charge(
    stats: ImportWireStats,
    candidate: usize,
) -> Result<EstimatedImportCharge, AppliedError> {
    let journal = stats.journal;
    let vectors = stats
        .execution
        .transaction_count
        .checked_add(stats.execution.receipt_count)
        .and_then(|count| count.checked_add(stats.lookahead.transaction_count))
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let native = stats
        .finalized
        .encoded_bytes
        .checked_add(stats.lookahead.frame.encoded_bytes)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let signatures = stats
        .finalized
        .signature_count
        .checked_add(stats.lookahead.frame.signature_count)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let raw = stats
        .execution
        .transaction_bytes
        .checked_add(stats.execution.receipt_bytes)
        .and_then(|bytes| bytes.checked_add(stats.lookahead.transaction_bytes))
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let retained = [
        (candidate, 1),
        (raw, 1),
        (vectors, 2 * std::mem::size_of::<Bytes>()),
        (native, 8),
        (signatures, 1_024),
    ]
    .into_iter()
    .try_fold(131_072_usize, |sum, (count, factor)| {
        sum.checked_add(count.checked_mul(factor)?)
    })
    .ok_or(AppliedError::ArithmeticOverflow)?;
    let decoded_wire = stats
        .execution
        .encoded_bytes
        .checked_add(native)
        .and_then(|bytes| bytes.checked_add(stats.lookahead.encoded_bytes))
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let native_transactions = stats
        .execution
        .transaction_count
        .checked_add(stats.lookahead.transaction_count)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let native_bytes = stats
        .execution
        .transaction_bytes
        .checked_add(stats.lookahead.transaction_bytes)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let terms = [
        (candidate, 1),
        (retained, 1),
        (journal.operation_allocation_bytes, 1),
        (journal.code_bytes, 1),
        (journal.system_payload_bytes, 1),
        (journal.system_leaf_count, 2 * std::mem::size_of::<Bytes>()),
        (journal.parent_network_name_bytes, 2),
        (journal.conservative_codec_scratch_bytes, 1),
        (journal.code_operations, 64),
        (decoded_wire, 8),
        (vectors, 2 * std::mem::size_of::<Bytes>()),
        (native_bytes, 1),
        (native_transactions, std::mem::size_of::<Vec<u8>>()),
        (native, 8),
        (signatures, 1_024),
    ];
    let total = terms
        .into_iter()
        .try_fold(262_144_usize, |sum, (count, factor)| {
            sum.checked_add(count.checked_mul(factor)?)
        })
        .ok_or(AppliedError::ArithmeticOverflow)?;
    Ok(EstimatedImportCharge {
        candidate,
        retained,
        total,
    })
}
