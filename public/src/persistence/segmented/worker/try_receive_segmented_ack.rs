// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedError, SegmentedLogicalAck, SegmentedTicket};
use std::sync::{atomic::Ordering, mpsc::TryRecvError};
pub fn try_receive_segmented_ack(
    ticket: &SegmentedTicket,
) -> Result<Option<SegmentedLogicalAck>, SegmentedError> {
    // Lock before consumption: poison rejection cannot lose a still-pending acknowledgement.
    let mut admission = ticket
        .state
        .admission
        .lock()
        .map_err(|_| SegmentedError::AccountingUnavailable)?;
    let ack = match ticket.receiver.try_recv() {
        Ok(result) => result?,
        Err(TryRecvError::Empty) => return Ok(None),
        Err(TryRecvError::Disconnected) => return Err(SegmentedError::AckLost),
    };
    let slot = admission.slots[ticket.slot]
        .as_ref()
        .ok_or(SegmentedError::AckMismatch)?;
    let batch = &slot.batch.0;
    if batch.id != ticket.id
        || slot.complete != Some(ack)
        || ack.marker_cursor != batch.marker_cursor
        || ack.references != batch.references
        || ack.segment_count != batch.plan.layout.segment_count
        || ack.identity != batch.plan.marker.identity
        || ack.target_state_binding != batch.plan.marker.target_state_binding
    {
        ticket.state.failed.store(true, Ordering::Release);
        return Err(SegmentedError::AckMismatch);
    }
    let retained = admission.slots[ticket.slot].take();
    drop(admission);
    drop(retained);
    Ok(Some(ack))
}
