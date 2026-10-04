// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::replay::types::ReplayDecision;
use crate::consensus::{
    approval::{ApprovalRegistry, ExecutionApproval},
    signing::DurableSigner,
};
use crate::development::acceptance::AcceptanceFixture;
use alloy_primitives::Address;
use eve_consensus_comet::wire::tendermint::{
    abci::{RequestFinalizeBlock, ValidatorUpdate},
    types::ConsensusParams,
};
use eve_state::{StateBudget, StateCommit};
use eve_storage::{
    records::{OpaqueRecordCursor, OpaqueRecordRepository},
    state::{StateRepository, StateService},
};
use prost_types::Timestamp;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// Root bootstrap supplies validated immutable development genesis/configuration.
pub(in crate::consensus) struct ApplicationConfig {
    pub acceptance_fixture: Option<Arc<AcceptanceFixture>>,
    pub genesis: Arc<StateCommit>,
    pub initial_validators: Vec<ValidatorUpdate>,
    pub consensus_params: ConsensusParams,
    pub initial_time: Timestamp,
    pub expected_app_state: Value,
    pub proposer_owners: BTreeMap<[u8; 20], Address>,
    pub logical_budget: StateBudget,
    pub delta_serving_budget: super::DeltaServingBudget,
    pub reserved_clone_bytes: usize,
    pub maximum_cached_candidates: usize,
    pub maximum_cached_bytes: usize,
}

pub(super) struct PendingBlock {
    pub decision: ReplayDecision,
    pub decision_cursor: OpaqueRecordCursor,
    pub approval: Arc<ExecutionApproval>,
    pub request: RequestFinalizeBlock,
}

/// One actor owns both write capabilities; query/signing consumers use StateService.
pub(in crate::consensus) struct ConsensusApplication {
    pub(super) acceptance_poison_used: std::sync::atomic::AtomicBool,
    pub(super) config: ApplicationConfig,
    pub(super) repository: StateRepository,
    pub(super) service: Arc<StateService>,
    pub(super) replay_repository: OpaqueRecordRepository,
    pub(super) replay_cursor: OpaqueRecordCursor,
    pub(super) signer: Arc<Mutex<DurableSigner>>,
    pub(super) completed: Option<ReplayDecision>,
    pub(super) completed_cursor: Option<OpaqueRecordCursor>,
    pub(super) retained_decision: Option<(ReplayDecision, OpaqueRecordCursor)>,
    pub(super) pending: Option<PendingBlock>,
    pub(super) replayed_height: Option<u64>,
    pub(super) approvals: Arc<ApprovalRegistry>,
    pub(super) fenced: bool,
    #[cfg(test)]
    pub(super) simulated_failure: Option<SimulatedApplicationFailure>,
}

#[cfg(test)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum SimulatedApplicationFailure {
    BeforeState,
    StateSynced,
    MetadataSynced,
    ExitAfterState,
}
