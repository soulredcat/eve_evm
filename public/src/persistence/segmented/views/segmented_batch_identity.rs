// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::SealedSegmentedBatch;

pub fn segmented_batch_identity(
    batch: &SealedSegmentedBatch,
) -> eve_storage::records::segmented::SegmentedLogicalIdentity {
    batch.0.plan.marker.identity
}
