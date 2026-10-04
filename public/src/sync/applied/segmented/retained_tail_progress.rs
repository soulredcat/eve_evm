// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{super::RetainedAppliedTail, types::SegmentedTailProgress};
use crate::persistence::segmented::segmented_batch_identity;
/// Worker tail-slot progress; not a durable logical-height assertion or current physical head.
pub fn retained_tail_progress(
    tail: &RetainedAppliedTail,
    slot: usize,
) -> Option<SegmentedTailProgress> {
    let tail = tail.segmented_tails.as_ref()?.get(slot)?.as_ref()?;
    Some(SegmentedTailProgress {
        logical_target: segmented_batch_identity(&tail.batch).target_height,
        last_acknowledged_physical: tail.last_acknowledged_physical_cursor,
        complete_marker: tail.complete_marker.map(|ack| ack.marker_cursor),
    })
}
