// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::LogicalImportWirePreflight;
use crate::recovery::import::wire::ImportWireStats;

pub fn logical_import_wire_stats(preflight: &LogicalImportWirePreflight<'_>) -> ImportWireStats {
    preflight.stats
}
