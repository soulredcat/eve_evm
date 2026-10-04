// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{SegmentedRecoveryObservation, SegmentedRecoveryScan};

pub fn segmented_recovery_observation(
    scan: &SegmentedRecoveryScan,
) -> SegmentedRecoveryObservation {
    SegmentedRecoveryObservation {
        logical_anchor: scan.anchor,
        physical_cursor: scan.physical,
        received_segments: scan.candidate.map_or(0, |value| value.received),
        expected_segments: scan.candidate.map_or(0, |value| value.expected),
        orphan_segments: scan.orphan_segments,
        awaiting_confirmation: scan.pending.is_some(),
        failed: scan.failed,
    }
}
