// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use eve_consensus_comet::wire::tendermint::abci::RequestProcessProposal;

pub(crate) fn record_acceptance_rejection(
    _fixture: Option<&AcceptanceFixture>,
    _request: &RequestProcessProposal,
) -> std::io::Result<()> {
    Ok(())
}
