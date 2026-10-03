// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::LogicalImportWirePreflight;

pub fn logical_import_wire_bytes<'a>(preflight: &LogicalImportWirePreflight<'a>) -> &'a [u8] {
    preflight.bytes
}
