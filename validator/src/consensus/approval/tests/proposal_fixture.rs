// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    signing::SignerConfig,
    transport::proposals::{
        EngineProposalBinding, VerifiedLocalEngineProposal, fixture_verified_local_engine_proposal,
    },
};
use alloy_primitives::Address;
use eve_consensus_comet::{
    consensus::certificates::validator_address, wire::tendermint::abci::RequestProcessProposal,
};

/// Synthetic local-provenance unit fixture; only actual canonical execution is being asserted.
pub(super) fn proposal_fixture(
    config: &SignerConfig,
    transactions: Vec<Vec<u8>>,
) -> VerifiedLocalEngineProposal {
    let request = RequestProcessProposal {
        height: 1,
        hash: vec![7; 32],
        time: Some(prost_types::Timestamp {
            seconds: 1_728_000_001,
            nanos: 123,
        }),
        txs: transactions,
        proposer_address: validator_address(&config.expected_public_key).to_vec(),
        ..Default::default()
    };
    let binding = EngineProposalBinding {
        genesis_hash: config.genesis_hash,
        chain_id: config.chain_id.clone(),
        authentication: config.authentication,
        key_epoch: config.key_epoch,
        proposer_owner: Address::repeat_byte(1),
        previous_consensus_hash: [0; 32],
    };
    fixture_verified_local_engine_proposal(request, binding)
}
