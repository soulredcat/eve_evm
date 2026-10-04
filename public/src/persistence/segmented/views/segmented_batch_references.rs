// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::SealedSegmentedBatch;

pub fn segmented_batch_references(
    batch: &SealedSegmentedBatch,
) -> &[eve_storage::records::OpaqueRecordCursor] {
    &batch.0.references[..batch.0.plan.layout.segment_count]
}
