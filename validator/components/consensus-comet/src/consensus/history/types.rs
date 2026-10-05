// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    consensus::{
        authentication::ConsensusAuthenticationRequirement, certificates::CertificateError,
    },
    wire::tendermint::types::{BlockId, Header},
};

/// Locally anchored, sequential native verification; no EVE execution/freshness claim.
#[derive(Clone, Debug)]
pub struct NativeHistoryVerifier {
    pub(super) chain_id: String,
    pub(super) height: i64,
    pub(super) block_id: Option<BlockId>,
    pub(super) header: Option<Header>,
    pub(super) next_validator_hash: [u8; 32],
    pub(super) genesis_app_hash: Option<[u8; 32]>,
    pub(super) authentication: ConsensusAuthenticationRequirement,
}

/// Certified native header with checked transaction data, not verified EVM post-state.
#[derive(Clone, Debug)]
pub struct VerifiedNativeHeader {
    pub(super) header: Header,
    pub(super) block_id: BlockId,
    pub(super) authentication: ConsensusAuthenticationRequirement,
    pub(super) signed_voting_power: i64,
    pub(super) total_voting_power: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoryError {
    Certificate(CertificateError),
    WrongSuccessorHeight,
    WrongParentBlock,
    WrongValidatorTransition,
    WrongTransactionData,
    WrongGenesisApplicationHash,
    NonIncreasingTime,
    HeightOverflow,
}
