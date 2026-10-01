// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateProofError;
use crate::{StateView, view_state};

/// Conservative logical charge for sorted leaves and retained account/slot paths.
/// Existing captured state and caller buffers require separate reservations.
pub fn estimate_proof_reservation(
    view: &StateView,
    slots: usize,
) -> Result<usize, StateProofError> {
    let state = view_state(view);
    let max_storage = state
        .accounts
        .values()
        .map(|account| account.storage.len())
        .max()
        .unwrap_or(0);
    let accounts = state.accounts.len().checked_mul(256);
    let storage = max_storage.checked_mul(128);
    let paths = slots
        .checked_add(1)
        .and_then(|paths| paths.checked_mul(65 * 1_024));
    [accounts, storage, paths]
        .into_iter()
        .try_fold(1_048_576_usize, |sum, value| {
            value
                .and_then(|value| sum.checked_add(value))
                .ok_or(StateProofError::ArithmeticOverflow)
        })
}
