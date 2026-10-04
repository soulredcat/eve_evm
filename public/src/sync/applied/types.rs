// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::resources::{EstimatedWorkingLease, EstimatedWorkingPool};
use crate::persistence::segmented::{
    SealedSegmentedBatch, SegmentedError, SegmentedPartPool, SegmentedTicket, SegmentedWorker,
};
use crate::persistence::{
    handoff::{HandoffError, HandoffPool, RecoveryPayload},
    worker::{RecordTicket, RecordWorker, RecordWorkerError},
};
use eve_finality_verifier::{ImportError, ImportWireError, RecoveryError};
use eve_node_policy::{AppliedHeight, PublicBudget, PublicWatermarks};
use eve_state::{StateBudget, StateVersion};
use eve_storage::records::segmented::recovery::SegmentedRecoveryError;
use eve_storage::records::segmented::{SegmentedCodecLimits, SegmentedLogicalIdentity};
use eve_storage::records::{
    OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordIdentity, OpaqueRecordRepository,
};
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, RwLock},
};

/// Local operator limits. They do not change consensus validity or the canonical executor.
pub struct AppliedConfig {
    pub path: PathBuf,
    pub identity: OpaqueRecordIdentity,
    pub public_budget: PublicBudget,
    pub state_budget: StateBudget,
    pub repository_budget: OpaqueRecordBudget,
    pub worker_scratch_limit: u64,
    pub maximum_recovery_payload_bytes: usize,
}

/// Import authenticates certified outcomes; replay independently executes its supported inputs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppliedMode {
    EmptyReplay,
    AuthenticatedImport,
}

#[derive(Debug)]
pub enum AppliedError {
    InvalidConfiguration,
    ArithmeticOverflow,
    EstimatedCapacity,
    AccountingUnavailable,
    AllocationFailed,
    PayloadLimit,
    QueueLimit,
    StorageUnavailable,
    StorageFailed,
    PublicationUnavailable,
    InvalidDurablePrefix,
    UnexpectedAcknowledgement,
    Closed,
    CheckpointPending,
    WrongMode,
    Recovery(RecoveryError),
    Import(ImportError),
    ImportWire(ImportWireError),
    Handoff(HandoffError),
    Worker(RecordWorkerError),
    Segmented(SegmentedError),
    SegmentedRecovery(SegmentedRecoveryError),
}

pub(super) struct ChargedAppliedState {
    pub(super) state: super::state::AppliedState,
    pub(super) _lease: EstimatedWorkingLease,
}

/// One coherent immutable state and markers. Captures retain its estimated state charge.
pub struct AppliedPublication {
    pub(super) generation: Arc<ChargedAppliedState>,
    pub(super) markers: PublicWatermarks,
    pub(super) durable_cursor: OpaqueRecordCursor,
    pub(super) admitted_cursor: OpaqueRecordCursor,
    pub(super) storage_failed: bool,
    pub(super) segmented_position: Option<super::segmented::SegmentedAppliedPosition>,
}

#[derive(Clone)]
pub struct AppliedReader {
    pub(super) storage: Arc<super::resources::storage_admission::StorageAdmissionPool>,
    pub(super) publication: Arc<RwLock<Arc<AppliedPublication>>>,
    pub(super) working: Arc<EstimatedWorkingPool>,
}

pub(super) struct PendingRecord {
    pub(super) parent: OpaqueRecordCursor,
    pub(super) target_cursor: OpaqueRecordCursor,
    pub(super) target: StateVersion,
    pub(super) payload: PendingPayload,
}

pub(super) enum PendingPayload {
    Compact {
        ticket: RecordTicket,
        payload: RecoveryPayload,
    },
    Segmented {
        ticket: SegmentedTicket,
        payload: SealedSegmentedBatch,
        identity: SegmentedLogicalIdentity,
        target_binding: [u8; 32],
    },
}

pub(super) enum AppliedBackend {
    Compact {
        worker: Option<RecordWorker>,
        pool: Arc<HandoffPool>,
    },
    Segmented {
        worker: Option<SegmentedWorker>,
        pool: Arc<SegmentedPartPool>,
        codec: SegmentedCodecLimits,
    },
}

/// Exclusive admission/acknowledgement owner. The sole writer never reads mutable RAM state.
pub struct AppliedOwner {
    pub(super) config: AppliedConfig,
    pub(super) effective_storage_identity: OpaqueRecordIdentity,
    pub(super) backend: AppliedBackend,
    pub(super) reader: AppliedReader,
    pub(super) pending: VecDeque<PendingRecord>,
    pub(super) checkpoint: Option<Box<super::checkpoints::PendingCheckpointActivation>>,
    pub(super) admitted_cursor: OpaqueRecordCursor,
    pub(super) durable_cursor: OpaqueRecordCursor,
    pub(super) database_sequence: u64,
    pub(super) storage_failed: bool,
    pub(super) metadata_lease: EstimatedWorkingLease,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppliedAdmission {
    pub applied: AppliedHeight,
    pub admitted_cursor: OpaqueRecordCursor,
}

/// Shutdown preserves exact charged bytes whose acknowledgements were not validated.
pub struct RetainedAppliedTail {
    pub(super) pending: VecDeque<PendingRecord>,
    pub(super) _metadata_lease: EstimatedWorkingLease,
    pub(super) segmented_tails: Option<[Option<crate::persistence::segmented::SegmentedTail>; 2]>,
    pub(super) checkpoint: Option<Box<super::checkpoints::PendingCheckpointActivation>>,
    pub(super) checkpoint_tail: Option<crate::persistence::segmented::checkpoints::CheckpointTail>,
}

pub struct AppliedShutdown {
    pub repository: Result<OpaqueRecordRepository, RecordWorkerError>,
    pub acknowledgement_error: Option<AppliedError>,
    pub checkpoint_error: Option<super::checkpoints::CheckpointAppliedError>,
    pub unacknowledged_tail: RetainedAppliedTail,
    pub publication: Option<Arc<AppliedPublication>>,
}
