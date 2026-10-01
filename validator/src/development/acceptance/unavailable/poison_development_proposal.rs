// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use eve_consensus_comet::wire::tendermint::abci::RequestPrepareProposal;
use std::sync::atomic::AtomicBool;

pub(crate) fn poison_development_proposal(
    _fixture: Option<&AcceptanceFixture>,
    _request: &RequestPrepareProposal,
    _used: &AtomicBool,
) -> Option<Vec<Vec<u8>>> {
    None
}
