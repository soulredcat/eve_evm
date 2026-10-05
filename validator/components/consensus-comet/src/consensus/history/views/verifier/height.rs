// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::history::NativeHistoryVerifier;

impl NativeHistoryVerifier {
    pub fn height(&self) -> i64 {
        self.height
    }
}
