// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::transport::proposals::{NativeProposalSource, VerifiedLocalEngineProposal};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::RequestFinalizeBlock;

pub(super) fn validate_finalize_source(
    source: &VerifiedLocalEngineProposal,
    request: &RequestFinalizeBlock,
) -> Result<()> {
    let normalized = source.request();
    ensure!(
        source.source() == NativeProposalSource::FinalizeBlock
            && normalized.txs == request.txs
            && normalized.proposed_last_commit == request.decided_last_commit
            && normalized.misbehavior == request.misbehavior
            && normalized.hash == request.hash
            && normalized.height == request.height
            && normalized.time == request.time
            && normalized.next_validators_hash == request.next_validators_hash
            && normalized.proposer_address == request.proposer_address,
        "finalization input differs from actual authenticated callback"
    );
    Ok(())
}
