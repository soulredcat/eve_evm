// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::VerifiedDevelopmentHeader;
use eve_consensus_comet::consensus::history::VerifiedNativeHeader;

impl VerifiedDevelopmentHeader {
    pub fn native(&self) -> &VerifiedNativeHeader {
        &self.native
    }
}
