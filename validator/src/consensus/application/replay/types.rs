// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, B256};
use eve_consensus_comet::wire::tendermint::abci::RequestFinalizeBlock;
use eve_state::StateVersion;
use eve_storage::records::OpaqueRecordCursor;

/// Exact locally decided replay material, not a portable finality certificate.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::consensus::application) struct ReplayDecision {
    pub parent: StateVersion,
    pub target: StateVersion,
    pub commit_identity: B256,
    pub previous_consensus_hash: [u8; 32],
    pub proposer_owner: Address,
    pub request: RequestFinalizeBlock,
}

pub(in crate::consensus::application) enum ReplayRecord {
    Decided(Box<ReplayDecision>),
    Synced {
        target: Box<StateVersion>,
        commit_identity: B256,
        decision_cursor: OpaqueRecordCursor,
    },
}

/// Domain-specific local metadata over the existing synced opaque repository.
#[derive(Clone, PartialEq, prost::Message)]
pub(in crate::consensus::application) struct ReplayEnvelope {
    #[prost(uint32, tag = "1")]
    pub schema: u32,
    #[prost(uint32, tag = "2")]
    pub phase: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub genesis: Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub parent: Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub target: Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub commit_identity: Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub previous_consensus_hash: Vec<u8>,
    #[prost(message, optional, tag = "8")]
    pub request: Option<RequestFinalizeBlock>,
    #[prost(bytes = "vec", tag = "9")]
    pub proposer_owner: Vec<u8>,
    #[prost(uint64, tag = "10")]
    pub decision_sequence: u64,
    #[prost(bytes = "vec", tag = "11")]
    pub decision_hash: Vec<u8>,
}

pub(in crate::consensus::application) const MAX_REPLAY_BYTES: usize = 4 * 1_048_576 + 4000;
