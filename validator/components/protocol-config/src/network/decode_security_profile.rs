// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ProfileError, SecurityProfile};

pub fn decode_security_profile(tag: u8) -> Result<SecurityProfile, ProfileError> {
    match tag {
        1 => Ok(SecurityProfile::ClassicalDev),
        2 => Ok(SecurityProfile::HybridExperimental),
        3 => Ok(SecurityProfile::PqProfileVerified),
        _ => Err(ProfileError::InvalidBinding),
    }
}
