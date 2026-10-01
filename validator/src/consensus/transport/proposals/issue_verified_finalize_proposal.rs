// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{EngineProposalBinding, NativeProposalSource, VerifiedLocalEngineProposal};
use crate::consensus::transport::peer::{AuthenticatedEnginePeer, ensure_application_engine_peer};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::{RequestFinalizeBlock, RequestProcessProposal};

/// Bind actual decided bytes; this token cannot be treated as unlocked-round evidence.
pub(in crate::consensus::transport) fn issue_verified_finalize_proposal(
    peer: &AuthenticatedEnginePeer,
    finalized: &RequestFinalizeBlock,
    binding: EngineProposalBinding,
) -> Result<VerifiedLocalEngineProposal> {
    ensure_application_engine_peer(peer)?;
    ensure!(
        finalized.hash.len() == 32 && finalized.height > 0,
        "invalid native finalized identity"
    );
    let request = RequestProcessProposal {
        txs: finalized.txs.clone(),
        proposed_last_commit: finalized.decided_last_commit.clone(),
        misbehavior: finalized.misbehavior.clone(),
        hash: finalized.hash.clone(),
        height: finalized.height,
        time: finalized.time,
        next_validators_hash: finalized.next_validators_hash.clone(),
        proposer_address: finalized.proposer_address.clone(),
    };
    Ok(VerifiedLocalEngineProposal {
        request,
        binding,
        source: NativeProposalSource::FinalizeBlock,
    })
}
