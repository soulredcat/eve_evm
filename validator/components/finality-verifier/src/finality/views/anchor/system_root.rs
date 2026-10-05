// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::AuthenticatedApplicationAnchor;
use eve_state::SystemStateRoot;

impl AuthenticatedApplicationAnchor {
    pub fn system_root(&self) -> SystemStateRoot {
        self.system_root
    }
}
