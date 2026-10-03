// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::wire::ImportWireError, check_logical_component_lengths::check_logical_component_lengths,
};
use crate::recovery::{
    bounds::{
        measure_execution_payload_bytes::measure_execution_payload_bytes,
        measure_native_frame_bytes::measure_native_frame_bytes,
        measure_transaction_list_bytes::measure_transaction_list_bytes,
    },
    import::AuthenticatedImportInput,
};
use eve_state::{StateBudget, measure_state_journal_bytes};

/// Exact complete size before large component/output Vecs. Canonical journal
/// measurement retains bounded version/system scratch that the caller must charge.
pub fn measure_logical_import_wire(
    input: &AuthenticatedImportInput,
    budget: &StateBudget,
) -> Result<usize, ImportWireError> {
    let journal =
        measure_state_journal_bytes(&input.journal, budget).map_err(ImportWireError::State)?;
    let execution =
        measure_execution_payload_bytes(&input.execution).map_err(ImportWireError::Recovery)?;
    if input.execution.transactions.len() > budget.maximum_journal_operations {
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
    check_logical_component_lengths([journal, execution, finalized, lookahead], budget)
}
