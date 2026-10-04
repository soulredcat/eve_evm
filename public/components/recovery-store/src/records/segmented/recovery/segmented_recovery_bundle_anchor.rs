// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::RecoveredSegmentedBundle;
use crate::records::segmented::SegmentedRecoveryAnchor;

pub fn segmented_recovery_bundle_anchor(
    bundle: &RecoveredSegmentedBundle,
) -> SegmentedRecoveryAnchor {
    bundle.completion.target
}
