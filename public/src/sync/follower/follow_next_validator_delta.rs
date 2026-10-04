// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ValidatorFollowerSource;
use crate::sync::applied::{
    AppliedAdmission, AppliedMode, AppliedOwner, applied_commit, applied_mode,
    applied_owner_reader, applied_owner_state_budget, applied_segmented_position,
    capture_applied_state, reserve_applied_working, try_apply_recovery_bytes_matching_target,
};
use anyhow::{Result, ensure};
use eve_state::StateDeltaRequest;
use eve_sync_client::{
    downloaded_import_target, downloaded_import_wire, fetch_authenticated_import_wire,
};

/// Bind the exact admission owner, then delegate shared untrusted download assembly
/// and canonical applied validation. The source never selects application authority.
pub fn follow_next_validator_delta(
    owner: &mut AppliedOwner,
    source: ValidatorFollowerSource,
) -> Result<AppliedAdmission> {
    let reader = applied_owner_reader(owner);
    let state_budget = applied_owner_state_budget(owner);
    let publication = capture_applied_state(&reader)
        .map_err(|error| anyhow::anyhow!("FOLLOWER_CAPTURE: {error:?}"))?;
    ensure!(
        applied_mode(&publication) == AppliedMode::AuthenticatedImport
            && applied_segmented_position(&publication).is_some(),
        "FOLLOWER_WRONG_PERSISTENCE_PROFILE"
    );
    let parent = applied_commit(&publication).target.clone();
    let height = parent
        .height
        .checked_add(1)
        .ok_or_else(|| anyhow::anyhow!("FOLLOWER_HEIGHT"))?;
    let download = fetch_authenticated_import_wire(
        source.rpc,
        StateDeltaRequest {
            parent,
            target_height: height,
            offset: 0,
            maximum_chunk_bytes: source.maximum_chunk_bytes,
        },
        &state_budget,
        |bytes| {
            reserve_applied_working(&reader, bytes)
                .map_err(|error| anyhow::anyhow!("FOLLOWER_RESOURCE_LIMIT: {error:?}"))
        },
    )?;
    try_apply_recovery_bytes_matching_target(
        owner,
        downloaded_import_wire(&download),
        downloaded_import_target(&download),
    )
    .map_err(|error| anyhow::anyhow!("FOLLOWER_IMPORT: {error:?}"))
}
