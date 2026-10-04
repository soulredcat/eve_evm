// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::RecoveredSegmentedBundle;
use crate::records::OpaqueRecordCursor;

pub fn segmented_recovery_bundle_references(
    bundle: &RecoveredSegmentedBundle,
) -> &[OpaqueRecordCursor] {
    &bundle.completion.references[..bundle.completion.count]
}
