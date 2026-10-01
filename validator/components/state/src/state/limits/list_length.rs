// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::StateError;

pub(crate) fn list_length(payload: usize) -> Result<usize, StateError> {
    payload
        .checked_add(alloy_rlp::length_of_length(payload))
        .ok_or(StateError::ArithmeticOverflow)
}
