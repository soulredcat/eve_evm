// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedError;
use eve_evm::estimate_clone_reservation_ceiling;
use eve_state::StateBudget;

/// Configuration-only ceiling owned by the canonical executor.
/// Replay still calls that estimator on its actual parent; no fake state is materialized.
pub(in crate::sync::applied) fn estimated_clone_ceiling(
    budget: &StateBudget,
) -> Result<usize, AppliedError> {
    estimate_clone_reservation_ceiling(budget).map_err(|_| AppliedError::ArithmeticOverflow)
}
