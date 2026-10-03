// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RecordTicket, RecordWorkerError};
use eve_storage::records::OpaqueRecordAck;
use std::sync::mpsc::TryRecvError;

/// Poll without waiting; keep a pending ticket and remove it after any terminal result.
/// A successful result is the worker's actual synced opaque acknowledgement, never finality.
pub fn try_receive_record_ack(
    ticket: &RecordTicket,
) -> Result<Option<OpaqueRecordAck>, RecordWorkerError> {
    match ticket.receiver.try_recv() {
        Ok(result) => result.map(Some),
        Err(TryRecvError::Empty) => Ok(None),
        Err(TryRecvError::Disconnected) => Err(RecordWorkerError::AcknowledgementLost),
    }
}
