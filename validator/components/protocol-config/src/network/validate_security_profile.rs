// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::consensus::authentication::{
    ConsensusAuthenticationRequirement, require_supported_authentication,
};

use super::{ProfileError, SecurityProfile};

pub fn validate_security_profile(profile: SecurityProfile) -> Result<(), ProfileError> {
    let requirement = match profile {
        SecurityProfile::ClassicalDev => ConsensusAuthenticationRequirement::ClassicalDev,
        SecurityProfile::HybridExperimental | SecurityProfile::PqProfileVerified => {
            ConsensusAuthenticationRequirement::ClassicalAndMldsa65
        }
    };
    require_supported_authentication(requirement).map_err(|_| ProfileError::UnsupportedHybrid)
}
