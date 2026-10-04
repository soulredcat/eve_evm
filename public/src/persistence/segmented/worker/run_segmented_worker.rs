// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        checkpoints::dispatch_checkpoint_request,
        types::{WorkerRequest, WorkerState},
    },
    dispatch_segmented_batch_request::dispatch_segmented_batch_request,
};
use eve_storage::records::OpaqueRecordRepository;
use std::sync::{Arc, mpsc::Receiver};
pub(super) fn run_segmented_worker(
    mut repository: OpaqueRecordRepository,
    state: Arc<WorkerState>,
    receiver: Receiver<WorkerRequest>,
) -> OpaqueRecordRepository {
    for request in receiver {
        match request {
            WorkerRequest::Segmented(request) => {
                dispatch_segmented_batch_request(&mut repository, &state, request)
            }
            WorkerRequest::Checkpoint(request) => {
                dispatch_checkpoint_request(&mut repository, &state, request)
            }
        }
    }
    repository
}
