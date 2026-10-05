// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::DevelopmentFinalityVerifier;
use eve_state::StateIdentity;

impl DevelopmentFinalityVerifier {
    pub fn identity(&self) -> &StateIdentity {
        &self.identity
    }
}
