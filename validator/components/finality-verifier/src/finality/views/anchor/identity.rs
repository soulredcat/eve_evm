// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::AuthenticatedApplicationAnchor;
use eve_state::StateIdentity;

impl AuthenticatedApplicationAnchor {
    pub fn identity(&self) -> &StateIdentity {
        &self.identity
    }
}
