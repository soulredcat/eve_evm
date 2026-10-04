// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::archive_types::ArchiveMaintenance;
use crate::sync::applied::tests::import_fixtures::ImportChain;
use alloy_primitives::keccak256;
use eve_storage::state::{
    capture_history_snapshot, ensure_history_index, lookup_execution_hash, lookup_transaction,
    open_state_repository, read_history_block, state_reader,
};

/// Actual durable secondary-key lookups and exact version/transaction/receipt oracle comparisons.
pub(super) fn lookup_indexed_archive(
    archive: &ArchiveMaintenance,
    chain: &ImportChain,
) -> Result<(), String> {
    let mut repository =
        open_state_repository(&archive.index_path, &chain.commits[0], archive.state_budget)
            .map_err(|_| "maintenance indexed state reopen")?;
    let status = ensure_history_index(&mut repository, archive.history_budget)
        .map_err(|_| "maintenance canonical index readiness")?;
    if !status.complete || status.target_height != chain.commits.last().unwrap().target.height {
        return Err("maintenance index head mismatch".into());
    }
    let reader = state_reader(&repository);
    let snapshot = capture_history_snapshot(&reader, archive.history_budget)
        .map_err(|_| "maintenance indexed snapshot")?;
    let transaction = &chain.commits[1].block.transactions[0];
    let location = lookup_transaction(&snapshot, keccak256(transaction))
        .map_err(|_| "actual transaction secondary lookup")?
        .ok_or("transaction secondary key missing")?;
    if location.height != 1 || location.transaction_index != 0 {
        return Err("transaction secondary location mismatch".into());
    }
    for expected in &chain.commits {
        let height = lookup_execution_hash(&snapshot, expected.target.execution_hash.0)
            .map_err(|_| "actual block secondary lookup")?;
        if height != Some(expected.target.height) {
            return Err("block secondary location mismatch".into());
        }
        let retained = read_history_block(&snapshot, expected.target.height)
            .map_err(|_| "actual indexed block read")?
            .ok_or("indexed block missing")?;
        if retained.version != expected.target || retained.block != expected.block {
            return Err("indexed canonical version/transactions/receipts changed".into());
        }
    }
    Ok(())
}
