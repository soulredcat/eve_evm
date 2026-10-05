// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RecordTicket, RecordWorkerError};
use eve_storage::records::OpaqueRecordAck;

/// Blocking receive; do not call while holding applied-state or execution locks.
pub fn receive_record_ack(ticket: RecordTicket) -> Result<OpaqueRecordAck, RecordWorkerError> {
    ticket
        .receiver
        .recv()
        .map_err(|_| RecordWorkerError::AcknowledgementLost)?
}
