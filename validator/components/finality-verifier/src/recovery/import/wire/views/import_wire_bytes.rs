// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::ImportWirePreflight;

pub fn import_wire_bytes<'a>(preflight: &ImportWirePreflight<'a>) -> &'a [u8] {
    preflight.bytes
}
