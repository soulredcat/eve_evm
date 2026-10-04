// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{RetainedAppliedTail, types::PendingPayload};
use crate::persistence::segmented::segmented_part_bytes;
pub fn retained_tail_part_bytes(
    tail: &RetainedAppliedTail,
    entry: usize,
    part: usize,
    segment: usize,
) -> Option<&[u8]> {
    match &tail.pending.get(entry)?.payload {
        PendingPayload::Segmented { payload, .. } => segmented_part_bytes(payload, part, segment),
        PendingPayload::Compact { .. } => None,
    }
}
