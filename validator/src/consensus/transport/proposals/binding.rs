// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{EngineProposalBinding, VerifiedLocalEngineProposal};

impl VerifiedLocalEngineProposal {
    pub(in crate::consensus) fn binding(&self) -> &EngineProposalBinding {
        &self.binding
    }
}
