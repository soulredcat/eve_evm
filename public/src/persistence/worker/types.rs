// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::persistence::handoff::{HandoffPool, RecoveryPayload};
use eve_storage::records::{
    OpaqueRecordAck, OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordRepository,
};
use std::{
    collections::BTreeSet,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64},
        mpsc::{Receiver, SyncSender},
    },
    thread::JoinHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordWorkerError {
    InvalidConfiguration,
    ForeignPool,
    QueueFull,
    Closed,
    StorageFailed,
    ScratchLimit,
    RecordLimit,
    SpawnFailed,
    WorkerPanicked,
    AcknowledgementLost,
    AlreadySubmitted,
    AdmissionUnavailable,
}

/// Rejected admission retains the original charged payload for the caller.
pub struct RejectedRecord {
    pub error: RecordWorkerError,
    pub payload: RecoveryPayload,
}

/// Receives at most one actual synced opaque acknowledgement, never a finality height.
pub struct RecordTicket {
    pub(super) receiver: Receiver<Result<OpaqueRecordAck, RecordWorkerError>>,
}

pub(super) struct RecordRequest {
    pub(super) expected: OpaqueRecordCursor,
    pub(super) payload: RecoveryPayload,
    pub(super) reply: SyncSender<Result<OpaqueRecordAck, RecordWorkerError>>,
}

pub(super) struct WorkerState {
    pub(super) pool: Arc<HandoffPool>,
    pub(super) repository_budget: OpaqueRecordBudget,
    pub(super) scratch_limit: u64,
    pub(super) active_scratch: AtomicU64,
    pub(super) failed: AtomicBool,
    pub(super) submitted: Mutex<BTreeSet<u64>>,
    #[cfg(test)]
    pub(super) pause: std::sync::Mutex<Option<super::tests::AppendPause>>,
}

/// Sole submission sender and exclusive thread owner; no Clone sender API is exposed.
pub struct RecordWorker {
    pub(super) sender: SyncSender<RecordRequest>,
    pub(super) thread: JoinHandle<OpaqueRecordRepository>,
    pub(super) state: Arc<WorkerState>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecordWorkerObservation {
    pub active_scratch_bytes: u64,
    pub scratch_limit: u64,
    pub storage_failed: bool,
}
