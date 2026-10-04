// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedBatchPlan, SegmentedBatchReservation, SegmentedError, SegmentedPartPool,
};
use super::{bind_segmented_reservation, reserve_segmented_layout};
use std::sync::Arc;

pub fn reserve_segmented_batch(
    pool: &Arc<SegmentedPartPool>,
    plan: &SegmentedBatchPlan,
) -> Result<SegmentedBatchReservation, SegmentedError> {
    let reservation = reserve_segmented_layout(pool, &plan.layout)?;
    bind_segmented_reservation(reservation, plan.marker.target_state_binding)
        .map_err(|rejected| rejected.error)
}
