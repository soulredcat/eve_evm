// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointAppliedError, PreparedAppliedCheckpoint};
use crate::sync::applied::{AppliedOwner, capture_applied_state, types::AppliedBackend};
use std::sync::Arc;

pub(super) fn validate_checkpoint_activation_parent(
    owner: &AppliedOwner,
    prepared: &PreparedAppliedCheckpoint,
) -> Result<(), CheckpointAppliedError> {
    if owner.storage_failed {
        return Err(CheckpointAppliedError::Applied(
            crate::sync::applied::AppliedError::StorageFailed,
        ));
    }
    if !matches!(owner.backend, AppliedBackend::Segmented { .. }) {
        return Err(CheckpointAppliedError::WrongMode);
    }
    if !owner.pending.is_empty() || owner.checkpoint.is_some() {
        return Err(CheckpointAppliedError::PendingDurability);
    }
    let current = capture_applied_state(&owner.reader).map_err(CheckpointAppliedError::Applied)?;
    let parent = &prepared.artifacts.content.parent;
    let position = current
        .segmented_position
        .ok_or(CheckpointAppliedError::WrongMode)?;
    if !Arc::ptr_eq(&current, parent)
        || !Arc::ptr_eq(&owner.reader.working, &prepared.artifacts.content.working)
        || current.storage_failed
        || current.admitted_cursor != owner.admitted_cursor
        || current.durable_cursor != owner.durable_cursor
        || position.applied != position.durable
        || position.durable.cursor != owner.durable_cursor
        || current.markers.applied.0 != current.markers.durable_recovery.0
    {
        return Err(CheckpointAppliedError::StaleParent);
    }
    Ok(())
}
