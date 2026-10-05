// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::authentication::ConsensusAuthenticationRequirement;
use crate::consensus::history::NativeHistoryVerifier;

impl NativeHistoryVerifier {
    pub fn authentication(&self) -> ConsensusAuthenticationRequirement {
        self.authentication
    }
}
