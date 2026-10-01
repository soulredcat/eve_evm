// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::ValidatedTransaction;
use revm::context::TxEnv;

impl ValidatedTransaction {
    pub fn evm(&self) -> &TxEnv {
        &self.evm
    }
}
