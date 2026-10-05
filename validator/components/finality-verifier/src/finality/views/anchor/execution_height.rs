// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::AuthenticatedApplicationAnchor;

impl AuthenticatedApplicationAnchor {
    pub fn execution_height(&self) -> u64 {
        self.execution_height
    }
}
