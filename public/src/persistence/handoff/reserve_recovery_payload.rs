// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{HandoffError, HandoffPool, PayloadReservation, types::ReservationLease};
use std::{sync::Arc, time::Instant};

/// Reserve count and bytes atomically before allocating a fixed-capacity encoded buffer.
/// Does not block for capacity; call before publishing RAM state, outside its locks.
pub fn reserve_recovery_payload(
    pool: &Arc<HandoffPool>,
    encoded_length: usize,
) -> Result<PayloadReservation, HandoffError> {
    let charge = u64::try_from(encoded_length).map_err(|_| HandoffError::InvalidLength)?;
    if charge == 0 {
        return Err(HandoffError::InvalidLength);
    }
    if charge > pool.budget.maximum_record_bytes || charge > pool.budget.maximum_batch_bytes {
        return Err(HandoffError::PayloadLimit);
    }
    let lease = {
        let mut accounting = pool
            .accounting
            .lock()
            .map_err(|_| HandoffError::AccountingUnavailable)?;
        let bytes = accounting
            .bytes
            .checked_add(charge)
            .ok_or(HandoffError::QueueLimit)?;
        if bytes > pool.budget.queue_bytes
            || accounting.leases.len() as u64 >= pool.budget.queue_batches
        {
            return Err(HandoffError::QueueLimit);
        }
        let id = accounting
            .next_id
            .checked_add(1)
            .ok_or(HandoffError::AccountingUnavailable)?;
        accounting.leases.insert(id, Instant::now());
        accounting.next_id = id;
        accounting.bytes = bytes;
        ReservationLease {
            pool: Arc::clone(pool),
            id,
            bytes: charge,
        }
    };
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(encoded_length)
        .map_err(|_| HandoffError::AllocationFailed)?;
    // Reject hidden logical spare capacity rather than charging a caller-supplied estimate.
    if bytes.capacity() != encoded_length {
        return Err(HandoffError::UnexpectedCapacity);
    }
    Ok(PayloadReservation { bytes, lease })
}
