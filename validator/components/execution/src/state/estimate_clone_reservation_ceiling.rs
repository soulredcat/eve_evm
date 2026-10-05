// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CompleteExecutionError, calculate_clone_reservation::calculate_clone_reservation};
use eve_state::StateBudget;

/// Configuration ceiling for the same logical clone model used on actual state.
/// This does not reserve memory or bound execution growth, allocator capacity or RSS.
pub fn estimate_clone_reservation_ceiling(
    budget: &StateBudget,
) -> Result<usize, CompleteExecutionError> {
    calculate_clone_reservation(
        budget.maximum_accounts,
        budget.maximum_storage_slots,
        budget.maximum_total_code_bytes,
        budget.maximum_codes,
        budget.maximum_block_hashes,
    )
}
