// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::{
    consensus::{
        certificates::CertificateError,
        commitments::HeightMappingError,
        history::{HistoryError, NativeHistoryVerifier, VerifiedNativeHeader},
    },
    wire::tendermint::types::BlockId,
};
use eve_protocol_config::records::{ApplicationCommitment, RecordError};
use eve_state::{EvmStateRoot, ExecutionBlockHash, StateError, StateIdentity, SystemStateRoot};

/// Local canonical EVE genesis plus sequential native history; no freshness claim.
#[derive(Clone, Debug)]
pub struct DevelopmentFinalityVerifier {
    pub(in crate::finality) identity: StateIdentity,
    pub(in crate::finality) native: NativeHistoryVerifier,
    pub(in crate::finality) latest: Option<VerifiedNativeHeader>,
    pub(in crate::finality) unsupported_activation: Option<u64>,
}

/// Native header tied to this verifier's canonical EVE genesis.
#[derive(Clone, Debug)]
pub struct VerifiedDevelopmentHeader {
    pub(in crate::finality) identity: StateIdentity,
    pub(in crate::finality) native: VerifiedNativeHeader,
}

/// Committed outcome at H authenticated through certified H+1, not EVM execution.
#[derive(Clone, Debug)]
pub struct AuthenticatedApplicationAnchor {
    pub(in crate::finality) identity: StateIdentity,
    pub(in crate::finality) execution_height: u64,
    pub(in crate::finality) evm_root: EvmStateRoot,
    pub(in crate::finality) system_root: SystemStateRoot,
    pub(in crate::finality) execution_hash: ExecutionBlockHash,
    pub(in crate::finality) application: ApplicationCommitment,
    pub(in crate::finality) consensus_height: i64,
    pub(in crate::finality) consensus_block_id: BlockId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FinalityError {
    State(StateError),
    Native(HistoryError),
    Certificate(CertificateError),
    Commitment(RecordError),
    HeightMapping(HeightMappingError),
    UnsupportedActivation,
    WrongApplicationIdentity,
    WrongApplicationCommitment,
    WrongApplicationHeight,
    InvalidApplicationHashWidth,
    WrongNativeApplicationVersion,
    HeightOverflow,
}
