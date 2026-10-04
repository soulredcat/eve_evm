// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    SegmentedAppliedConfig, map_legacy_segmented_open_error::map_legacy_segmented_open_error,
    open_segmented_applied_state_service_with_recovery::open_segmented_applied_state_service_with_recovery,
};
use crate::sync::applied::{AppliedError, AppliedOwner, AppliedReader};
use eve_state::DevelopmentGenesis;

/// Preserve the legacy constructor and its refusal to accept a checkpoint through the old guard.
pub fn open_segmented_applied_state_service(
    config: SegmentedAppliedConfig,
    genesis: &DevelopmentGenesis,
) -> Result<(AppliedOwner, AppliedReader), AppliedError> {
    open_segmented_applied_state_service_with_recovery(config, None, genesis)
        .map_err(map_legacy_segmented_open_error)
}
