// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    ConsensusApplication, application_finalization_binding, application_proposal_binding,
};
use crate::consensus::transport::proposals::{
    VerifiedLocalEngineProposal, fixture_verified_finalize_proposal,
    fixture_verified_local_engine_proposal,
};
use eve_consensus_comet::wire::tendermint::abci::{RequestFinalizeBlock, RequestProcessProposal};

pub(super) fn process_request(
    application: &ConsensusApplication,
    height: i64,
    transactions: Vec<Vec<u8>>,
) -> RequestProcessProposal {
    let proposer = *application.config.proposer_owners.keys().next().unwrap();
    RequestProcessProposal {
        height,
        txs: transactions,
        hash: vec![u8::try_from(height).unwrap(); 32],
        time: Some(prost_types::Timestamp {
            seconds: application.config.initial_time.seconds + height,
            nanos: 123,
        }),
        proposer_address: proposer.to_vec(),
        next_validators_hash: vec![9; 32],
        ..Default::default()
    }
}

pub(super) fn process_source(
    application: &ConsensusApplication,
    request: RequestProcessProposal,
) -> VerifiedLocalEngineProposal {
    let binding = application_proposal_binding(application, &request.proposer_address).unwrap();
    fixture_verified_local_engine_proposal(request, binding)
}

pub(super) fn final_request(request: &RequestProcessProposal) -> RequestFinalizeBlock {
    RequestFinalizeBlock {
        txs: request.txs.clone(),
        decided_last_commit: request.proposed_last_commit.clone(),
        misbehavior: request.misbehavior.clone(),
        hash: request.hash.clone(),
        height: request.height,
        time: request.time,
        next_validators_hash: request.next_validators_hash.clone(),
        proposer_address: request.proposer_address.clone(),
    }
}

pub(super) fn final_source(
    application: &ConsensusApplication,
    request: &RequestFinalizeBlock,
) -> VerifiedLocalEngineProposal {
    let binding =
        application_finalization_binding(application, request.height, &request.proposer_address)
            .unwrap();
    let normalized = RequestProcessProposal {
        txs: request.txs.clone(),
        proposed_last_commit: request.decided_last_commit.clone(),
        misbehavior: request.misbehavior.clone(),
        hash: request.hash.clone(),
        height: request.height,
        time: request.time,
        next_validators_hash: request.next_validators_hash.clone(),
        proposer_address: request.proposer_address.clone(),
    };
    fixture_verified_finalize_proposal(normalized, binding)
}
