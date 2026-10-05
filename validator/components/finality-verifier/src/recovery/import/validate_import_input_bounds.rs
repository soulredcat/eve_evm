// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::StateBudget;

use super::{AuthenticatedImportInput, ImportError};
use crate::recovery::{
    RecoveryError,
    bounds::{
        measure_execution_payload_bytes::measure_execution_payload_bytes,
        measure_native_frame_bytes::measure_native_frame_bytes,
        measure_transaction_list_bytes::measure_transaction_list_bytes,
    },
};

/// Borrowed component admission before native copies. Enclosing wire/handoff bounds
/// remain caller-owned; the state reservation helper preflights journal growth.
pub(super) fn validate_import_input_bounds(
    input: &AuthenticatedImportInput,
    budget: &StateBudget,
) -> Result<(), ImportError> {
    if input.journal.operations.len() > budget.maximum_journal_operations {
        return Err(ImportError::Recovery(RecoveryError::BudgetExceeded));
    }
    measure_execution_payload_bytes(&input.execution).map_err(ImportError::Recovery)?;
    measure_native_frame_bytes(&input.finalized).map_err(ImportError::Recovery)?;
    measure_native_frame_bytes(&input.lookahead.frame).map_err(ImportError::Recovery)?;
    measure_transaction_list_bytes(&input.lookahead.transactions).map_err(ImportError::Recovery)?;
    Ok(())
}
