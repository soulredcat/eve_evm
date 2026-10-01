// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    consensus::{authentication::ConsensusAuthenticationRequirement, signing::SigningError},
    wire::tendermint::types::BlockId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClassicalValidator {
    pub public_key: [u8; 32],
    pub voting_power: i64,
}

/// Provenance is a caller obligation; this data type authenticates no peer/history.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalValidatorSet {
    pub height: i64,
    pub authentication: ConsensusAuthenticationRequirement,
    pub validators: Vec<ClassicalValidator>,
}

/// Verified native certificate only, not independent execution or PQ security.
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiedClassicalCommit {
    pub chain_id: String,
    pub height: i64,
    pub round: i32,
    pub block_id: BlockId,
    pub validator_set_hash: [u8; 32],
    pub signed_voting_power: i64,
    pub total_voting_power: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertificateError {
    Signing(SigningError),
    UnsupportedAuthentication,
    InvalidHeaderVersion,
    InvalidHeaderHash,
    InvalidTransactionData,
    InvalidValidatorSet,
    InvalidPublicKey,
    VotingPowerOverflow,
    WrongChain,
    WrongHeight,
    WrongRound,
    WrongBlock,
    WrongValidatorSet,
    InvalidSignatureLayout,
    InvalidSignature,
    InsufficientVotingPower,
}

/// Explicit local-development bound; not a production scaling acceptance claim.
pub const MAX_DEVELOPMENT_VALIDATORS: usize = 64;
pub(crate) const MAX_NATIVE_VOTING_POWER: i64 = i64::MAX / 8;
