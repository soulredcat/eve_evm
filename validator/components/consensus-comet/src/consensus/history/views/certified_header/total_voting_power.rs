// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::history::VerifiedNativeHeader;

impl VerifiedNativeHeader {
    pub fn total_voting_power(&self) -> i64 {
        self.total_voting_power
    }
}
