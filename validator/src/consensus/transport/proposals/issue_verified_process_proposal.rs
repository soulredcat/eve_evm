// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{EngineProposalBinding, NativeProposalSource, VerifiedLocalEngineProposal};
use crate::consensus::transport::peer::{AuthenticatedEnginePeer, ensure_application_engine_peer};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::RequestProcessProposal;

/// The dispatcher pairs this decoded request with its actual authenticated input FD.
pub(in crate::consensus::transport) fn issue_verified_process_proposal(
    peer: &AuthenticatedEnginePeer,
    request: RequestProcessProposal,
    binding: EngineProposalBinding,
) -> Result<VerifiedLocalEngineProposal> {
    ensure_application_engine_peer(peer)?;
    ensure!(
        request.hash.len() == 32 && request.height > 0,
        "invalid native process proposal identity"
    );
    Ok(VerifiedLocalEngineProposal {
        request,
        binding,
        source: NativeProposalSource::ProcessProposal,
    })
}
