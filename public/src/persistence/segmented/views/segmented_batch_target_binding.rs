// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::SealedSegmentedBatch;

pub fn segmented_batch_target_binding(batch: &SealedSegmentedBatch) -> [u8; 32] {
    batch.0.plan.marker.target_state_binding
}
