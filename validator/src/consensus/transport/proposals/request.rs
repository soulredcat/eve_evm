// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::VerifiedLocalEngineProposal;
use eve_consensus_comet::wire::tendermint::abci::RequestProcessProposal;

impl VerifiedLocalEngineProposal {
    pub(in crate::consensus) fn request(&self) -> &RequestProcessProposal {
        &self.request
    }
}
