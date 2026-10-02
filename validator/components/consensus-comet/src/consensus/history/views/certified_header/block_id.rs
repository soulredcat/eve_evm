// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::history::VerifiedNativeHeader;
use crate::wire::tendermint::types::BlockId;

impl VerifiedNativeHeader {
    pub fn block_id(&self) -> &BlockId {
        &self.block_id
    }
}
