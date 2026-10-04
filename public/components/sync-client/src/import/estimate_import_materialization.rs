// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result};
use eve_state::{JournalOperation, StateDeltaPayloadStats};

/// Actual borrowed input counts plus complete canonical reencoding copies. No
/// execution candidate is constructed here; applied service charges that separately.
pub(super) fn estimate_import_materialization(stats: StateDeltaPayloadStats) -> Result<usize> {
    let vectors = stats
        .execution
        .transaction_count
        .checked_add(stats.execution.receipt_count)
        .context("FOLLOWER_MATERIALIZATION_OVERFLOW")?;
    let terms = [
        (stats.encoded_bytes, 4),
        (stats.journal.operation_allocation_bytes, 2),
        (stats.journal.code_bytes, 2),
        (stats.journal.system_payload_bytes, 2),
        (stats.journal.system_leaf_count, 64),
        (
            stats.journal.operation_count,
            2 * std::mem::size_of::<JournalOperation>(),
        ),
        (vectors, 128),
        (stats.execution.encoded_bytes, 4),
    ];
    terms
        .into_iter()
        .try_fold(262_144_usize, |sum, (count, factor)| {
            sum.checked_add(count.checked_mul(factor)?)
        })
        .context("FOLLOWER_MATERIALIZATION_OVERFLOW")
}
