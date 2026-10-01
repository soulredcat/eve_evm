// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Fail-closed capability check for the unextended pinned consensus engine.

mod require_supported_authentication;
mod types;

pub use require_supported_authentication::require_supported_authentication;
pub use types::{ConsensusAuthenticationRequirement, UnsupportedHybridConsensus};
