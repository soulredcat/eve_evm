// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::ValidatedTransaction;
use alloy_primitives::Address;

impl ValidatedTransaction {
    pub fn sender(&self) -> Address {
        self.sender
    }
}
