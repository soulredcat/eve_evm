// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::PoolError;
pub(crate) fn replacement_threshold(previous: u128) -> Result<u128, PoolError> {
    let rise = previous / 10 + u128::from(!previous.is_multiple_of(10));
    previous
        .checked_add(rise.max(1))
        .ok_or_else(|| PoolError("replacement fee threshold overflow".into()))
}
