// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{ImportWirePreflight, ImportWireSlices};

pub fn import_wire_slices<'a>(preflight: &ImportWirePreflight<'a>) -> ImportWireSlices<'a> {
    preflight.slices
}
