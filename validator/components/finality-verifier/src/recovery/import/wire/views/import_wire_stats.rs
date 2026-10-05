// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{ImportWirePreflight, ImportWireStats};

pub fn import_wire_stats(preflight: &ImportWirePreflight<'_>) -> ImportWireStats {
    preflight.stats
}
