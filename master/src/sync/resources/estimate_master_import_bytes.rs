// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::Result;
use eve_finality_verifier::ImportWireStats;
use eve_state::{Bytes, JournalOperation};

/// Count-derived logical decode/reencode/proof/candidate envelope, retained conservatively
/// with the resulting private capability. This does not implement protocol validation.
pub(in crate::sync) fn estimate_master_import_bytes(
    stats: ImportWireStats,
    candidate: usize,
) -> Result<usize> {
    let journal = stats.journal;
    let vectors = stats
        .execution
        .transaction_count
        .checked_add(stats.execution.receipt_count)
        .and_then(|count| count.checked_add(stats.lookahead.transaction_count))
        .ok_or_else(|| anyhow::anyhow!("MASTER_IMPORT_RESOURCE_ARITHMETIC"))?;
    let signatures = stats
        .finalized
        .signature_count
        .checked_add(stats.lookahead.frame.signature_count)
        .ok_or_else(|| anyhow::anyhow!("MASTER_IMPORT_RESOURCE_ARITHMETIC"))?;
    let native = stats
        .finalized
        .encoded_bytes
        .checked_add(stats.lookahead.frame.encoded_bytes)
        .ok_or_else(|| anyhow::anyhow!("MASTER_IMPORT_RESOURCE_ARITHMETIC"))?;
    let terms = [
        (candidate, 2),
        (stats.encoded_bytes, 12),
        (journal.operation_allocation_bytes, 2),
        (
            journal.operation_count,
            2 * std::mem::size_of::<JournalOperation>(),
        ),
        (journal.code_bytes, 4),
        (journal.system_payload_bytes, 4),
        (journal.system_leaf_count, 4 * std::mem::size_of::<Bytes>()),
        (journal.parent_network_name_bytes, 4),
        (journal.conservative_codec_scratch_bytes, 2),
        (vectors, 4 * std::mem::size_of::<Bytes>()),
        (native, 16),
        (signatures, 2_048),
    ];
    terms
        .into_iter()
        .try_fold(2_097_152_usize, |sum, (count, factor)| {
            sum.checked_add(count.checked_mul(factor)?)
        })
        .ok_or_else(|| anyhow::anyhow!("MASTER_IMPORT_RESOURCE_ARITHMETIC"))
}
