// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::AuthenticatedApplicationAnchor;

impl AuthenticatedApplicationAnchor {
    pub fn consensus_height(&self) -> i64 {
        self.consensus_height
    }
}
