// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{HandoffError, PayloadReservation};

/// Append only within the preallocated buffer; a rejected write leaves bytes unchanged.
pub fn write_reserved_payload(
    reservation: &mut PayloadReservation,
    bytes: &[u8],
) -> Result<(), HandoffError> {
    let length = reservation
        .bytes
        .len()
        .checked_add(bytes.len())
        .ok_or(HandoffError::PayloadLimit)?;
    if length > reservation.bytes.capacity() {
        return Err(HandoffError::PayloadLimit);
    }
    reservation.bytes.extend_from_slice(bytes);
    Ok(())
}
