// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::{DevelopmentFinalityVerifier, FinalityError};
use eve_state::StateVersion;

pub(super) fn validate_application_identity(
    verifier: &DevelopmentFinalityVerifier,
    version: &StateVersion,
) -> Result<(), FinalityError> {
    if version.identity != verifier.identity {
        return Err(FinalityError::WrongApplicationIdentity);
    }
    if version.height == 0 {
        return Err(FinalityError::WrongApplicationHeight);
    }
    Ok(())
}
