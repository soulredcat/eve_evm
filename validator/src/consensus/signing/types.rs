// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use ed25519_dalek::SigningKey;
use eve_consensus_comet::consensus::authentication::ConsensusAuthenticationRequirement;
use eve_storage::{
    records::{OpaqueRecordCursor, OpaqueRecordRepository},
    state::StateService,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Expected enrollment comes from runtime-authenticated genesis/history, never this struct alone.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::consensus) struct SignerConfig {
    pub genesis_hash: [u8; 32],
    pub chain_id: String,
    pub expected_public_key: [u8; 32],
    pub authentication: ConsensusAuthenticationRequirement,
    pub key_epoch: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Hrs {
    pub height: i64,
    pub round: i32,
    pub step: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SignedRecord {
    pub version: u8,
    pub genesis_hash: [u8; 32],
    pub chain_id: String,
    pub public_key: [u8; 32],
    pub profile: u8,
    pub key_epoch: u64,
    pub hrs: Hrs,
    pub sign_bytes: Vec<u8>,
    pub signature: Vec<u8>,
}

/// Secret key and raw repository are confined to this private runtime capability.
pub(in crate::consensus) struct DurableSigner {
    pub(in crate::consensus) config: SignerConfig,
    pub(in crate::consensus) service: Arc<StateService>,
    pub(super) key: SigningKey,
    pub(super) repository: OpaqueRecordRepository,
    pub(super) cursor: OpaqueRecordCursor,
    pub(super) last: Option<SignedRecord>,
    pub(super) fenced: bool,
    #[cfg(test)]
    pub(super) simulated_failure: Option<SimulatedSignerFailure>,
}

/// Safe operator metadata; it contains no signing bytes, signatures or secret material.
pub(in crate::consensus) struct SignerStatus {
    pub last_height: Option<i64>,
    pub last_round: Option<i32>,
    pub last_step: Option<u8>,
    pub cursor: OpaqueRecordCursor,
    pub fenced: bool,
}

#[cfg(test)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum SimulatedSignerFailure {
    BeforeWrite,
    AfterSync,
    ExitAfterSync,
}
