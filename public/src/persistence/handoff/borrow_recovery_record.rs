// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::RecoveryPayload;

/// Borrow the original charged buffer for the repository's existing Vec record API.
pub(in crate::persistence) fn borrow_recovery_record(payload: &RecoveryPayload) -> &[Vec<u8>] {
    std::slice::from_ref(&payload.0.bytes)
}
