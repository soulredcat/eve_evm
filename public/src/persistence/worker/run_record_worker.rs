// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    append_record_request::append_record_request,
    types::{RecordRequest, WorkerState},
};
use crate::persistence::handoff::payload_reservation_id;
use eve_storage::records::OpaqueRecordRepository;
use std::sync::{Arc, mpsc::Receiver};

pub(super) fn run_record_worker(
    mut repository: OpaqueRecordRepository,
    receiver: Receiver<RecordRequest>,
    state: Arc<WorkerState>,
) -> OpaqueRecordRepository {
    for request in receiver {
        let result = append_record_request(&mut repository, &state, &request);
        state
            .submitted
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(&payload_reservation_id(&request.payload));
        // One slot and one result: absent/cancelled consumers cannot block the writer.
        let _ = request.reply.try_send(result);
        // The charged payload remains owned through repository I/O and reply publication.
    }
    repository
}
