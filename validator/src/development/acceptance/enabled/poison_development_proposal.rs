// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use eve_consensus_comet::wire::tendermint::abci::RequestPrepareProposal;
use std::sync::atomic::{AtomicBool, Ordering};

/// Inject one malformed proposal on a disposable selected proposer; vote validation stays mandatory.
pub(crate) fn poison_development_proposal(
    fixture: Option<&AcceptanceFixture>,
    request: &RequestPrepareProposal,
    used: &AtomicBool,
) -> Option<Vec<Vec<u8>>> {
    let (height, proposer) = fixture?.poison?;
    if request.height != height
        || request.proposer_address.as_slice() != proposer
        || used.swap(true, Ordering::AcqRel)
    {
        return None;
    }
    let proposer = hex::encode(proposer);
    eprintln!("EVE_B3_POISONED_PROPOSAL height={height} proposer={proposer} transaction=ff");
    Some(vec![vec![0xff]])
}
