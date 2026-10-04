// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::*;
use std::mem::size_of;

/// Concrete descriptor sizes plus explicit logical channel/control allowance; not RSS.
pub(in crate::persistence::segmented) fn estimate_segmented_metadata() -> (usize, usize, usize) {
    let pool = size_of::<SegmentedPartPool>() + 2 * size_of::<usize>() + 4_096;
    let batch = size_of::<SegmentedBatchReservation>()
        + size_of::<SealedBatch>()
        + SEGMENTS * size_of::<eve_storage::records::OpaqueRecordCursor>()
        + 8_192;
    let worker = size_of::<WorkerState>()
        + BATCHES
            * (size_of::<Request>()
                + size_of::<SegmentedTicket>()
                + size_of::<SegmentedLogicalAck>()
                + 4_096)
        + 4_096;
    (pool, batch, worker)
}
