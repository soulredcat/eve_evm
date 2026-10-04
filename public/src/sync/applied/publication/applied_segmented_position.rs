// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedPublication, SegmentedAppliedPosition};
pub fn applied_segmented_position(
    publication: &AppliedPublication,
) -> Option<SegmentedAppliedPosition> {
    publication.segmented_position
}
