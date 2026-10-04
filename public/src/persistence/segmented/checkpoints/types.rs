// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedError,
    types::{MetadataLease, WorkerState},
};
use eve_storage::records::{OpaqueRecordCursor, segmented::checkpoints::CheckpointBaseMetadata};
use std::{
    sync::{
        Arc,
        mpsc::{Receiver, SyncSender},
    },
    time::Instant,
};

/// Immutable local encoding and real same-pool metadata charge; no artifact authority.
#[derive(Clone)]
pub struct SealedCheckpointRecord(pub(in crate::persistence::segmented) Arc<CheckpointRecord>);
pub(in crate::persistence::segmented) struct CheckpointRecord {
    pub(in crate::persistence::segmented) payload: Vec<u8>,
    pub(in crate::persistence::segmented) metadata: CheckpointBaseMetadata,
    pub(in crate::persistence::segmented) cursor: OpaqueRecordCursor,
    pub(in crate::persistence::segmented) id: u64,
    pub(in crate::persistence::segmented) created: Instant,
    pub(in crate::persistence::segmented) _metadata: MetadataLease,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointAck {
    pub metadata: CheckpointBaseMetadata,
    pub cursor: OpaqueRecordCursor,
    pub database_sequence: u64,
}
pub(in crate::persistence::segmented) struct AdmittedCheckpoint {
    pub(in crate::persistence::segmented) record: SealedCheckpointRecord,
    pub(in crate::persistence::segmented) acknowledged: OpaqueRecordCursor,
    pub(in crate::persistence::segmented) complete: Option<CheckpointAck>,
}
pub(in crate::persistence::segmented) struct CheckpointRequest {
    pub(in crate::persistence::segmented) record: SealedCheckpointRecord,
    pub(in crate::persistence::segmented) reply: SyncSender<Result<CheckpointAck, SegmentedError>>,
}
pub struct CheckpointTicket {
    pub(in crate::persistence::segmented) receiver: Receiver<Result<CheckpointAck, SegmentedError>>,
    pub(in crate::persistence::segmented) state: Arc<WorkerState>,
    pub(in crate::persistence::segmented) id: u64,
}
pub struct RejectedCheckpointRecord {
    pub error: SegmentedError,
    pub record: SealedCheckpointRecord,
}
pub struct CheckpointTail {
    pub record: SealedCheckpointRecord,
    pub last_acknowledged_physical_cursor: OpaqueRecordCursor,
    pub complete: Option<CheckpointAck>,
}
