// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedCodecLimits, SegmentedRecordPreflight};

pub fn segmented_record_limits(preflight: &SegmentedRecordPreflight<'_>) -> SegmentedCodecLimits {
    preflight.limits
}
