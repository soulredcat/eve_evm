// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::ImportError;
use crate::{
    AuthenticatedApplicationAnchor, DevelopmentFinalityVerifier, VerifiedDevelopmentHeader,
    recovery::{NativeDataFrame, RecoveryError, types::capability::FixedGenesisPolicy},
};
use eve_state::{BlockPayload, Header, StateBudget, StateCommit, StateError, StateVersion};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct CheckpointLimits {
    pub maximum_height_gap: u64,
    pub maximum_witness_bytes: usize,
}

/// Untrusted proof/environment material; versions' auxiliary digests create no authority.
#[derive(Clone, Debug)]
pub enum CheckpointWitness {
    Execution(Box<CheckpointExecutionWitness>),
    Lookahead(Box<NativeDataFrame>),
}

#[derive(Clone, Debug)]
pub struct CheckpointExecutionWitness {
    pub native: NativeDataFrame,
    pub version: StateVersion,
    pub block: BlockPayload,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckpointError {
    InvalidLimit,
    WrongIdentity,
    WrongHeight,
    WrongParent,
    WrongHistory,
    WrongNativeData,
    WrongTarget,
    Incomplete,
    Overflow,
    ReservationTooSmall,
    State(StateError),
    Recovery(RecoveryError),
    Import(ImportError),
}

/// Private streaming state retains only one preceding execution/header and the fixed local policy.
pub struct CheckpointSession {
    pub(super) target: Arc<StateCommit>,
    pub(super) budget: StateBudget,
    pub(super) limits: CheckpointLimits,
    pub(super) required: usize,
    pub(super) base_height: u64,
    pub(super) next_height: u64,
    pub(super) previous_version: StateVersion,
    pub(super) previous_header: Header,
    pub(super) finality: DevelopmentFinalityVerifier,
    pub(super) policy: Arc<FixedGenesisPolicy>,
    pub(super) seed_data: Option<Arc<NativeDataFrame>>,
    pub(super) seed_header: Option<VerifiedDevelopmentHeader>,
    pub(super) closing_data: Option<Arc<NativeDataFrame>>,
    pub(super) closing_header: Option<VerifiedDevelopmentHeader>,
}

/// Certified roots/execution and checked local representation; no replay/storage/freshness proof.
pub struct AuthenticatedCheckpoint {
    pub(super) commit: Arc<StateCommit>,
    pub(super) finality: DevelopmentFinalityVerifier,
    pub(super) policy: Arc<FixedGenesisPolicy>,
    pub(super) lookahead: Arc<NativeDataFrame>,
    pub(super) lookahead_header: VerifiedDevelopmentHeader,
    pub(super) anchor: AuthenticatedApplicationAnchor,
}
