// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{
    AppliedOwner, AppliedReader, SegmentedAppliedConfig,
    checkpoints::{CheckpointAppliedError, CheckpointRecoveryConfig},
    segmented::open_segmented_applied_state_service_with_recovery,
};
use eve_state::DevelopmentGenesis;

/// Checkpoint artifacts must be locally configured; no source-supplied paths or trust policy.
pub fn open_segmented_applied_state_service_with_checkpoints(
    config: SegmentedAppliedConfig,
    recovery: CheckpointRecoveryConfig,
    local_configured_genesis: &DevelopmentGenesis,
) -> Result<(AppliedOwner, AppliedReader), CheckpointAppliedError> {
    open_segmented_applied_state_service_with_recovery(
        config,
        Some(recovery),
        local_configured_genesis,
    )
}
