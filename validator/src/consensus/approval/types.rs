// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::signing::SignerConfig;
use eve_consensus_comet::wire::tendermint::abci::RequestProcessProposal;
use eve_evm::{CompleteExecutionError, PreparedStateBlock};
use eve_state::StateVersion;

/// Fields and production construction stay private to the execution factory domain.
/// The pinned local engine is trusted for the request hash/data binding; no full header is present.
pub(in crate::consensus) struct ExecutionApproval {
    pub(super) config: SignerConfig,
    pub(super) parent: StateVersion,
    pub(super) database_sequence: u64,
    pub(super) consensus_hash: [u8; 32],
    pub(super) request: RequestProcessProposal,
    pub(super) prepared: PreparedStateBlock,
}

/// Deterministic invalid execution is distinct from unavailable local state/resources/context.
#[derive(Debug)]
pub(in crate::consensus) enum ApprovalError {
    InvalidExecution(CompleteExecutionError),
    Unavailable {
        reason: &'static str,
        cause: Option<CompleteExecutionError>,
    },
}
