// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ConsensusAuthenticationRequirement, UnsupportedHybridConsensus};

/// Refuse activated hybrid consensus until an engine extension is integrated.
///
/// Allowing a native ML-DSA key type does not enforce classical AND ML-DSA
/// verification for the same enrolled identity and canonical engine message.
pub fn require_supported_authentication(
    requirement: ConsensusAuthenticationRequirement,
) -> Result<(), UnsupportedHybridConsensus> {
    match requirement {
        ConsensusAuthenticationRequirement::ClassicalDev => Ok(()),
        ConsensusAuthenticationRequirement::ClassicalAndMldsa65 => Err(UnsupportedHybridConsensus),
    }
}
