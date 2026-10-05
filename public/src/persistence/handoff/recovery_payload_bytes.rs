// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::RecoveryPayload;

/// Borrow bytes while their payload owner retains its resource charge.
pub fn recovery_payload_bytes(payload: &RecoveryPayload) -> &[u8] {
    &payload.0.bytes
}
