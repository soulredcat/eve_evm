// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedPublication;
use eve_node_policy::PublicWatermarks;

pub fn applied_markers(publication: &AppliedPublication) -> PublicWatermarks {
    publication.markers
}
