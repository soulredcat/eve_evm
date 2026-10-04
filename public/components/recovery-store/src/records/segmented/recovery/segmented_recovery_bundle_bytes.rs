// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::RecoveredSegmentedBundle;

pub fn segmented_recovery_bundle_bytes(bundle: &RecoveredSegmentedBundle) -> &[u8] {
    &bundle.body
}
