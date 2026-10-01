// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ExecutionApproval;
use eve_consensus_comet::wire::tendermint::abci::RequestProcessProposal;

pub(in crate::consensus) fn approval_request(
    approval: &ExecutionApproval,
) -> &RequestProcessProposal {
    &approval.request
}
