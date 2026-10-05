// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::history::VerifiedNativeHeader;
use crate::wire::tendermint::types::Header;

impl VerifiedNativeHeader {
    pub fn header(&self) -> &Header {
        &self.header
    }
}
