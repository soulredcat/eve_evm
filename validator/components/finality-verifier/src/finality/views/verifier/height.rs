// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::DevelopmentFinalityVerifier;

impl DevelopmentFinalityVerifier {
    pub fn height(&self) -> i64 {
        self.native.height()
    }
}
