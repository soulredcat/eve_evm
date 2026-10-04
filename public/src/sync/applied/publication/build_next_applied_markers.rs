// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::build_applied_markers;
use crate::sync::applied::{AppliedError, AppliedPublication, state::AppliedState};
use eve_node_policy::{PublicWatermarks, validate_watermarks};

/// A new verified generation retains the actual parent's recovery checkpoint.
pub(in crate::sync::applied) fn build_next_applied_markers(
    state: &AppliedState,
    durable: u64,
    parent: &AppliedPublication,
) -> Result<PublicWatermarks, AppliedError> {
    let mut markers = build_applied_markers(state, durable)?;
    markers.checkpoint = parent.markers.checkpoint;
    markers.authenticated_snapshot_height = parent.markers.authenticated_snapshot_height;
    markers.oldest_retained_height = parent.markers.oldest_retained_height;
    validate_watermarks(markers).map_err(|_| AppliedError::InvalidDurablePrefix)?;
    Ok(markers)
}
