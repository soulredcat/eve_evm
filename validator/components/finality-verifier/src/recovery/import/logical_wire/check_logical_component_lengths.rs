// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::wire::ImportWireError,
    types::{
        LOGICAL_IMPORT_DOMAIN, MAXIMUM_LOGICAL_EXECUTION_BYTES, MAXIMUM_LOGICAL_FINALIZED_BYTES,
        MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES, MAXIMUM_LOGICAL_JOURNAL_BYTES,
        MAXIMUM_LOGICAL_LOOKAHEAD_BYTES,
    },
};
use eve_state::StateBudget;

pub(super) fn check_logical_component_lengths(
    lengths: [usize; 4],
    budget: &StateBudget,
) -> Result<usize, ImportWireError> {
    let limits = [
        budget
            .maximum_journal_bytes
            .min(MAXIMUM_LOGICAL_JOURNAL_BYTES),
        budget
            .maximum_commit_bytes
            .min(MAXIMUM_LOGICAL_EXECUTION_BYTES),
        MAXIMUM_LOGICAL_FINALIZED_BYTES,
        MAXIMUM_LOGICAL_LOOKAHEAD_BYTES,
    ];
    let mut total = LOGICAL_IMPORT_DOMAIN.len() + 16;
    for (length, limit) in lengths.into_iter().zip(limits) {
        if length > limit || u32::try_from(length).is_err() {
            return Err(ImportWireError::BudgetExceeded);
        }
        total = total
            .checked_add(length)
            .ok_or(ImportWireError::BudgetExceeded)?;
    }
    if total > MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES {
        return Err(ImportWireError::BudgetExceeded);
    }
    Ok(total)
}
