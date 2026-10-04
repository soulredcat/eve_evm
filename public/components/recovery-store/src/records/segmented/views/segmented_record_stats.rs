// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedRecordPreflight, SegmentedRecordStats};

pub fn segmented_record_stats(preflight: &SegmentedRecordPreflight<'_>) -> SegmentedRecordStats {
    preflight.stats
}
