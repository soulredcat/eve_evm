// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::StateError;

/// Allocation-free canonical field framing and network-string sizing admission.
/// This does not authenticate a version or replace owned semantic validation.
pub fn preflight_state_version(bytes: &[u8]) -> Result<usize, StateError> {
    if bytes.len() > 4_096 {
        return Err(StateError::BudgetExceeded);
    }
    super::journal_decoding::scan_version_allocations(bytes)
}
