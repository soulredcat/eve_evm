// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{EngineProposalBinding, NativeProposalSource, VerifiedLocalEngineProposal};
use eve_consensus_comet::wire::tendermint::abci::RequestProcessProposal;

/// Controlled unit-test fixture only; production transport must authenticate its engine.
pub(in crate::consensus) fn fixture_verified_local_engine_proposal(
    request: RequestProcessProposal,
    binding: EngineProposalBinding,
) -> VerifiedLocalEngineProposal {
    VerifiedLocalEngineProposal {
        request,
        binding,
        source: NativeProposalSource::ProcessProposal,
    }
}
