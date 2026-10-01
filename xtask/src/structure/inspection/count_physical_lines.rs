// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub fn count_physical_lines(source: &str) -> usize {
    source
        .as_bytes()
        .iter()
        .filter(|byte| **byte == b'\n')
        .count()
        + usize::from(!source.is_empty() && !source.ends_with('\n'))
}
