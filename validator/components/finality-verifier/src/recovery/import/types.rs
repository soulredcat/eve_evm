// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_state::{BlockPayload, StateCommit, StateError, StateJournal};

use crate::recovery::{
    NativeDataFrame, NativeFrame, RecoveryError, types::capability::FixedGenesisPolicy,
};
use crate::{
    AuthenticatedApplicationAnchor, DevelopmentFinalityVerifier, VerifiedDevelopmentHeader,
};

/// Untrusted typed delta and complete ordered block/proof data. This is not a wire codec.
/// The journal binds the receiver's exact LOCAL parent version, including its auxiliary
/// content digest. A different auxiliary representation is refused, never rebound.
#[derive(Clone, Debug, PartialEq)]
pub struct AuthenticatedImportInput {
    pub journal: StateJournal,
    pub execution: BlockPayload,
    pub finalized: NativeFrame,
    pub lookahead: NativeDataFrame,
}

/// Locally selected genesis or H/H+1-authenticated imported committed state.
/// This capability is NOT independent EVM replay or a durability/freshness assertion.
/// EVE_APP_V1 authenticates roots and execution hash. Unused code and content_digest
/// remain checked local auxiliary representation, not separately certified values.
#[derive(Clone, Debug)]
pub struct ImportedState {
    pub(in crate::recovery::import) commit: Arc<StateCommit>,
    pub(in crate::recovery::import) finality: DevelopmentFinalityVerifier,
    pub(in crate::recovery::import) policy: Arc<FixedGenesisPolicy>,
    pub(in crate::recovery::import) lookahead: Option<Arc<NativeDataFrame>>,
    pub(in crate::recovery::import) lookahead_header: Option<VerifiedDevelopmentHeader>,
    pub(in crate::recovery::import) anchor: Option<AuthenticatedApplicationAnchor>,
}

/// Private preparation retains the exact input; public owns admission/publication.
#[derive(Debug)]
pub struct ImportedTransition {
    pub(in crate::recovery::import) state: Arc<ImportedState>,
    pub(in crate::recovery::import) input: Arc<AuthenticatedImportInput>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImportError {
    Recovery(RecoveryError),
    State(StateError),
    Header(eve_protocol_config::headers::HeaderError),
    WrongParent,
    WrongHeight,
    InvalidExecutionContext,
    InvalidExecutionHistory,
    CandidateReservation { required: usize, reserved: usize },
}
