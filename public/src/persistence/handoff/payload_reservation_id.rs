// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::RecoveryPayload;

pub(in crate::persistence) fn payload_reservation_id(payload: &RecoveryPayload) -> u64 {
    payload.0._lease.id
}
