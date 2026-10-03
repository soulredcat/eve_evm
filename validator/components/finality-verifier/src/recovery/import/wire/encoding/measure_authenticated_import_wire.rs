// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{StateBudget, measure_state_journal_bytes};

use super::super::{ImportWireError, MAXIMUM_IMPORT_WIRE_BYTES, types::IMPORT_DOMAIN};
use crate::recovery::{
    bounds::{
        measure_execution_payload_bytes::measure_execution_payload_bytes,
        measure_native_frame_bytes::measure_native_frame_bytes,
        measure_transaction_list_bytes::measure_transaction_list_bytes,
    },
    import::AuthenticatedImportInput,
};

/// Exact complete size before whole-component Vecs. Canonical state measurement
/// still uses bounded version/system field scratch; the caller must charge it.
pub fn measure_authenticated_import_wire(
    input: &AuthenticatedImportInput,
    budget: &StateBudget,
) -> Result<usize, ImportWireError> {
    let journal =
        measure_state_journal_bytes(&input.journal, budget).map_err(ImportWireError::State)?;
    let execution =
        measure_execution_payload_bytes(&input.execution).map_err(ImportWireError::Recovery)?;
    if execution > budget.maximum_commit_bytes
        || input.execution.transactions.len() > budget.maximum_journal_operations
    {
        return Err(ImportWireError::BudgetExceeded);
    }
    let finalized =
        measure_native_frame_bytes(&input.finalized).map_err(ImportWireError::Recovery)?;
    let lookahead = measure_native_frame_bytes(&input.lookahead.frame)
        .map_err(ImportWireError::Recovery)?
        .checked_add(
            measure_transaction_list_bytes(&input.lookahead.transactions)
                .map_err(ImportWireError::Recovery)?,
        )
        .and_then(|bytes| bytes.checked_add(4))
        .ok_or(ImportWireError::BudgetExceeded)?;
    let total = [journal, execution, finalized, lookahead]
        .into_iter()
        .try_fold(IMPORT_DOMAIN.len() + 16, |sum, field| {
            sum.checked_add(field)
        })
        .ok_or(ImportWireError::BudgetExceeded)?;
    if total > MAXIMUM_IMPORT_WIRE_BYTES {
        return Err(ImportWireError::BudgetExceeded);
    }
    Ok(total)
}
