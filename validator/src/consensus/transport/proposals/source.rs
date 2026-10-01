// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{NativeProposalSource, VerifiedLocalEngineProposal};

impl VerifiedLocalEngineProposal {
    pub(in crate::consensus) fn source(&self) -> NativeProposalSource {
        self.source
    }
}
