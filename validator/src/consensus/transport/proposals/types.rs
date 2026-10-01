// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::Address;
use eve_consensus_comet::{
    consensus::authentication::ConsensusAuthenticationRequirement,
    wire::tendermint::abci::RequestProcessProposal,
};

/// Filled from canonical configuration and retained consensus replay metadata.
pub(in crate::consensus) struct EngineProposalBinding {
    pub(in crate::consensus) genesis_hash: [u8; 32],
    pub(in crate::consensus) chain_id: String,
    pub(in crate::consensus) authentication: ConsensusAuthenticationRequirement,
    pub(in crate::consensus) key_epoch: u64,
    pub(in crate::consensus) proposer_owner: Address,
    pub(in crate::consensus) previous_consensus_hash: [u8; 32],
}

/// Only the private transport issuer can construct production proposal provenance.
/// Native ValidateBlock/hash-data binding is trusted for the pinned local engine;
/// this capability does not claim an independently reconstructed full header.
pub(in crate::consensus) struct VerifiedLocalEngineProposal {
    pub(in crate::consensus::transport) request: RequestProcessProposal,
    pub(in crate::consensus::transport) binding: EngineProposalBinding,
    pub(in crate::consensus::transport) source: NativeProposalSource,
}

/// Finalize carries a decision, not the native ProcessProposal unlocked signal.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(in crate::consensus) enum NativeProposalSource {
    ProcessProposal,
    FinalizeBlock,
}
