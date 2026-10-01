// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::ValidatedTransaction;

impl ValidatedTransaction {
    pub fn transaction_type(&self) -> u8 {
        self.transaction_type
    }
}
