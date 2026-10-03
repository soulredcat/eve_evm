// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    BATCHES, MetadataLease, SEGMENTS, SealedSegmentedBatch, SegmentedError, SegmentedPartPool,
    WorkerLifetimeLease,
};
use eve_storage::records::segmented::{SegmentedLogicalIdentity, SegmentedRecoveryAnchor};
use eve_storage::records::{OpaqueRecordCursor, OpaqueRecordRepository};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64},
        mpsc::{Receiver, SyncSender},
    },
    thread::JoinHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedLogicalAck {
    pub identity: SegmentedLogicalIdentity,
    pub target_state_binding: [u8; 32],
    pub marker_cursor: OpaqueRecordCursor,
    pub references: [OpaqueRecordCursor; SEGMENTS],
    pub segment_count: usize,
    pub database_sequence: u64,
}

pub(in crate::persistence::segmented) struct AdmittedBatch {
    pub(in crate::persistence::segmented) batch: SealedSegmentedBatch,
    pub(in crate::persistence::segmented) acknowledged: OpaqueRecordCursor,
    pub(in crate::persistence::segmented) complete: Option<SegmentedLogicalAck>,
}
pub(in crate::persistence::segmented) struct Admission {
    pub(in crate::persistence::segmented) cursor: OpaqueRecordCursor,
    pub(in crate::persistence::segmented) logical: SegmentedRecoveryAnchor,
    pub(in crate::persistence::segmented) slots: [Option<AdmittedBatch>; BATCHES],
}
pub(in crate::persistence::segmented) struct WorkerState {
    pub(in crate::persistence::segmented) pool: Arc<SegmentedPartPool>,
    pub(in crate::persistence::segmented) admission: Mutex<Admission>,
    pub(in crate::persistence::segmented) failed: AtomicBool,
    pub(in crate::persistence::segmented) scratch: AtomicU64,
    pub(in crate::persistence::segmented) _metadata: MetadataLease,
    #[cfg(test)]
    pub(in crate::persistence::segmented) pause: Mutex<Option<super::super::tests::Pause>>,
    /// Last field: release the shared-pool worker fence after admission and metadata cleanup.
    pub(in crate::persistence::segmented) _worker_lifetime: WorkerLifetimeLease,
}
pub(in crate::persistence::segmented) struct Request {
    pub(in crate::persistence::segmented) batch: SealedSegmentedBatch,
    pub(in crate::persistence::segmented) slot: usize,
    pub(in crate::persistence::segmented) reply:
        SyncSender<Result<SegmentedLogicalAck, SegmentedError>>,
}
pub struct SegmentedTicket {
    pub(in crate::persistence::segmented) receiver:
        Receiver<Result<SegmentedLogicalAck, SegmentedError>>,
    pub(in crate::persistence::segmented) state: Arc<WorkerState>,
    pub(in crate::persistence::segmented) slot: usize,
    pub(in crate::persistence::segmented) id: u64,
}
pub struct SegmentedWorker {
    pub(in crate::persistence::segmented) sender: SyncSender<Request>,
    pub(in crate::persistence::segmented) state: Arc<WorkerState>,
    pub(in crate::persistence::segmented) thread: JoinHandle<OpaqueRecordRepository>,
}
pub struct RejectedSegmentedBatch {
    pub error: SegmentedError,
    pub batch: SealedSegmentedBatch,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedWorkerObservation {
    pub active_estimated_scratch_bytes: u64,
    pub scratch_limit: u64,
    pub retained_logical_batches: usize,
    pub storage_failed: bool,
}
pub struct SegmentedTail {
    pub batch: SealedSegmentedBatch,
    pub last_acknowledged_physical_cursor: OpaqueRecordCursor,
    pub complete_marker: Option<SegmentedLogicalAck>,
}
pub struct SegmentedWorkerShutdown {
    pub repository: Result<OpaqueRecordRepository, SegmentedError>,
    pub tails: [Option<SegmentedTail>; BATCHES],
}
pub(in crate::persistence::segmented) struct ScratchLease {
    pub(in crate::persistence::segmented) state: Arc<WorkerState>,
}
