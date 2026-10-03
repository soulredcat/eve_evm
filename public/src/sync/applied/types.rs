// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::resources::{EstimatedWorkingLease, EstimatedWorkingPool};
use crate::persistence::{
    handoff::{HandoffError, HandoffPool, RecoveryPayload},
    worker::{RecordTicket, RecordWorker, RecordWorkerError},
};
use eve_finality_verifier::{ImportError, ImportWireError, RecoveryError};
use eve_node_policy::{AppliedHeight, PublicBudget, PublicWatermarks};
use eve_state::{StateBudget, StateVersion};
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
    WrongMode,
    Recovery(RecoveryError),
    Import(ImportError),
    ImportWire(ImportWireError),
    Handoff(HandoffError),
    Worker(RecordWorkerError),
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
}

#[derive(Clone)]
pub struct AppliedReader {
    pub(super) publication: Arc<RwLock<Arc<AppliedPublication>>>,
    pub(super) working: Arc<EstimatedWorkingPool>,
}

pub(super) struct PendingRecord {
    pub(super) ticket: RecordTicket,
    pub(super) parent: OpaqueRecordCursor,
    pub(super) target_cursor: OpaqueRecordCursor,
    pub(super) target: StateVersion,
    pub(super) payload: RecoveryPayload,
}

/// Exclusive admission/acknowledgement owner. The sole writer never reads mutable RAM state.
pub struct AppliedOwner {
    pub(super) config: AppliedConfig,
    pub(super) effective_storage_identity: OpaqueRecordIdentity,
    pub(super) worker: Option<RecordWorker>,
    pub(super) pool: Arc<HandoffPool>,
    pub(super) reader: AppliedReader,
    pub(super) pending: VecDeque<PendingRecord>,
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
}

pub struct AppliedShutdown {
    pub repository: Result<OpaqueRecordRepository, RecordWorkerError>,
    pub acknowledgement_error: Option<AppliedError>,
    pub unacknowledged_tail: RetainedAppliedTail,
    pub publication: Option<Arc<AppliedPublication>>,
}
