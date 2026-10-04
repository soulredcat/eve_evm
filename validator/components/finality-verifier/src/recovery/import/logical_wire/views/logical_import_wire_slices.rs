// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::LogicalImportWirePreflight;
use crate::recovery::import::wire::ImportWireSlices;

pub fn logical_import_wire_slices<'a>(
    preflight: &LogicalImportWirePreflight<'a>,
) -> ImportWireSlices<'a> {
    preflight.slices
}
