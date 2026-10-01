// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::ValidatedTransaction;
use alloy_primitives::B256;

impl ValidatedTransaction {
    pub fn hash(&self) -> B256 {
        self.hash
    }
}
