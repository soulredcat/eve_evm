// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::open_applied_state_service_with_mode::open_applied_state_service_with_mode;
use crate::sync::applied::{AppliedConfig, AppliedError, AppliedMode, AppliedOwner, AppliedReader};
use eve_state::DevelopmentGenesis;

/// Backward-compatible constructor: explicit independent empty replay and legacy namespace.
pub fn open_applied_state_service(
    config: AppliedConfig,
    genesis: &DevelopmentGenesis,
) -> Result<(AppliedOwner, AppliedReader), AppliedError> {
    open_applied_state_service_with_mode(config, genesis, AppliedMode::EmptyReplay)
}
