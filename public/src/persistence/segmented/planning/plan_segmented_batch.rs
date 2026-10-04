// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedBatchPlan, SegmentedError, SegmentedPartPool};
use super::plan_segmented_layout::plan_segmented_layout;
use eve_storage::records::segmented::{SegmentedLogicalIdentity, SegmentedMarkerMetadata};

pub fn plan_segmented_batch(
    pool: &SegmentedPartPool,
    identity: SegmentedLogicalIdentity,
    target_state_binding: [u8; 32],
) -> Result<SegmentedBatchPlan, SegmentedError> {
    if target_state_binding == [0; 32] {
        return Err(SegmentedError::InvalidPlan);
    }
    Ok(SegmentedBatchPlan {
        layout: plan_segmented_layout(pool, identity)?,
        marker: SegmentedMarkerMetadata {
            identity,
            target_state_binding,
        },
    })
}
