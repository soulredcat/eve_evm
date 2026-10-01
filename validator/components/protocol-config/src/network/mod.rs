// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Expected profile matching; expectation provenance remains the caller's duty.

mod decode_security_profile;
mod types;
mod validate_network_profile;
mod validate_security_profile;

pub use decode_security_profile::decode_security_profile;
pub use types::{LaunchMode, NetworkProfileBinding, ProfileError, SecurityProfile};
pub use validate_network_profile::validate_network_profile;
pub use validate_security_profile::validate_security_profile;
