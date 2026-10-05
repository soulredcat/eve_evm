// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::AuthenticatedApplicationAnchor;
use eve_state::ApplicationCommitment;

impl AuthenticatedApplicationAnchor {
    pub fn application(&self) -> ApplicationCommitment {
        self.application
    }
}
