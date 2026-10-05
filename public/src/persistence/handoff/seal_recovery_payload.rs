// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{HandoffError, PayloadReservation, RecoveryPayload, types::RetainedPayload};
use std::sync::Arc;

/// Freeze fully encoded bytes without copying their allocation or releasing the lease.
pub fn seal_recovery_payload(
    reservation: PayloadReservation,
) -> Result<RecoveryPayload, HandoffError> {
    if reservation.bytes.len() != reservation.bytes.capacity() {
        return Err(HandoffError::IncompletePayload);
    }
    Ok(RecoveryPayload(Arc::new(RetainedPayload {
        bytes: reservation.bytes,
        _lease: reservation.lease,
    })))
}
