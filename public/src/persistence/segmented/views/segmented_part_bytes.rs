// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::SealedSegmentedBatch;
pub fn segmented_part_bytes(
    batch: &SealedSegmentedBatch,
    part: usize,
    segment_in_part: usize,
) -> Option<&[u8]> {
    batch
        .0
        .parts
        .get(part)?
        .as_ref()?
        .buffers
        .get(segment_in_part)
        .filter(|bytes| !bytes.is_empty())
        .map(Vec::as_slice)
}
