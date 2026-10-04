// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::SegmentedError;
use super::{CheckpointAck, CheckpointTicket};
use std::sync::{atomic::Ordering, mpsc::TryRecvError};
pub fn try_receive_checkpoint_ack(
    ticket: &CheckpointTicket,
) -> Result<Option<CheckpointAck>, SegmentedError> {
    let mut admission = ticket
        .state
        .admission
        .lock()
        .map_err(|_| SegmentedError::AccountingUnavailable)?;
    let ack = match ticket.receiver.try_recv() {
        Ok(result) => result?,
        Err(TryRecvError::Empty) => return Ok(None),
        Err(TryRecvError::Disconnected) => {
            ticket.state.failed.store(true, Ordering::Release);
            return Err(SegmentedError::AckLost);
        }
    };
    let Some(slot) = admission.checkpoint.as_ref() else {
        ticket.state.failed.store(true, Ordering::Release);
        return Err(SegmentedError::AckMismatch);
    };
    if slot.record.0.id != ticket.id
        || slot.complete != Some(ack)
        || slot.acknowledged != ack.cursor
        || ack.cursor != slot.record.0.cursor
        || ack.metadata != slot.record.0.metadata
        || ack.database_sequence == 0
    {
        ticket.state.failed.store(true, Ordering::Release);
        return Err(SegmentedError::AckMismatch);
    }
    let retained = admission.checkpoint.take();
    drop(admission);
    drop(retained);
    Ok(Some(ack))
}
