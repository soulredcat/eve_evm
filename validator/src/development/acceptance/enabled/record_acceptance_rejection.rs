// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use eve_consensus_comet::wire::tendermint::abci::RequestProcessProposal;
use std::{io::Write, sync::atomic::Ordering};

/// One public diagnostic proves the selected fault reached actual execution rejection.
pub(crate) fn record_acceptance_rejection(
    fixture: Option<&AcceptanceFixture>,
    request: &RequestProcessProposal,
) -> std::io::Result<()> {
    let Some(fixture) = fixture else {
        return Ok(());
    };
    let Some((height, proposer)) = fixture.poison else {
        return Ok(());
    };
    if request.height != height
        || request.proposer_address.as_slice() != proposer
        || request.txs.len() != 1
        || request.txs[0].as_slice() != [0xff]
        || fixture.rejection_recorded.swap(true, Ordering::AcqRel)
    {
        return Ok(());
    }
    let proposer = hex::encode(proposer);
    let hash = hex::encode(&request.hash);
    writeln!(
        std::io::stderr().lock(),
        "EVE_B3_REJECTED_PROPOSAL height={height} proposer={proposer} hash={hash}"
    )
}
