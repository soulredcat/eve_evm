// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::history::NativeHistoryVerifier;
use crate::wire::tendermint::types::BlockId;

impl NativeHistoryVerifier {
    pub fn block_id(&self) -> Option<&BlockId> {
        self.block_id.as_ref()
    }
}
