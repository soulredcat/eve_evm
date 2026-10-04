// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::persistence::segmented::SegmentedError;

/// Per-record pacing without idle credit. Ceil arithmetic avoids rounding above the budget.
pub(super) fn required_worker_cpu_pacing_ns(
    cpu_ns: u64,
    wall_ns: u64,
    basis_points: u64,
) -> Result<u64, SegmentedError> {
    if basis_points == 0 || basis_points > 10_000 {
        return Err(SegmentedError::InvalidConfiguration);
    }
    let target = u128::from(cpu_ns)
        .checked_mul(10_000)
        .and_then(|value| value.checked_add(u128::from(basis_points - 1)))
        .ok_or(SegmentedError::Overflow)?
        / u128::from(basis_points);
    u64::try_from(target.saturating_sub(u128::from(wall_ns))).map_err(|_| SegmentedError::Overflow)
}
